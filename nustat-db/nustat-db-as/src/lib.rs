pub mod db;

use std::path::PathBuf;

use serde::{Serialize, Deserialize};

const USER_CONFIG_DIR_NAME: &str = ".nustat";
const CONTENT_BASE_URL: &str = "https://raw.githubusercontent.com/shellrow/nustat";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AutonomousSystem {
    pub asn: u32,
    pub as_name: String,
}

impl AutonomousSystem {
    pub fn bin_file_path() -> Option<PathBuf> {
        match home::home_dir() {
            Some(mut path) => {
                path.push(USER_CONFIG_DIR_NAME);
                path.push(db::AS_BIN_NAME);
                Some(path)
            }
            None => None,
        }
    }
    pub fn get_github_url(commit_hash: &str) -> String {
        format!("{}/{}/nustat-db/nustat-db-as/resources/{}", CONTENT_BASE_URL, commit_hash, db::AS_BIN_NAME)
    }
}
