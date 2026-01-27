use crate::prelude::*;
use chrono::Utc;
use log::Level;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

/// The logger instance
static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new());

/// The logger
pub struct Logger {
    level: State<Option<Level>>,
    path: State<Option<PathBuf>>,
    file: State<Option<Arc<File>>>,
}

impl Logger {
    /// Creates a new instance of logger
    fn new() -> Self {
        Self {
            level: State::from(Some(Level::Info)),
            path: State::from(None),
            file: State::from(None),
        }
    }

    /// Returns the current .log file path
    pub fn get_path() -> Option<PathBuf> {
        LOGGER.path.unsafe_get_cloned()
    }

    /// Returns a log lines of current .log file
    pub fn read_logs() -> Result<Option<Vec<String>>> {
        Ok(match Self::get_path() {
            Some(path) => {
                let contents = fs::read_to_string(path)?;
                let lines = contents.lines().map(|s| s.to_owned()).collect::<Vec<_>>();

                Some(lines)
            }
            _ => None,
        })
    }

    /// Returns log level
    pub fn get_level() -> Level {
        LOGGER.level.unsafe_get_cloned().unwrap_or(Level::Info)
    }

    /// Sets minimum log level
    pub fn set_level(level: Level) {
        LOGGER.level.unsafe_set(Some(level));
    }

    /// Initializes logger
    pub fn init<P: Into<PathBuf>>(logs_dir: P, max_files: usize) -> Result<()> {
        let logs_dir = logs_dir.into();

        // write log file:
        let (path, file) = if max_files > 0 {
            let file_path =
                logs_dir.join(Utc::now().format("%Y-%m-%d_%H-%M-%S%.6f.log").to_string());

            // create logs dir:
            fs::create_dir_all(&logs_dir)?;

            // read logs dir:
            let mut log_files: Vec<PathBuf> = fs::read_dir(&logs_dir)?
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "log") {
                        Some(path)
                    } else {
                        None
                    }
                })
                .collect();

            // sort log files by time:
            log_files.sort_by_key(|path| fs::metadata(path).and_then(|m| m.created()).ok());

            // remove extra files:
            if log_files.len() > max_files {
                for old_file in &log_files[0..log_files.len() - max_files] {
                    let _ = fs::remove_file(old_file);
                }
            }

            // create a new file:
            let file = File::create(&file_path)?;
            (Some(file_path), Some(Arc::new(file)))
        } else {
            (None, None)
        };

        LOGGER.path.unsafe_set(path);
        LOGGER.file.unsafe_set(file);
        LOGGER.init_self()
    }

    /// Helper method to initialize logger
    fn init_self(&'static self) -> Result<()> {
        log::set_logger(self).map_err(|e| Error::from(e))?;
        log::set_max_level(log::LevelFilter::Info);
        Ok(())
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Self::get_level()
    }

    fn log(&self, record: &log::Record) {
        let msg = record.args().to_string();

        if self.enabled(record.metadata())
            && &msg != "NewEvents emitted without explicit RedrawEventsCleared"
            && &msg != "RedrawEventsCleared emitted without explicit MainEventsCleared"
        {
            let dt = Utc::now().format("%Y-%m-%dT%H:%M:%S%.6f");
            let prefix = match record.level() {
                log::Level::Info => "  ",
                log::Level::Warn => "  ",
                log::Level::Error => " ",
                log::Level::Debug => " ",
                log::Level::Trace => " ",
            };
            let color = match record.level() {
                log::Level::Info => "\x1b[32m",  // green
                log::Level::Warn => "\x1b[33m",  // yellow
                log::Level::Error => "\x1b[31m", // red
                log::Level::Debug => "\x1b[34m", // blue
                log::Level::Trace => "\x1b[36m", // cyan
            };
            let reset_color = "\x1b[0m";

            // printing log to terminal:
            println!(
                "{dt}Z{prefix}{color}{lvl}{reset} {msg}",
                lvl = record.level(),
                reset = reset_color,
                msg = record.args()
            );

            // writing log to file:
            if let Some(file) = self.file.unsafe_lock().as_mut() {
                let _ = file.write_all(
                    fmt!(
                        "{dt}Z{prefix}{lvl} {msg}\n",
                        lvl = record.level(),
                        msg = record.args()
                    )
                    .as_bytes(),
                );
                let _ = file.flush();
            }
        }
    }

    fn flush(&self) {}
}
