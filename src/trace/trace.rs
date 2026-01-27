use crate::{flag::Flag, prelude::*, state::State};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};
use tokio::{
    fs::{self as tfs, File},
    io::{self as tio, AsyncBufReadExt, AsyncReadExt, AsyncSeekExt},
    sync::Mutex,
    task::JoinHandle,
    time::{Duration, sleep},
};

/// High-performance async log tracer
pub struct Trace {
    path: PathBuf,
    file: Arc<Mutex<File>>,
    stack: Arc<State<VecDeque<String>>>,
    available: Arc<Flag>,
    _reader_handle: JoinHandle<()>,
}

impl Trace {
    /// Returns file path
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Opens file and starts background metadata polling task
    pub async fn open<P: AsRef<Path>>(file_path: P, timeout: Duration) -> Result<Self> {
        let path = file_path.as_ref().to_path_buf();
        let file = {
            let f = File::open(&file_path).await.map_err(Error::OpenFile)?;
            Arc::new(Mutex::new(f))
        };
        let stack = Arc::new(State::from(VecDeque::with_capacity(5)));
        let available = Arc::new(Flag::from(false));

        // clone data for spawn:
        let path_clone = path.clone();
        let file_clone = file.clone();
        let stack_clone = stack.clone();
        let available_clone = available.clone();

        // spawn background file monitoring task:
        let reader_handle = tokio::spawn(async move {
            let mut last_mod = tfs::metadata(&path_clone)
                .await
                .ok()
                .and_then(|m| m.modified().ok())
                .unwrap_or(std::time::UNIX_EPOCH);

            loop {
                if let Ok(Ok(mod_time)) = tokio::fs::metadata(&path_clone)
                    .await
                    .map(|meta| meta.modified())
                {
                    // check file last update:
                    if mod_time > last_mod {
                        let mut file = file_clone.lock().await;
                        if let Ok(new_lines) = Self::read_new_lines(&mut file).await
                            && !new_lines.is_empty()
                        {
                            stack_clone.lock().await.extend(new_lines);
                            if available_clone.is_false() {
                                available_clone.set(true);
                            }
                        }
                        last_mod = mod_time;
                    }
                }

                sleep(timeout).await;
            }
        });

        Ok(Self {
            path,
            file,
            stack,
            available,
            _reader_handle: reader_handle,
        })
    }

    /// Reads next line from stack (waits until available)
    pub async fn next_line(&self) -> Option<String> {
        loop {
            // wait until new lines are available:
            while self.available.is_false() {
                self.available.wait(true).await;
            }

            // get line from stack:
            if let Some(line) = self.stack.lock().await.pop_front() {
                return Some(line);
            }

            // stack empty - reset flag:
            self.available.set(false);
        }
    }

    /// Reads entire file content as Vec<String>
    pub async fn read_all(&self) -> Result<Vec<String>> {
        let mut f = self.file.lock().await;
        let mut buf = Vec::<u8>::with_capacity(128);

        f.rewind().await.map_err(Error::ReadFile)?;
        f.read_to_end(&mut buf).await.map_err(Error::ReadFile)?;

        let content = String::from_utf8_lossy(&buf);
        Ok(content.lines().map(|s| s.to_string()).collect())
    }

    /// Reads new lines from current file position to end
    async fn read_new_lines(file: &mut File) -> Result<VecDeque<String>> {
        let current_pos = file.stream_position().await.map_err(Error::ReadFile)?;

        // create buffered reader from current position:
        let mut reader = tio::BufReader::new(file);
        reader
            .seek(tio::SeekFrom::Start(current_pos))
            .await
            .map_err(Error::ReadFile)?;

        let mut new_lines = VecDeque::new();
        let mut line = String::new();

        // read all new lines until EOF:
        while reader.read_line(&mut line).await.map_err(Error::ReadFile)? > 0 {
            if line.ends_with('\n') {
                line.pop();
            }
            if !line.is_empty() {
                new_lines.push_back(line.clone());
            }
            line.clear();
        }

        Ok(new_lines)
    }
}
