pub mod db;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AutonomousSystem {
    pub asn: u32,
    pub as_name: String,
}
