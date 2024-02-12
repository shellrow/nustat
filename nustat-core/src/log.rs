use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

pub const DEFAULT_LOG_FILE_PATH: &str = "nustat.log";

/// Global Mutex lock guard for logging.
pub static LOG_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// Thread-safe logging macro. This macro is used to log messages from multiple threads.
#[macro_export]
macro_rules! thread_log {
    ($log_macro: ident, $($fmt_args:expr),*) => {{
        let guard = $crate::log::LOG_LOCK.get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        log::$log_macro!($($fmt_args,)*);
        drop(guard);
    }};
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    ERROR,
}

impl LogLevel {
    pub fn allows(&self, level: &LogLevel) -> bool {
        match self {
            LogLevel::DEBUG => true,
            LogLevel::INFO => level != &LogLevel::DEBUG,
            LogLevel::WARNING => level == &LogLevel::WARNING || level == &LogLevel::ERROR,
            LogLevel::ERROR => level == &LogLevel::ERROR,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            LogLevel::DEBUG => "DEBUG",
            LogLevel::INFO => "INFO",
            LogLevel::WARNING => "WARNING",
            LogLevel::ERROR => "ERROR",
        }
        .to_owned()
    }
    pub fn to_level_filter(&self) -> simplelog::LevelFilter {
        match self {
            LogLevel::DEBUG => simplelog::LevelFilter::Debug,
            LogLevel::INFO => simplelog::LevelFilter::Info,
            LogLevel::WARNING => simplelog::LevelFilter::Warn,
            LogLevel::ERROR => simplelog::LevelFilter::Error,
        }
    }
}

/* #[derive(Deserialize, Serialize, Debug)]
pub struct AppLogger {
    level: LogLevel,
    file_path: PathBuf,
}

impl AppLogger {
    pub fn new() -> AppLogger {
        let file_path = if let Some(path) = sys::get_user_file_path(DEFAULT_LOG_FILE_PATH) {
            path
        } else {
            PathBuf::new()
        };
        AppLogger {
            level: LogLevel::Error,
            file_path,
        }
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn set_file_path(&mut self, file_path: &Path) {
        self.file_path = file_path.to_path_buf();
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if !self.level.allows(&level) {
            return;
        }
        if let Some(file_path) = self.file_path.to_str() {
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
            {
                if let Err(e) = writeln!(file, "{}: {}", level.to_string(), message) {
                    error!("Error: {:?}", e);
                }
            }
        }
    }
}
 */

/* pub fn write_log(level: &LogLevel, message: &str) {
    let log = format!("[{} {:?}] {}", sys::get_sysdate(), level, message);
    // Write log to file. If file not exists, create it.
    if let Some(path) = sys::get_user_file_path(DEFAULT_LOG_FILE_PATH) {
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
            if let Err(e) = writeln!(file, "{}", log) {
                error!("Error: {:?}", e);
            }
        }
    }
} */
