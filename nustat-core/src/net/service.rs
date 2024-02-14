use serde::{Serialize, Deserialize};
use crate::net::traffic::TrafficInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceDisplayInfo {
    pub port: u16,
    pub protocol: String,
    pub name: String,
    pub traffic: TrafficInfo,
}
