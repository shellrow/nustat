use serde::{Deserialize, Serialize};
use crate::thread_log;
use crate::sys;
use crate::log::LogLevel;
use crate::log::DEFAULT_LOG_FILE_PATH;
pub const NUSTAT_CONFIG_FILE_NAME: &str = "nustat-config.json";

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    /// Logging configuration.
    pub logging: LoggingConfig,
    /// Network configuration.
    pub network: NetworkConfig,
    /// Display configuration.
    pub display: DisplayConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            logging: LoggingConfig::new(),
            network: NetworkConfig::new(),
            display: DisplayConfig::new(),
        }
    }
    pub fn load() -> AppConfig {
        match sys::get_user_file_path(NUSTAT_CONFIG_FILE_NAME) {
            Some(path) => {
                match std::fs::read_to_string(&path) {
                    Ok(content) => {
                        match serde_json::from_str(&content) {
                            Ok(config) => config,
                            Err(e) => {
                                thread_log!(error, "{:?}", e);
                                AppConfig::new()
                            }
                        }
                    }
                    Err(e) => {
                        thread_log!(error, "{:?}", e);
                        // Create default config
                        let config = AppConfig::new();
                        config.save();
                        config
                    }
                }
            }
            None => {
                // Create default config
                let config = AppConfig::new();
                config.save();
                config
            },
        }
    }
    pub fn save(&self) {
        if let Some(path) = sys::get_user_file_path(NUSTAT_CONFIG_FILE_NAME) {
            match serde_json::to_string_pretty(&self) {
                Ok(content) => {
                    match std::fs::write(&path, content) {
                        Ok(_) => {}
                        Err(e) => {
                            thread_log!(error, "{:?}", e);
                        }
                    }
                }
                Err(e) => {
                    thread_log!(error, "{:?}", e);
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoggingConfig {
    /// Log level.
    pub level: LogLevel,
    /// Log file path.
    pub file_path: Option<String>,
}

impl LoggingConfig {
    pub fn new() -> LoggingConfig {
        LoggingConfig {
            level: LogLevel::ERROR,
            file_path: if let Some(path) = sys::get_user_file_path(DEFAULT_LOG_FILE_PATH) {
                Some(path.to_string_lossy().to_string())
            }else {
                None
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    /// Network interfaces to use. If empty, all interfaces will be use.
    pub interfaces: Vec<String>,
    /// Enable reverse DNS lookup.
    pub reverse_dns: bool,
}

impl NetworkConfig {
    pub fn new() -> NetworkConfig {
        NetworkConfig {
            interfaces: Vec::new(),
            reverse_dns: false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayConfig {
    /// The number of top remote hosts to display in the Overview tab.
    pub top_remote_hosts: usize,
    /// The number of connections to display in the Overview tab.
    pub connection_count: usize,
    /// The tick rate in milliseconds. Default is 1000.
    /// This is the default rate at which the UI will update.
    pub tick_rate: u64,
}

impl DisplayConfig {
    pub fn new() -> DisplayConfig {
        DisplayConfig {
            top_remote_hosts: 10,
            connection_count: 10,
            tick_rate: 1000,
        }
    }
}
