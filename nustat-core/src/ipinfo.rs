use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::sync::{Arc, Mutex};
use crate::db::table::{Ipv4Info, Ipv6Info};
use crate::net::stat::NetStatStrage;
use crate::db;

pub fn get_ipv4info_map(target_ips: Vec<Ipv4Addr>) -> HashMap<Ipv4Addr, Ipv4Info> {
    let mut map: HashMap<Ipv4Addr, Ipv4Info> = HashMap::new();
    let conn = db::connect_db(db::IP_DB_NAME).unwrap();
    for ipv4 in target_ips {
        if !map.contains_key(&ipv4) {
            match Ipv4Info::get_ipv4_info(&conn, ipv4) {
                Some(ipv4_info) => {
                    map.insert(ipv4, ipv4_info);
                }
                None => {},
            }
        }
    }
    map
}

pub fn get_ipv6info_map(target_ips: Vec<Ipv6Addr>) -> HashMap<Ipv6Addr, Ipv6Info> {
    let mut map: HashMap<Ipv6Addr, Ipv6Info> = HashMap::new();
    let conn = db::connect_db(db::IP_DB_NAME).unwrap();
    for ipv6 in target_ips {
        if !map.contains_key(&ipv6) {
            match Ipv6Info::get_ipv6_info(&conn, ipv6) {
                Some(ipv6_info) => {
                    map.insert(ipv6, ipv6_info);
                }
                None => {},
            }
        }
    }
    map
}

// Lookup ip.db and update remote_hosts.
// Target: country_code, country_name, asn, as_name
pub fn start_ipinfo_update(netstat_strage: &mut Arc<Mutex<NetStatStrage>>) {
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
        let ipv4_map = get_ipv4info_map(target_ipv4);
        let ipv6_map = get_ipv6info_map(target_ipv6);
        println!("[ipinfo_update] target ipv4 addresses: {}", ipv4_map.len());
        println!("[ipinfo_update] target ipv6 addresses: {}", ipv6_map.len());
        match netstat_strage.try_lock() {
            Ok(mut netstat_strage) => {
                for (ipv4_addr, ipv4_info) in ipv4_map {
                    if let Some(remote_host) = netstat_strage.remote_hosts.get_mut(&IpAddr::V4(ipv4_addr)) {
                        remote_host.country_code = ipv4_info.country_code;
                        remote_host.country_name = ipv4_info.country_name;
                        remote_host.asn = ipv4_info.asn;
                        remote_host.as_name = ipv4_info.as_name;
                    }
                }
                for (ipv6_addr, ipv6_info) in ipv6_map {
                    if let Some(remote_host) = netstat_strage.remote_hosts.get_mut(&IpAddr::V6(ipv6_addr)) {
                        remote_host.country_code = ipv6_info.country_code;
                        remote_host.country_name = ipv6_info.country_name;
                        remote_host.asn = ipv6_info.asn;
                        remote_host.as_name = ipv6_info.as_name;
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