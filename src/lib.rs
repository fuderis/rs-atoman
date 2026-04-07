#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![allow(clippy::module_inception)]
pub mod error;
pub mod prelude;
pub use error::Error;

/// The dynamic error type
pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
/// The short result alias
pub type Result<T> = std::result::Result<T, DynError>;
/// The std result alias
pub use std::result::Result as StdResult;

pub mod flag;
pub use flag::Flag;
pub mod state;
pub use state::{State, StateGuard};

pub use arc_swap::{self, ArcSwap};
pub use once_cell::{self, sync::Lazy};

#[cfg(any(feature = "json-config", feature = "toml-config"))]
pub mod config;
#[cfg(any(feature = "json-config", feature = "toml-config"))]
pub use config::Config;

#[cfg(feature = "logger")]
pub mod logger;
#[cfg(feature = "logger")]
pub use logger::Logger;

#[cfg(feature = "logger")]
pub use log::{self, Level, debug, error, info, trace, warn};

#[cfg(feature = "trace")]
pub mod trace;
#[cfg(feature = "trace")]
pub use trace::Trace;

#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "stream")]
pub use bytes::{self, Bytes};
#[cfg(feature = "stream")]
pub use futures::{self, StreamExt};
#[cfg(feature = "stream")]
pub use stream::{Stream, StreamReader, StreamSender};

#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "server")]
pub use axum;
#[cfg(feature = "server")]
pub use server::{Response, Server};

#[cfg(feature = "file")]
pub mod file;
#[cfg(feature = "file")]
pub use file::{Dir, Entry, File, FileKind, Metadata, OpenMode, SeekFrom};

/// Initializes a static variable by 'once_cell::Lazy'
#[macro_export]
macro_rules! lazy {
    ($e:expr) => {
        $crate::Lazy::new(|| $e)
    };
}
