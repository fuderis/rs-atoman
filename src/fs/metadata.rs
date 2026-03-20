use crate::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The file type
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum FileKind {
    File,
    Dir,
    Symlink,
}

/// The file metadata structure
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    pub file_type: FileKind,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub size: u64,
}
