pub mod db;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Country {
    pub country_code: String,
    pub country_name: String,
}
