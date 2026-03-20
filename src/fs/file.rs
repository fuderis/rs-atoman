use super::*;
use crate::prelude::*;

use bytes::Bytes;
use tokio::fs::{self, File as TokioFile, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom as TokioSeekFrom};

/// The file open mode
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum OpenMode {
    Read,
    Write,
    ReadWrite,
}

/// The file cursor seek options
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

impl Into<TokioSeekFrom> for SeekFrom {
    fn into(self) -> TokioSeekFrom {
        match self {
            Self::Start(pos) => TokioSeekFrom::Start(pos),
            Self::End(pos) => TokioSeekFrom::End(pos),
            Self::Current(pos) => TokioSeekFrom::Current(pos),
        }
    }
}

impl From<TokioSeekFrom> for SeekFrom {
    fn from(value: TokioSeekFrom) -> Self {
        match value {
            TokioSeekFrom::Start(pos) => Self::Start(pos),
            TokioSeekFrom::End(pos) => Self::End(pos),
            TokioSeekFrom::Current(pos) => Self::Current(pos),
        }
    }
}

/// The file reader/writer
#[derive(Debug)]
pub struct File {
    path: PathBuf,
    file: TokioFile,
    cursor: u64,
    cursor_end: Option<u64>,
}

impl File {
    /// Open file with open options
    pub async fn open(path: impl AsRef<Path>, mode: OpenMode) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        // gen open options:
        let mut options = OpenOptions::new();
        match mode {
            OpenMode::Read => {
                options.read(true);
            }
            OpenMode::Write => {
                options.create(true).write(true);
            }
            OpenMode::ReadWrite => {
                options.create(true).read(true).write(true);
            }
        }

