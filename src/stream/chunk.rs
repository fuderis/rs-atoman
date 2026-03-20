// use crate::prelude::*;
use bytes::Bytes;

/// The stream chunk wrapper
#[derive(Debug, Clone)]
pub struct Chunk(pub Bytes);

impl From<String> for Chunk {
    fn from(v: String) -> Self {
        Self(Bytes::from(v))
    }
}
impl From<&str> for Chunk {
    fn from(v: &str) -> Self {
        Self(Bytes::copy_from_slice(v.as_bytes()))
    }
}
impl From<Vec<u8>> for Chunk {
    fn from(v: Vec<u8>) -> Self {
        Self(Bytes::from(v))
    }
}
impl From<Bytes> for Chunk {
    fn from(v: Bytes) -> Self {
        Self(v)
    }
}

impl AsRef<[u8]> for Chunk {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
