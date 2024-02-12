use std::path::PathBuf;
use crate::thread_log;
pub const USER_CONFIG_DIR_NAME: &str = ".nustat";

#[cfg(target_os = "windows")]
pub fn get_os_type() -> String {
    "windows".to_owned()
}

#[cfg(target_os = "linux")]
pub fn get_os_type() -> String {
    "linux".to_owned()
}

#[cfg(target_os = "macos")]
pub fn get_os_type() -> String {
    "macos".to_owned()
}

pub fn get_sysdate() -> String {
    let now = chrono::Local::now();
    now.to_rfc3339()
}

pub fn get_config_dir_path() -> Option<PathBuf> {
    match home::home_dir() {
        Some(mut path) => {
            path.push(USER_CONFIG_DIR_NAME);
            if !path.exists() {
                match std::fs::create_dir_all(&path) {
                    Ok(_) => {}
                    Err(e) => {
                        thread_log!(error, "{:?}", e);
                        return None;
                    }
                }
            }
            Some(path)
        }
        None => None,
    }
}

pub fn get_user_file_path(file_name: &str) -> Option<PathBuf> {
    match get_config_dir_path() {
        Some(mut path) => {
            path.push(file_name);
            Some(path)
        }
        None => None,
    }
}
