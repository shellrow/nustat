use serde::{Deserialize, Serialize};
use nustat_core::net::traffic::TrafficInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessDisplayInfo {
    pub pid: u32,
    pub name: String,
    pub traffic: TrafficInfo,
}