        // create parent dir:
        if mode != OpenMode::Read {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await?;
            }
        }

        Ok(Self {
            file: options.open(&path).await?,
            path,
            cursor: 0,
            cursor_end: None,
        })
    }

    /// Open file for read
    pub async fn open_read(path: impl AsRef<Path>) -> Result<Self> {
        Self::open(path, OpenMode::Read).await
    }

    /// Open file for write
    pub async fn open_write(path: impl AsRef<Path>) -> Result<Self> {
        Self::open(path, OpenMode::Write).await
    }

    /// Open file for read & write
    pub async fn open_read_write(path: impl AsRef<Path>) -> Result<Self> {
        Self::open(path, OpenMode::ReadWrite).await
    }

    /// Reads a file metadata
    pub async fn metadata(path: impl AsRef<Path>) -> Result<Metadata> {
        let path = path.as_ref().to_path_buf();
        let meta = fs::metadata(&path).await?;

        Ok(Metadata {
            extension: path.extension().and_then(|s| s.to_str()).map(String::from),
            path,
            file_type: if meta.is_file() {
                FileKind::File
            } else if meta.is_dir() {
                FileKind::Dir
            } else {
                FileKind::Symlink
            },
            created: meta.created()?.into(),
            modified: meta.modified()?.into(),
            size: meta.len(),
        })
    }

    /// Reads a file metadata
    pub async fn get_metadata(&self) -> Result<Metadata> {
        Self::metadata(&self.path).await
    }

    /// Sets the cursor position
    pub async fn seek(&mut self, seek: impl Into<TokioSeekFrom>) -> Result<u64> {
        self.cursor = self.file.seek(seek.into()).await?;
        Ok(self.cursor)
    }

    /// Sets the cursor position from the file start
    pub async fn seek_start(&mut self, pos: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(pos)).await
    }

    /// Sets the cursor position from the file end
    pub async fn seek_end(&mut self, pos: i64) -> Result<u64> {
        self.seek(SeekFrom::End(pos)).await
    }

    /// Sets the cursor position from the current position
    pub async fn seek_current(&mut self, pos: i64) -> Result<u64> {
        self.seek(SeekFrom::Current(pos)).await
    }

    /// Syns the cursor position
    pub async fn sync_cursor(&mut self) -> Result<()> {
        self.cursor = self.file.stream_position().await?;
        Ok(())
    }

    /// Returns the current position
    pub fn get_cursor(&mut self) -> u64 {
        self.cursor
    }

    /// Limits the cursor position range
    pub async fn limit(&mut self, seek: SeekFrom) -> Result<u64> {
        let start = self.get_cursor();
        let end = self.seek(seek).await?;

        self.seek_start(start).await?;
        self.cursor_end = Some(end);

        Ok(end)
    }

    /// Limits the cursor position range from the file start
    pub async fn limit_start(&mut self, pos: u64) -> Result<u64> {
        self.limit(SeekFrom::Start(pos)).await
    }

    /// Limits the cursor position range from the file end
    pub async fn limit_end(&mut self, pos: i64) -> Result<u64> {
        self.limit(SeekFrom::End(pos)).await
    }

    /// Limits the cursor position range from the current position
    pub async fn limit_current(&mut self, pos: i64) -> Result<u64> {
        self.limit(SeekFrom::Current(pos)).await
    }

    /// Unlimits the cursor position range
    pub async fn unlimit(&mut self) {
        self.cursor_end = None;
    }

    /// Reads some bytes of contents
    pub async fn read(&mut self) -> Result<Option<Bytes>> {
        let mut buffer = vec![0u8; 128];
        match self.file.read(&mut buffer).await {
            Ok(0) => Ok(None),
            Ok(n) => {
                self.cursor += n as u64;
                buffer.truncate(n);
                Ok(Some(Bytes::from(buffer)))
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Reads N-bytes of contents (returns Error if EOF before filling the buffer)
    pub async fn read_exact(&mut self, len: usize) -> Result<Option<Bytes>> {
        if len == 0 {
            return Ok(Some(Bytes::new()));
        }

        let mut buffer = vec![0u8; len];
        match self.file.read_exact(&mut buffer).await {
            Ok(_) => {
                self.cursor += len as u64;
                Ok(Some(Bytes::from(buffer)))
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Reads the file until it encounters stop bytes
    /// * capt_eof=true: returns the remaining data before EOF (even if this doesn't fit the stop-bytes pattern)
    pub async fn read_until(&mut self, stop_bytes: &[u8], capt_eof: bool) -> Result<Option<Bytes>> {
        let mut buffer = Vec::<u8>::with_capacity(128);
        let mut chunk = [0u8; 64];
        let stop_len = stop_bytes.len();

        loop {
            // checking limits (cursor_end):
            if let Some(end) = self.cursor_end {
                if self.cursor >= end {
                    return Ok(None);
                }
            }

            // we calculate how many bytes we can read at most:
            let max_to_read = if let Some(end) = self.cursor_end {
                (end - self.cursor).min(chunk.len() as u64) as usize
            } else {
                chunk.len()
            };

            // reading chunk:
            let n = match self.file.read(&mut chunk[..max_to_read]).await {
                Ok(0) => {
                    if capt_eof && !buffer.is_empty() {
                        // we haven't found stop_bytes, but we've reached the end
                        // we do not return the position back, as this is the end of the file
                        return Ok(Some(Bytes::from(buffer)));
                    }
                    if !buffer.is_empty() {
                        self.seek_current(-(buffer.len() as i64)).await?;
                    }
                    return Ok(None);
                }
                Ok(n) => n,
                Err(e) => return Err(e.into()),
            };

            buffer.extend_from_slice(&chunk[..n]);
            self.cursor += n as u64;

            // search (taking into account the overlap between chunks)
            // we start the search not from 0, but from the position where stop_bytes could have started
            let search_start = buffer.len().saturating_sub(n + stop_len - 1);

            if let Some(rel_pos) = buffer[search_start..]
                .windows(stop_len)
                .position(|w| w == stop_bytes)
            {
                let abs_pos = search_start + rel_pos;

                let match_end = abs_pos + stop_len;
                let over_read = buffer.len() - match_end;

                if over_read > 0 {
                    self.seek_current(-(over_read as i64)).await?;
                }

                buffer.truncate(abs_pos); // return data without stop_bytes
                return Ok(Some(Bytes::from(buffer)));
            }
        }
    }

    /// Reads the next line
    pub async fn read_line(&mut self) -> Result<Option<Bytes>> {
        self.read_until(b"\n", true).await
    }

    /// Reads the all next lines
    pub async fn read_lines(&mut self) -> Result<Vec<Bytes>> {
        let mut lines = Vec::new();
        while let Some(line) = self.read_line().await? {
            lines.push(line);
        }
        Ok(lines)
    }

    /// Reads the remaining file contents
    pub async fn read_to_end(&mut self) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        self.file.read_to_end(&mut data).await?;
        self.cursor = self.file.stream_position().await?;
        Ok(data)
    }

    /// Reads the remaining file contents to string
    pub async fn read_to_string(&mut self) -> Result<String> {
        let bytes = self.read_to_end().await?;
        let s = String::from_utf8_lossy(&bytes).to_string();
        Ok(s)
    }

    /// Writes a bytes after the cursor (returns written bytes count)
    pub async fn write(&mut self, data: &[u8]) -> Result<usize> {
        let written = self.file.write(data).await?;
        self.cursor += written as u64;
        Ok(written)
    }

    /// Guaranteed writes the all bytes after the cursor
    pub async fn write_all(&mut self, data: &[u8]) -> Result<()> {
        self.file.write_all(data).await?;
        self.cursor += data.len() as u64;
        Ok(())
    }

    /// Re-writes the full file contents from start
    pub async fn rewrite_all(&mut self, data: &[u8]) -> Result<()> {
        self.seek(SeekFrom::Start(0)).await?;
        self.file.set_len(0).await?;
        self.write_all(data).await
    }

    /// Cuts the file contents by new size
    pub async fn cut(&mut self, new_size: u64) -> Result<()> {
        self.file.set_len(new_size).await?;
        if self.cursor > new_size {
            self.cursor = new_size;
        }
        Ok(())
    }

    /// Force write changes to file
    pub async fn flush(&mut self) -> Result<()> {
        self.file.flush().await?;
        Ok(())
    }

    /// Force write changes to file with metadata & cursor sync
    pub async fn sync_all(&mut self) -> Result<()> {
        self.file.sync_all().await?;
        self.sync_cursor().await?;
        Ok(())
    }

    /// Removes the file
    pub async fn remove(path: impl AsRef<Path>) -> Result<()> {
        Ok(fs::remove_file(path).await?)
    }

    /// Removes the current file
    pub async fn remove_self(self) -> Result<()> {
        let path = self.path;
        drop(self.file);
        Self::remove(path).await
    }
}
