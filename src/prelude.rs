#![allow(unused_imports)]

pub use crate::error::{Error, Result, StdResult};

pub(crate) use std::fmt::Debug as Debugging;
pub(crate) use std::fmt::Display as Displaying;
pub(crate) use std::format as fmt;
pub(crate) use std::sync::{
    Arc, Mutex, MutexGuard,
    atomic::{AtomicBool, Ordering},
};
pub(crate) use tokio::sync::Notify;

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
pub use crate::stream::Stream;
#[cfg(feature = "stream")]
pub use bytes::Bytes;
#[cfg(feature = "stream")]
pub use futures::StreamExt;
