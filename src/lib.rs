#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
pub mod prelude;
pub mod error;    pub use error::{ Result, Error };

pub mod flag;     pub use flag::Flag;
pub mod state;    pub use state::{ State, StateGuard };

pub use once_cell::{ self, sync::Lazy };
pub use arc_swap::{ self, ArcSwap };

#[cfg(any(feature = "json-config", feature = "toml-config"))]
pub mod config;
#[cfg(any(feature = "json-config", feature = "toml-config"))]
pub use config::Config;

#[cfg(any(feature = "logger"))]
pub mod logger;
#[cfg(any(feature = "logger"))]
pub use logger::Logger;

#[cfg(any(feature = "logger"))]
pub use log::{ self, info, warn, error, debug, trace };

/// Initializes a static variable by 'once_cell::Lazy'
#[macro_export]
macro_rules! lazy {
    ($e:expr) => {
        $crate::Lazy::new(|| $e)
    }
}
