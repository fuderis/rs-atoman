// use crate::prelude::*;
use bytes::Bytes;

/// The stream chunk wrapper
#[derive(Debug, Clone)]
pub struct Chunk(pub Bytes);

impl Chunk {
    /// Creates a chunk from bytes
    pub fn bytes(b: impl Into<Bytes>) -> Self {
        Self(b.into())
    }

    /// Creates a chunk from string
    pub fn str<T: AsRef<str>>(s: T) -> Self {
        Self(Bytes::from(s.as_ref()))
    }

    /// Creates a chunk from any structure that can be converted to JSON
    pub fn json<T: Serialize>(v: &T) -> Self {
        let b = serde_json::to_vec(v).unwrap_or_default();
        Self(Bytes::from(b))
    }
}

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
