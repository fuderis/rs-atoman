#![allow(unused_imports)]
pub(crate) use crate::error::Error;
pub(crate) use macron::prelude::*;
pub(crate) use std::fmt::Debug as Debugging;
pub(crate) use std::fmt::Display as Displaying;
pub(crate) use std::path::{Path, PathBuf};
pub(crate) use std::sync::{
    Arc, Mutex, MutexGuard,
    atomic::{AtomicBool, Ordering},
};
pub(crate) use tokio::sync::Notify;

pub use crate::{DynError, Result, StdResult};

pub use arc_swap::ArcSwapAny;
pub use once_cell::{self, sync::Lazy};

pub use crate::{Flag, State, StateGuard, lazy};

#[cfg(any(feature = "json-config", feature = "toml-config"))]
pub use crate::config::Config;

#[cfg(feature = "logger")]
pub use crate::{Level, debug, error, info, logger::Logger, trace, warn};

#[cfg(feature = "trace")]
pub use crate::trace::Trace;

#[cfg(feature = "stream")]
pub use crate::stream::{Stream, StreamReader, StreamSender};
#[cfg(feature = "stream")]
pub use bytes::Bytes;
#[cfg(feature = "stream")]
pub use futures::StreamExt;

#[cfg(feature = "server")]
pub use crate::server::{Response, Server};
#[cfg(feature = "server")]
pub use std::net::SocketAddr;
