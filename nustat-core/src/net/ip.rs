use default_net::mac::MacAddr;
use netprobe::{neighbor::DeviceResolver, setting::ProbeSetting};
use std::{collections::HashMap, net::{IpAddr, Ipv4Addr, Ipv6Addr}};
use xenet::net::ipnet::{Ipv4Net, Ipv6Net};
use crate::net::interface;

pub fn get_network_address(ip_addr: IpAddr) -> Result<String, String> {
    match ip_addr {
        IpAddr::V4(ipv4_addr) => {
            let net: Ipv4Net = Ipv4Net::new(ipv4_addr, 24);
            Ok(net.network().to_string())
        }
        IpAddr::V6(ipv6_addr) => {
            let net: Ipv6Net = Ipv6Net::new(ipv6_addr, 24);
            Ok(net.network().to_string())
        }
    }
}

pub fn is_global_addr(ip_addr: IpAddr) -> bool {
    match ip_addr {
        IpAddr::V4(ipv4) => xenet::net::ipnet::is_global_ipv4(&ipv4),
        IpAddr::V6(ipv6) => xenet::net::ipnet::is_global_ipv6(&ipv6),
    }
}

pub fn in_same_network(src_ip: IpAddr, dst_ip: IpAddr) -> bool {
    let src_ip_nw = match get_network_address(src_ip) {
        Ok(nw) => nw,
        Err(_) => return false,
    };
    let dst_ip_nw = match get_network_address(dst_ip) {
        Ok(nw) => nw,
        Err(_) => return false,
    };
    if src_ip_nw == dst_ip_nw {
        true
    } else {
        false
    }
}

pub fn guess_initial_ttl(ttl: u8) -> u8 {
    if ttl <= 64 {
        64
    } else if 64 < ttl && ttl <= 128 {
        128
    } else {
        255
    }
}

pub fn get_mac_addresses(ips: Vec<IpAddr>, src_ip: IpAddr) -> HashMap<IpAddr, String> {
    let mut map: HashMap<IpAddr, String> = HashMap::new();
    if let Some(c_interface) = interface::get_interface_by_ip(src_ip) {
        for ip in ips {
            if ip == src_ip {
                map.insert(
                    ip,
                    c_interface
                        .clone()
                        .mac_addr
                        .unwrap_or(MacAddr::zero())
                        .to_string(),
                );
                continue;
            }
            if !is_global_addr(ip) && in_same_network(src_ip, ip) {
                let setting: ProbeSetting = match ip {
                    IpAddr::V4(ipv4) => ProbeSetting::arp(c_interface.clone(), ipv4, 1).unwrap(),
                    IpAddr::V6(ipv6) => ProbeSetting::ndp(c_interface.clone(), ipv6, 1).unwrap(),
                };
                let resolver: DeviceResolver = DeviceResolver::new(setting).unwrap();
                match resolver.resolve() {
                    Ok(result) => {
                        if result.results.len() > 0 {
                            map.insert(ip, result.results[0].mac_addr.address());
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
    map
}

pub fn ipv4_to_int(ipv4: Ipv4Addr) -> u64 {
    //let ipv4: Ipv4Addr = ip_addr.parse().unwrap_or(Ipv4Addr::LOCALHOST);
    let o1:u64 = ipv4.octets()[0].into();
    let o2:u64 = ipv4.octets()[1].into();
    let o3:u64 = ipv4.octets()[2].into();
    let o4:u64 = ipv4.octets()[3].into();
    let ip_int: u64 = ((o1<<24)+(o2<<16)+(o3<<8)+o4).into();
    return ip_int;
}

pub fn ipv6_to_dec(ipv6: Ipv6Addr)-> u128 {
    //let ipv6: Ipv6Addr = ip_addr.parse().unwrap_or(Ipv6Addr::LOCALHOST);
    let segments: [u16; 8] = ipv6.segments();
    let mut ip_int: u128 = 0;
    for i in 0..ipv6.segments().len() {
        let cur_seg: u128 = segments[(ipv6.segments().len() - 1) - i].into();
        ip_int = (cur_seg << i*16) | ip_int;
    }
    return ip_int;
}
