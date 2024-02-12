pub mod db;

use std::path::PathBuf;

use serde::{Serialize, Deserialize};

const USER_CONFIG_DIR_NAME: &str = ".nustat";
const CONTENT_BASE_URL: &str = "https://raw.githubusercontent.com/shellrow/nustat";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ipv6Info {
    pub ip_from: u128,
    pub ip_to: u128,
    pub country_code: String,
    pub asn: u32,
}

impl Ipv6Info {
    pub fn bin_file_path() -> Option<PathBuf> {
        match home::home_dir() {
            Some(mut path) => {
                path.push(USER_CONFIG_DIR_NAME);
                path.push(db::IPV6_INFO_BIN_NAME);
                Some(path)
            }
            None => None,
        }
    }
    pub fn get_github_url(commit_hash: &str) -> String {
        format!("{}/{}/nustat-db/nustat-db-ipv6/resources/{}", CONTENT_BASE_URL, commit_hash, db::IPV6_INFO_BIN_NAME)
    }
}
