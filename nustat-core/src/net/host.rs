use std::{collections::HashMap, net::IpAddr};

use serde::{Serialize, Deserialize};

use crate::sys;

use super::traffic::TrafficInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteHostInfo {
    pub mac_addr: String,
    pub ip_addr: IpAddr,
    pub hostname: String,
    pub country_code: String,
    pub country_name: String,
    pub asn: u32,
    pub as_name: String,
    pub traffic_info: TrafficInfo,
    pub protocol_stat: HashMap<String, TrafficInfo>,
    pub first_seen: String,
    pub updated_at: String,
}

impl RemoteHostInfo {
    pub fn new(mac_addr: String, ip_addr: IpAddr) -> Self {
        RemoteHostInfo {
            mac_addr: mac_addr,
            ip_addr: ip_addr,
            hostname: String::new(),
            country_code: String::new(),
            country_name: String::new(),
            asn: 0,
            as_name: String::new(),
            traffic_info: TrafficInfo::new(),
            protocol_stat: HashMap::new(),
            first_seen: sys::get_sysdate(),
            updated_at: sys::get_sysdate(),
        }
    }
    pub fn merge(&mut self, other: &RemoteHostInfo) {
        // Update traffic_info and protocol_stat
        self.traffic_info.add_traffic(&other.traffic_info);
        for (proto, traffic) in &other.protocol_stat {
            if self.protocol_stat.contains_key(proto) {
                let traffic_info = self.protocol_stat.get_mut(proto).unwrap();
                traffic_info.add_traffic(traffic);
            } else {
                self.protocol_stat.insert(proto.clone(), traffic.clone());
            }
        }
        // Update other fields
        if self.hostname.is_empty() {
            self.hostname = other.hostname.clone();
        }
        if self.country_code.is_empty() {
            self.country_code = other.country_code.clone();
        }
        if self.country_name.is_empty() {
            self.country_name = other.country_name.clone();
        }
        if self.asn == 0 {
            self.asn = other.asn;
        }
        if self.as_name.is_empty() {
            self.as_name = other.as_name.clone();
        }
        self.updated_at = sys::get_sysdate();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostDisplayInfo {
    pub ip_addr: IpAddr,
    pub host_name: String,
    pub country_code: String,
    pub country_name: String,
    pub asn: u32,
    pub as_name: String,
    pub traffic: TrafficInfo,
}
