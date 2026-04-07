use super::{FileKind, Metadata};
use crate::prelude::*;

use tokio::fs;

/// The dir entry structure
#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub kind: FileKind,
}

impl Entry {
    /// Creates a new entry from file path
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        let kind = if path.is_dir() {
            FileKind::Dir
        } else if path.is_file() {
            FileKind::File
        } else {
            FileKind::Symlink
        };

        Entry { path, kind }
    }

    /// Reads the entry metadata
    pub async fn metadata(&self) -> Result<Metadata> {
        Metadata::new(&self.path, fs::metadata(&self.path).await?)
    }

    /// Returns the entry file name
    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Returns the entry file extension
    pub fn extension() -> Option<String> {
        unimplemented!()
    }

    /// Returns true if entry is exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}
