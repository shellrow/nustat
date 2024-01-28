use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

use crate::db::ip::IpDatabase;
use crate::net::stat::NetStatStrage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IpInfo {
    pub ip_addr: IpAddr,
    pub country_code: String,
    pub country_name: String,
    pub asn: u32,
    pub as_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ipv4Info {
    pub ip_addr: Ipv4Addr,
    pub country_code: String,
    pub country_name: String,
    pub asn: u32,
    pub as_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ipv6Info {
    pub ip_addr: Ipv6Addr,
    pub country_code: String,
    pub country_name: String,
    pub asn: u32,
    pub as_name: String,
}

// Lookup ip.db and update remote_hosts.
// Target: country_code, country_name, asn, as_name
pub fn start_ipinfo_update(netstat_strage: &mut Arc<Mutex<NetStatStrage>>) {
    let start = std::time::Instant::now();
    let ipdb = IpDatabase::load().unwrap();
    println!("[ipinfo_update] load ipdb: {} ms", start.elapsed().as_millis());
    loop {
        let mut target_ipv4: Vec<Ipv4Addr> = vec![];
        let mut target_ipv6: Vec<Ipv6Addr> = vec![];
        match netstat_strage.try_lock() {
            Ok(netstat_strage) => {
                // Find IP addresses that have not been resolved yet.
                for (ip_addr, remote_host) in &netstat_strage.remote_hosts {
                    if remote_host.country_code.is_empty() {
                        match ip_addr {
                            IpAddr::V4(ipv4) => {
                                target_ipv4.push(*ipv4);
                            }
                            IpAddr::V6(ipv6) => {
                                target_ipv6.push(*ipv6);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("[ipinfo_update] lock error: {}", e);
            }
        }
        println!("[ipinfo_update] target ipv4 addresses: {}", target_ipv4.len());
        println!("[ipinfo_update] target ipv6 addresses: {}", target_ipv6.len());
        match netstat_strage.try_lock() {
            Ok(mut netstat_strage) => {
                for ipv4 in target_ipv4 {
                    if let Some (ipv4_info) = ipdb.get_ipv4_info(ipv4) {
                        if let Some(remote_host) = netstat_strage.remote_hosts.get_mut(&IpAddr::V4(ipv4)) {
                            remote_host.country_code = ipv4_info.country_code;
                            remote_host.country_name = ipv4_info.country_name;
                            remote_host.asn = ipv4_info.asn;
                            remote_host.as_name = ipv4_info.as_name;
                        }
                    }
                }
                for ipv6 in target_ipv6 {
                    if let Some (ipv6_info) = ipdb.get_ipv6_info(ipv6) {
                        if let Some(remote_host) = netstat_strage.remote_hosts.get_mut(&IpAddr::V6(ipv6)) {
                            remote_host.country_code = ipv6_info.country_code;
                            remote_host.country_name = ipv6_info.country_name;
                            remote_host.asn = ipv6_info.asn;
                            remote_host.as_name = ipv6_info.as_name;
                        }
                    }
                }
            }
            Err(e) => {
                println!("[ipinfo_update] lock error: {}", e);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(8));
    }
}