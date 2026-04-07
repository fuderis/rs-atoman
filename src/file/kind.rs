// use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// The file type
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum FileKind {
    File,
    Dir,
    Symlink,
}

impl FileKind {
    /// Returns true if this is a file
    pub fn is_file(&self) -> bool {
        self == &Self::File
    }

    /// Returns true if this is a dir
    pub fn is_dir(&self) -> bool {
        self == &Self::Dir
    }

    /// Returns true if this is a symlink
    pub fn is_symlink(&self) -> bool {
        self == &Self::Symlink
    }
}
