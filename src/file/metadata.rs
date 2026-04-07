use super::FileKind;
use crate::prelude::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The file metadata structure
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    pub file_type: FileKind,
    pub executable: Option<bool>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub size: u64,
}

impl Metadata {
    /// Creates a new metadata structure
    pub fn new(path: impl AsRef<Path>, meta: std::fs::Metadata) -> Result<Self> {
        let path = path.as_ref();

        Ok(Self {
            extension: path.extension().and_then(|s| s.to_str()).map(String::from),
            file_type: if meta.is_file() {
                FileKind::File
            } else if meta.is_dir() {
                FileKind::Dir
            } else {
                FileKind::Symlink
            },
            executable: {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    Some(meta.permissions().mode() & 0o111 != 0)
                }
                #[cfg(not(unix))]
                {
                    None
                }
            },
            created: meta.created()?.into(),
            modified: meta.modified()?.into(),
            size: meta.len(),
        })
    }
}
