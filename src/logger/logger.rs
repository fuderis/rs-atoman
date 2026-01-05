use crate::prelude::*;
use std::{ fs::{ self, File }, io::Write, path::PathBuf };
use chrono::Utc;

/// The logger instance
static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new());

/// The logger
pub struct Logger {
    path: State<Option<PathBuf>>,
    file: State<Option<Arc<File>>>,
    cursor: State<usize>,
}

impl Logger {
    /// Creates a new instance of logger
    fn new() -> Self {
        Self {
            path: State::from(None),
            file: State::from(None),
            cursor: State::from(0),
        }
    }

    /// Returns the current .log file path
    pub fn get_path() -> Option<PathBuf> {
        LOGGER.path.get_cloned()
    }

    /// Returns an unreaded log lines
    pub fn read_logs() -> Result<Option<Vec<String>>> {
        Ok(match Self::get_path() {
            Some(path) => {
                let cursor = LOGGER.cursor.get();
                let contents = fs::read_to_string(path)?;
                let lines = contents[*cursor..].lines()
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>();

                LOGGER.cursor.set(contents.len());
                Some(lines)
            }
            _ => None
        })
    }
    
    /// Returns a log lines of current .log file
    pub fn read_all_logs() -> Result<Option<Vec<String>>> {
        Ok(match Self::get_path() {
            Some(path) => {
                let contents = fs::read_to_string(path)?;
                let lines = contents.lines()
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>();
                
                LOGGER.cursor.set(contents.len());
                Some(lines)
            }
            _ => None
        })
    }
    
    /// Initializes logger
    pub fn init<P: Into<PathBuf>>(logs_dir: P, max_files: usize) -> Result<()> {
        let logs_dir = logs_dir.into();
        
        // write log file:
        let (path, file) = if max_files > 0 {
            let file_path = logs_dir.join( Utc::now().format("%Y-%m-%d_%H-%M-%S%.6f.log").to_string() );

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

        LOGGER.path.set(path);
        LOGGER.file.set(file);
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
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        let msg = record.args().to_string();
        
        if self.enabled(record.metadata())
        && &msg != "NewEvents emitted without explicit RedrawEventsCleared"
        && &msg != "RedrawEventsCleared emitted without explicit MainEventsCleared"
        {
            let dt = Utc::now().format("%Y-%m-%dT%H:%M:%S%.6f");
            let color_code = match record.level() {
                log::Level::Info  => "\x1b[32m",   // green
                log::Level::Warn  => "\x1b[33m",   // yellow
                log::Level::Error => "\x1b[31m",   // red
                _ => "\x1b[0m",                    // default
            };
            let reset_code = "\x1b[0m";

            // printing log to terminal:
            println!("{dt}Z  {color}{lvl}{reset} {msg}",
                color = color_code,
                lvl = record.level(),
                reset = reset_code,
                msg = record.args()
            );

            // writing log to file:
            if let Some(file) = self.file.lock().as_mut() {
                let _ = file.write_all(
                    fmt!(
                        "{dt}Z  {lvl} {msg}\n",
                        lvl = record.level(),
                        msg = record.args()
                    ).as_bytes()
                );
                let _ = file.flush();
            }
        }
    }

    fn flush(&self) {}
}
