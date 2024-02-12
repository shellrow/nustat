pub mod db;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ipv4Info {
    pub ip_from: u32,
    pub ip_to: u32,
    pub country_code: String,
    pub asn: u32,
}
