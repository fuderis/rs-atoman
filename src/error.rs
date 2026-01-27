use macron::{Display, Error, From};

/// Std Result alias
pub type StdResult<T, E> = std::result::Result<T, E>;
/// Result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    Io(std::io::Error),

    #[cfg(feature = "logger")]
    #[from]
    #[display = "Logger initialize error: {0}"]
    LoggerInit(log::SetLoggerError),

    #[cfg(any(feature = "json-config", feature = "toml-config"))]
    #[display = "Unsupported config extension '.{0}'."]
    ConfigExt(String),

    //    #[cfg(feature = "trace")]
    #[display = "Failed to open file: {0}"]
    OpenFile(std::io::Error),

    #[display = "Failed to read file: {0}"]
    ReadFile(std::io::Error),
}
