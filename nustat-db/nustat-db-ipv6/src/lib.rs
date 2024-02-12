pub mod db;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ipv6Info {
    pub ip_from: u128,
    pub ip_to: u128,
    pub country_code: String,
    pub asn: u32,
}
