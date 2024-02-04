use serde::{Deserialize, Serialize};
use nustat_core::notification::Notification;
use nustat_core::net::traffic::TrafficInfo;
use super::process::ProcessDisplayInfo;
use super::host::HostDisplayInfo;
use super::socket::ServiceDisplayInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Overview {
    pub default_if_index: u32,
    pub default_if_name: String,
    pub captured_packets: usize,
    pub traffic: TrafficInfo,
    pub top_processes: Vec<ProcessDisplayInfo>,
    pub top_remote_hosts: Vec<HostDisplayInfo>,
    pub top_app_protocols: Vec<ServiceDisplayInfo>,
    pub notificatons: Vec<Notification>,
}

/* impl Overview {
    pub fn new() -> Self {
        Overview {
            default_if_index: 0,
            default_if_name: String::new(),
            captured_packets: 0,
            traffic: TrafficInfo::new(),
            top_processes: Vec::new(),
            top_remote_hosts: Vec::new(),
            top_app_protocols: Vec::new(),
            notificatons: Vec::new(),
        }
    }
} */
