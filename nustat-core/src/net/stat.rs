use default_net::mac::MacAddr;
use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, HashSet}, net::{IpAddr, SocketAddr}};
use super::{host::{HostDisplayInfo, RemoteHostInfo}, packet::PacketFrame, protocol::Protocol, service::ServiceDisplayInfo, traffic::{Direction, TrafficInfo}};
use super::interface;
use crate::{db::ip::IpDatabase, notification::Notification, process::ProcessDisplayInfo, socket::{PortInfo, SocketConnection, SocketConnectionInfo, SocketStatus, TransportProtocol}};
use crate::db::service::ServiceDatabase;

#[derive(Debug, Clone)]
pub struct NetStatStrage {
    pub if_index: u32,
    pub if_name: String,
    pub traffic: TrafficInfo,
    pub remote_hosts: HashMap<IpAddr, RemoteHostInfo>,
    pub connections: HashMap<SocketConnection, SocketConnectionInfo>,
    pub reverse_dns_map: HashMap<IpAddr, String>,
    pub local_ips: HashSet<IpAddr>,
    pub ipdb: IpDatabase,
}

impl NetStatStrage {
    pub fn new() -> Self {
        NetStatStrage {
            if_index: 0,
            if_name: String::new(),
            traffic: TrafficInfo::new(),
            remote_hosts: HashMap::new(),
            connections: HashMap::new(),
            reverse_dns_map: HashMap::new(),
            local_ips: interface::get_default_local_ips(),
            ipdb: IpDatabase::new(),
        }
    }
    pub fn load_ipdb(&mut self) {
        self.ipdb = IpDatabase::load().unwrap();
    }
    pub fn reset(&mut self) {
        self.traffic = TrafficInfo::new();
        self.remote_hosts.clear();
        self.connections.clear();
        self.reverse_dns_map.clear();
    }
    pub fn reset_data(&mut self) {
        self.traffic = TrafficInfo::new();
        self.remote_hosts.clear();
        self.connections.clear();
    }
    pub fn clone_and_reset(&mut self) -> Self {
        let clone = self.clone();
        self.reset();
        clone
    }
    pub fn clone_data_and_reset(&mut self) -> NetStatData {
        let clone: NetStatData = NetStatData {
            if_index: self.if_index,
            if_name: self.if_name.clone(),
            traffic: self.traffic.clone(),
            remote_hosts: self.remote_hosts.clone(),
            connections: self.connections.clone(),
        };
        self.reset_data();
        clone
    }
    pub fn change_interface(&mut self, if_index: u32) {
        self.reset();
        self.local_ips = interface::get_local_ips(if_index);
    }
    pub fn update(&mut self, frame: PacketFrame) {
        self.if_index = frame.if_index;
        self.if_name = frame.if_name;
        let datalink_layer = match frame.datalink {
            Some(datalink) => datalink,
            None => return,
        };
        let ip_layer = match frame.ip {
            Some(ip) => ip,
            None => return,
        };
        // Determine if the packet is incoming or outgoing.
        let direction: Direction = if let Some(ipv4) = &ip_layer.ipv4 {
            if self.local_ips.contains(&IpAddr::V4(ipv4.source)) {
                Direction::Egress
            } else if self.local_ips.contains(&IpAddr::V4(ipv4.destination)) {
                Direction::Ingress
            } else {
                return;
            }
        } else if let Some(ipv6) = &ip_layer.ipv6 {
            if self.local_ips.contains(&IpAddr::V6(ipv6.source)) {
                Direction::Egress
            } else if self.local_ips.contains(&IpAddr::V6(ipv6.destination)) {
                Direction::Ingress
            } else {
                return;
            }
        } else {
            return;
        };
        // Update TrafficInfo
        match direction {
            Direction::Egress => {
                self.traffic.packet_sent += 1;
                self.traffic.bytes_sent += frame.packet_len;
            },
            Direction::Ingress => {
                self.traffic.packet_received += 1;
                self.traffic.bytes_received += frame.packet_len;
            },
        }
        let mac_addr: String = match direction {
            Direction::Egress => {
                if let Some(ethernet) = datalink_layer.ethernet {
                    ethernet.destination.address()
                } else {
                    MacAddr::zero().to_string()
                }
            },
            Direction::Ingress => {
                if let Some(ethernet) = datalink_layer.ethernet {
                    ethernet.source.address()
                } else {
                    MacAddr::zero().to_string()
                }
            },
        };
        let local_ip_addr: IpAddr = match direction {
            Direction::Egress => {
                if let Some(ipv4) = &ip_layer.ipv4 {
                    IpAddr::V4(ipv4.source)
                } else if let Some(ipv6) = &ip_layer.ipv6 {
                    IpAddr::V6(ipv6.source)
                } else {
                    return;
                }
            },
            Direction::Ingress => {
                if let Some(ipv4) = &ip_layer.ipv4 {
                    IpAddr::V4(ipv4.destination)
                } else if let Some(ipv6) = &ip_layer.ipv6 {
                    IpAddr::V6(ipv6.destination)
                } else {
                    return;
                }
            },
        };
        let local_port: u16 = match direction {
            Direction::Egress => {
                if let Some(transport) = &frame.transport {
                    if let Some(tcp) = &transport.tcp {
                        tcp.source
                    } else if let Some(udp) = &transport.udp {
                        udp.source
                    } else {
                        0
                    }
                } else {
                    0
                }
            },
            Direction::Ingress => {
                if let Some(transport) = &frame.transport {
                    if let Some(tcp) = &transport.tcp {
                        tcp.destination
                    } else if let Some(udp) = &transport.udp {
                        udp.destination
                    } else {
                        0
                    }
                } else {
                    0
                }
            },
        };
        let remote_ip_addr: IpAddr = match direction {
            Direction::Egress => {
                if let Some(ipv4) = ip_layer.ipv4 {
                    IpAddr::V4(ipv4.destination)
                } else if let Some(ipv6) = ip_layer.ipv6 {
                    IpAddr::V6(ipv6.destination)
                } else {
                    return;
                }
            },
            Direction::Ingress => {
                if let Some(ipv4) = ip_layer.ipv4 {
                    IpAddr::V4(ipv4.source)
                } else if let Some(ipv6) = ip_layer.ipv6 {
                    IpAddr::V6(ipv6.source)
                } else {
                    return;
                }
            },
        };
        let remote_port: u16 = match direction {
            Direction::Egress => {
                if let Some(transport) = &frame.transport {
                    if let Some(tcp) = &transport.tcp {
                        tcp.destination
                    } else if let Some(udp) = &transport.udp {
                        udp.destination
                    } else {
                        0
                    }
                } else {
                    0
                }
            },
            Direction::Ingress => {
                if let Some(transport) = &frame.transport {
                    if let Some(tcp) = &transport.tcp {
                        tcp.source
                    } else if let Some(udp) = &transport.udp {
                        udp.source
                    } else {
                        0
                    }
                } else {
                    0
                }
            },
        };
        // Update or Insert RemoteHostInfo
        let remote_host: &mut RemoteHostInfo = self.remote_hosts.entry(remote_ip_addr).or_insert(RemoteHostInfo::new(
            mac_addr,
            remote_ip_addr,
        ));
        match direction {
            Direction::Egress => {
                remote_host.traffic_info.packet_sent += 1;
                remote_host.traffic_info.bytes_sent += frame.packet_len;
            },
            Direction::Ingress => {
                remote_host.traffic_info.packet_received += 1;
                remote_host.traffic_info.bytes_received += frame.packet_len;
            },
        }
        match remote_host.ip_addr {
            IpAddr::V4(ipv4) => {
                if let Some(ipv4_info) = self.ipdb.get_ipv4_info(ipv4) {
                    remote_host.country_code = ipv4_info.country_code;
                    remote_host.country_name = ipv4_info.country_name;
                    remote_host.asn = ipv4_info.asn;
                    remote_host.as_name = ipv4_info.as_name;
                }
            },
            IpAddr::V6(ipv6) => {
                if let Some(ipv6_info) = self.ipdb.get_ipv6_info(ipv6) {
                    remote_host.country_code = ipv6_info.country_code;
                    remote_host.country_name = ipv6_info.country_name;
                    remote_host.asn = ipv6_info.asn;
                    remote_host.as_name = ipv6_info.as_name;
                }
            },
        }
        // Update SocketInfo if the packet is TCP or UDP.
        if let Some(transport) = frame.transport {
            if let Some(tcp) = transport.tcp {
                let tcp_traffic_info: &mut TrafficInfo = remote_host.protocol_stat.entry(PortInfo::new(remote_port, TransportProtocol::TCP).to_key_string()).or_insert(TrafficInfo::new());
                match direction {
                    Direction::Egress => {
                        tcp_traffic_info.packet_sent += 1;
                        tcp_traffic_info.bytes_sent += frame.packet_len;
                    },
                    Direction::Ingress => {
                        tcp_traffic_info.packet_received += 1;
                        tcp_traffic_info.bytes_received += frame.packet_len;
                    },
                }
                // Update SocketInfo
                let socket_connection: SocketConnection = SocketConnection {
                    local_socket: SocketAddr::new(local_ip_addr, local_port),
                    remote_socket: SocketAddr::new(remote_ip_addr, remote_port),
                    protocol: Protocol::TCP,
                };
                let socket_info: &mut SocketConnectionInfo = self.connections.entry(socket_connection).or_insert(SocketConnectionInfo {
                    traffic_info: TrafficInfo::new(),
                    status: SocketStatus::from_xenet_tcp_flags(tcp.flags),
                    process: None,
                });
                match direction {
                    Direction::Egress => {
                        socket_info.traffic_info.packet_sent += 1;
                        socket_info.traffic_info.bytes_sent += frame.packet_len;
                    },
                    Direction::Ingress => {
                        socket_info.traffic_info.packet_received += 1;
                        socket_info.traffic_info.bytes_received += frame.packet_len;
                    },
                }
            }
            if let Some(_udp) = transport.udp {
                let udp_traffic_info: &mut TrafficInfo = remote_host.protocol_stat.entry(PortInfo::new(remote_port, TransportProtocol::UDP).to_key_string()).or_insert(TrafficInfo::new());
                match direction {
                    Direction::Egress => {
                        udp_traffic_info.packet_sent += 1;
                        udp_traffic_info.bytes_sent += frame.packet_len;
                    },
                    Direction::Ingress => {
                        udp_traffic_info.packet_received += 1;
                        udp_traffic_info.bytes_received += frame.packet_len;
                    },
                }
                // Update SocketInfo
                let socket_connection: SocketConnection = SocketConnection {
                    local_socket: SocketAddr::new(local_ip_addr, local_port),
                    remote_socket: SocketAddr::new(remote_ip_addr, remote_port),
                    protocol: Protocol::UDP,
                };
                let socket_info: &mut SocketConnectionInfo = self.connections.entry(socket_connection).or_insert(SocketConnectionInfo {
                    traffic_info: TrafficInfo::new(),
                    status: SocketStatus::Unknown,
                    process: None,
                });
                match direction {
                    Direction::Egress => {
                        socket_info.traffic_info.packet_sent += 1;
                        socket_info.traffic_info.bytes_sent += frame.packet_len;
                    },
                    Direction::Ingress => {
                        socket_info.traffic_info.packet_received += 1;
                        socket_info.traffic_info.bytes_received += frame.packet_len;
                    },
                }
            }
        }
    }
    /* pub fn get_overview(&self) -> Overview {
        let mut overview = Overview::new();
        match default_net::get_default_interface() {
            Ok(default_if) => {
                overview.default_if_index = default_if.index;
                overview.default_if_name = default_if.name;
            }
            Err(e) => {
                println!("get_overview default_net error: {:?}", e);
            }
        }
        // for performance, load service db from bundled data
        /* let service_db = match crate::db::service::ServiceDatabase::load() {
            Ok(db) => db,
            Err(e) => {
                println!("get_overview load service db error: {:?}", e);
                crate::db::service::ServiceDatabase::new()
            }
        }; */
        let service_db: ServiceDatabase = crate::db::service::ServiceDatabase::new();
        let mut host_traffic_map: HashMap<IpAddr, usize> = HashMap::new();
        // get total packet count
        self.remote_hosts.iter().for_each(|(_ip, host)| {
            overview.captured_packets += host.traffic_info.packet_sent;
            overview.captured_packets += host.traffic_info.packet_received;
            overview.traffic.packet_received += host.traffic_info.packet_received;
            overview.traffic.packet_sent += host.traffic_info.packet_sent;
            overview.traffic.bytes_received += host.traffic_info.bytes_received;
            overview.traffic.bytes_sent += host.traffic_info.bytes_sent;
            match host_traffic_map.get(&host.ip_addr) {
                Some(traffic) => {
                    let mut traffic = traffic.clone();
                    traffic += host.traffic_info.bytes_sent;
                    traffic += host.traffic_info.bytes_received;
                    host_traffic_map.insert(host.ip_addr, traffic);
                }
                None => {
                    host_traffic_map.insert(host.ip_addr, host.traffic_info.bytes_sent + host.traffic_info.bytes_received);
                }
            }
        });
        // Get top remote hosts
        let mut host_traffic_vec: Vec<(&IpAddr, &usize)> = host_traffic_map.iter().collect();
        host_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (ip, _) in host_traffic_vec.iter().take(4) {
            if let Some(host) = self.remote_hosts.get(ip) {
                let host = HostDisplayInfo {
                    ip_addr: host.ip_addr,
                    host_name: host.hostname.clone(),
                    country_code: host.country_code.clone(),
                    country_name: host.country_name.clone(),
                    asn: host.asn.clone(),
                    as_name: host.as_name.clone(),
                    traffic: host.traffic_info.clone(),
                };
                overview.top_remote_hosts.push(host);
            }
        }
        // Get top processes
        let mut process_map: HashMap<u32, ProcessDisplayInfo> = HashMap::new();
        let mut process_traffic_map: HashMap<u32, usize> = HashMap::new();
        self.connections.iter().for_each(|(_conn, conn_info)| {
            if let Some(proc) = &conn_info.process {
                match process_traffic_map.get(&proc.pid) {
                    Some(traffic) => {
                        let mut traffic = traffic.clone();
                        traffic += conn_info.traffic_info.bytes_sent;
                        traffic += conn_info.traffic_info.bytes_received;
                        process_traffic_map.insert(proc.pid, traffic);
                    }
                    None => {
                        process_traffic_map.insert(proc.pid, conn_info.traffic_info.bytes_sent + conn_info.traffic_info.bytes_received);
                    }
                }
                process_map.insert(proc.pid, ProcessDisplayInfo {
                    pid: proc.pid,
                    name: proc.name.clone(),
                    traffic: conn_info.traffic_info.clone(),
                });
            }
        });
        let mut process_traffic_vec: Vec<(&u32, &usize)> = process_traffic_map.iter().collect();
        process_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (pid, _) in process_traffic_vec.iter().take(4) {
            if let Some(proc) = process_map.get(pid) {
                overview.top_processes.push(proc.clone());
            }
        }
        // Get top app protocols
        let mut app_protocol_map: HashMap<u16, ServiceDisplayInfo> = HashMap::new();
        let mut app_protocol_traffic_map: HashMap<u16, usize> = HashMap::new();
        self.remote_hosts.iter().for_each(|(_ip, host)| {
            host.protocol_stat.iter().for_each(|(port_key, traffic_info)| {
                let port: u16 = match port_key.split("-").next() {
                    Some(port_str) => {
                        match port_str.parse::<u16>() {
                            Ok(port) => port,
                            Err(e) => {
                                println!("get_overview parse port error: {:?}", e);
                                0
                            }
                        }
                    }
                    None => 0,
                };
                if port == 0 {
                    return;
                }
                if let Some(service_name) = service_db.tcp_map.get(&port) {
                    match app_protocol_traffic_map.get(&port) {
                        Some(traffic) => {
                            let mut traffic = traffic.clone();
                            traffic += traffic_info.bytes_sent;
                            traffic += traffic_info.bytes_received;
                            app_protocol_traffic_map.insert(port, traffic);
                        }
                        None => {
                            app_protocol_traffic_map.insert(port, traffic_info.bytes_sent + traffic_info.bytes_received);
                        }
                    }
                    match app_protocol_map.get(&port) {
                        Some(app_protocol) => {
                            let mut traffic = app_protocol.traffic.clone();
                            traffic.add_traffic(traffic_info);
                            app_protocol_map.insert(port, ServiceDisplayInfo {
                                port: port,
                                name: service_name.clone(),
                                traffic: traffic,
                            });
                        }
                        None => {
                            app_protocol_map.insert(port, ServiceDisplayInfo {
                                port: port,
                                name: service_name.clone(),
                                traffic: traffic_info.clone(),
                            });
                        }
                    }
                }                    
            }); 
        });
        let mut app_protocol_traffic_vec: Vec<(&u16, &usize)> = app_protocol_traffic_map.iter().collect();
        app_protocol_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (port, _) in app_protocol_traffic_vec.iter().take(4) {
            if let Some(app_protocol) = app_protocol_map.get(port) {
                overview.top_app_protocols.push(app_protocol.clone());
            }
        }
        overview
    } */
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Overview {
    pub if_index: u32,
    pub if_name: String,
    pub captured_packets: usize,
    pub traffic: TrafficInfo,
    pub top_processes: Vec<ProcessDisplayInfo>,
    pub top_remote_hosts: Vec<HostDisplayInfo>,
    pub top_app_protocols: Vec<ServiceDisplayInfo>,
    pub notificatons: Vec<Notification>,
}

impl Overview {
    pub fn new() -> Self {
        Overview {
            if_index: 0,
            if_name: String::new(),
            captured_packets: 0,
            traffic: TrafficInfo::new(),
            top_processes: Vec::new(),
            top_remote_hosts: Vec::new(),
            top_app_protocols: Vec::new(),
            notificatons: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetStatData {
    pub if_index: u32,
    pub if_name: String,
    pub traffic: TrafficInfo,
    pub remote_hosts: HashMap<IpAddr, RemoteHostInfo>,
    pub connections: HashMap<SocketConnection, SocketConnectionInfo>,
}

impl NetStatData {
    pub fn new() -> Self {
        NetStatData {
            if_index: 0,
            if_name: String::new(),
            traffic: TrafficInfo::new(),
            remote_hosts: HashMap::new(),
            connections: HashMap::new(),
        }
    }
    /* pub fn merge(&mut self, other: NetStatData) {
        self.remote_hosts.extend(other.remote_hosts);
        self.connections.extend(other.connections);
    } */
    // merge using entry method to merge traffic info.
    pub fn merge(&mut self, other: NetStatData) {
        // Update Interface Info
        self.if_index = other.if_index;
        self.if_name = other.if_name;
        // Update Traffic Info
        self.traffic.add_traffic(&other.traffic);
        // Update RemoteHostInfo
        other.remote_hosts.iter().for_each(|(ip, host)| {
            match self.remote_hosts.entry(*ip) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    let host_entry = entry.get_mut();
                    host_entry.merge(host);
                },
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(host.clone());
                },
            }
        });
        // Update SocketConnectionInfo
        other.connections.iter().for_each(|(conn, conn_info)| {
            match self.connections.entry(*conn) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    let conn_info_entry = entry.get_mut();
                    conn_info_entry.merge(conn_info);
                },
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(conn_info.clone());
                },
            }
        });
    }
    pub fn get_top_remote_hosts(&self) -> Vec<HostDisplayInfo> {
        let mut host_traffic_map: HashMap<IpAddr, usize> = HashMap::new();
        self.remote_hosts.iter().for_each(|(_ip, host)| {
            match host_traffic_map.get(&host.ip_addr) {
                Some(traffic) => {
                    let mut traffic = traffic.clone();
                    traffic += host.traffic_info.bytes_sent;
                    traffic += host.traffic_info.bytes_received;
                    host_traffic_map.insert(host.ip_addr, traffic);
                }
                None => {
                    host_traffic_map.insert(host.ip_addr, host.traffic_info.bytes_sent + host.traffic_info.bytes_received);
                }
            }
        });
        let mut host_traffic_vec: Vec<(&IpAddr, &usize)> = host_traffic_map.iter().collect();
        host_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut top_remote_hosts: Vec<HostDisplayInfo> = Vec::new();
        for (ip, _) in host_traffic_vec.iter().take(10) {
            if let Some(host) = self.remote_hosts.get(ip) {
                let host = HostDisplayInfo {
                    ip_addr: host.ip_addr,
                    host_name: host.hostname.clone(),
                    country_code: host.country_code.clone(),
                    country_name: host.country_name.clone(),
                    asn: host.asn.clone(),
                    as_name: host.as_name.clone(),
                    traffic: host.traffic_info.clone(),
                };
                top_remote_hosts.push(host);
            }
        }
        top_remote_hosts
    }

    pub fn get_top_processes(&self) -> Vec<ProcessDisplayInfo> {
        let mut process_map: HashMap<u32, ProcessDisplayInfo> = HashMap::new();
        let mut process_traffic_map: HashMap<u32, usize> = HashMap::new();
        self.connections.iter().for_each(|(_conn, conn_info)| {
            if let Some(proc) = &conn_info.process {
                match process_traffic_map.get(&proc.pid) {
                    Some(traffic) => {
                        let mut traffic = traffic.clone();
                        traffic += conn_info.traffic_info.bytes_sent;
                        traffic += conn_info.traffic_info.bytes_received;
                        process_traffic_map.insert(proc.pid, traffic);
                    }
                    None => {
                        process_traffic_map.insert(proc.pid, conn_info.traffic_info.bytes_sent + conn_info.traffic_info.bytes_received);
                    }
                }
                match process_map.get(&proc.pid) {
                    Some(proc) => {
                        let mut traffic = proc.traffic.clone();
                        traffic.add_traffic(&conn_info.traffic_info);
                        process_map.insert(proc.pid, ProcessDisplayInfo {
                            pid: proc.pid,
                            name: proc.name.clone(),
                            traffic: traffic,
                        });
                    }
                    None => {
                        process_map.insert(proc.pid, ProcessDisplayInfo {
                            pid: proc.pid,
                            name: proc.name.clone(),
                            traffic: conn_info.traffic_info.clone(),
                        });
                    }
                }
            }
        });
        let mut process_traffic_vec: Vec<(&u32, &usize)> = process_traffic_map.iter().collect();
        process_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut top_processes: Vec<ProcessDisplayInfo> = Vec::new();
        for (pid, _) in process_traffic_vec.iter().take(10) {
            if let Some(proc) = process_map.get(pid) {
                top_processes.push(proc.clone());
            }
        }
        top_processes
    }

    pub fn get_top_app_protocols(&self) -> Vec<ServiceDisplayInfo> {
        let service_db: ServiceDatabase = crate::db::service::ServiceDatabase::new();
        let mut app_protocol_map: HashMap<u16, ServiceDisplayInfo> = HashMap::new();
        let mut app_protocol_traffic_map: HashMap<u16, usize> = HashMap::new();
        self.remote_hosts.iter().for_each(|(_ip, host)| {
            host.protocol_stat.iter().for_each(|(port_key, traffic_info)| {
                let port: u16 = match port_key.split("-").next() {
                    Some(port_str) => {
                        match port_str.parse::<u16>() {
                            Ok(port) => port,
                            Err(e) => {
                                println!("get_overview parse port error: {:?}", e);
                                0
                            }
                        }
                    }
                    None => 0,
                };
                if port == 0 {
                    return;
                }
                if let Some(service_name) = service_db.tcp_map.get(&port) {
                    match app_protocol_traffic_map.get(&port) {
                        Some(traffic) => {
                            let mut traffic = traffic.clone();
                            traffic += traffic_info.bytes_sent;
                            traffic += traffic_info.bytes_received;
                            app_protocol_traffic_map.insert(port, traffic);
                        }
                        None => {
                            app_protocol_traffic_map.insert(port, traffic_info.bytes_sent + traffic_info.bytes_received);
                        }
                    }
                    match app_protocol_map.get(&port) {
                        Some(app_protocol) => {
                            let mut traffic = app_protocol.traffic.clone();
                            traffic.add_traffic(traffic_info);
                            app_protocol_map.insert(port, ServiceDisplayInfo {
                                port: port,
                                name: service_name.clone(),
                                traffic: traffic,
                            });
                        }
                        None => {
                            app_protocol_map.insert(port, ServiceDisplayInfo {
                                port: port,
                                name: service_name.clone(),
                                traffic: traffic_info.clone(),
                            });
                        }
                    }
                }                    
            }); 
        });
        let mut app_protocol_traffic_vec: Vec<(&u16, &usize)> = app_protocol_traffic_map.iter().collect();
        app_protocol_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut top_app_protocols: Vec<ServiceDisplayInfo> = Vec::new();
        for (port, _) in app_protocol_traffic_vec.iter().take(10) {
            if let Some(app_protocol) = app_protocol_map.get(port) {
                top_app_protocols.push(app_protocol.clone());
            }
        }
        top_app_protocols
    }

    pub fn get_overview(&self) -> Overview {
        let mut overview = Overview::new();
        overview.if_index = self.if_index;
        overview.if_name = self.if_name.clone();

        let service_db: ServiceDatabase = crate::db::service::ServiceDatabase::new();
        let mut host_traffic_map: HashMap<IpAddr, usize> = HashMap::new();
        // get total packet count
        self.remote_hosts.iter().for_each(|(_ip, host)| {
            overview.captured_packets += host.traffic_info.packet_sent;
            overview.captured_packets += host.traffic_info.packet_received;
            overview.traffic.packet_received += host.traffic_info.packet_received;
            overview.traffic.packet_sent += host.traffic_info.packet_sent;
            overview.traffic.bytes_received += host.traffic_info.bytes_received;
            overview.traffic.bytes_sent += host.traffic_info.bytes_sent;
            match host_traffic_map.get(&host.ip_addr) {
                Some(traffic) => {
                    let mut traffic = traffic.clone();
                    traffic += host.traffic_info.bytes_sent;
                    traffic += host.traffic_info.bytes_received;
                    host_traffic_map.insert(host.ip_addr, traffic);
                }
                None => {
                    host_traffic_map.insert(host.ip_addr, host.traffic_info.bytes_sent + host.traffic_info.bytes_received);
                }
            }
        });
        // Get top remote hosts
        let mut host_traffic_vec: Vec<(&IpAddr, &usize)> = host_traffic_map.iter().collect();
        host_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (ip, _) in host_traffic_vec.iter().take(4) {
            if let Some(host) = self.remote_hosts.get(ip) {
                let host = HostDisplayInfo {
                    ip_addr: host.ip_addr,
                    host_name: host.hostname.clone(),
                    country_code: host.country_code.clone(),
                    country_name: host.country_name.clone(),
                    asn: host.asn.clone(),
                    as_name: host.as_name.clone(),
                    traffic: host.traffic_info.clone(),
                };
                overview.top_remote_hosts.push(host);
            }
        }
        // Get top processes
        let mut process_map: HashMap<u32, ProcessDisplayInfo> = HashMap::new();
        let mut process_traffic_map: HashMap<u32, usize> = HashMap::new();
        self.connections.iter().for_each(|(_conn, conn_info)| {
            if let Some(proc) = &conn_info.process {
                match process_traffic_map.get(&proc.pid) {
                    Some(traffic) => {
                        let mut traffic = traffic.clone();
                        traffic += conn_info.traffic_info.bytes_sent;
                        traffic += conn_info.traffic_info.bytes_received;
                        process_traffic_map.insert(proc.pid, traffic);
                    }
                    None => {
                        process_traffic_map.insert(proc.pid, conn_info.traffic_info.bytes_sent + conn_info.traffic_info.bytes_received);
                    }
                }
                process_map.insert(proc.pid, ProcessDisplayInfo {
                    pid: proc.pid,
                    name: proc.name.clone(),
                    traffic: conn_info.traffic_info.clone(),
                });
            }
        });
        let mut process_traffic_vec: Vec<(&u32, &usize)> = process_traffic_map.iter().collect();
        process_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (pid, _) in process_traffic_vec.iter().take(4) {
            if let Some(proc) = process_map.get(pid) {
                overview.top_processes.push(proc.clone());
            }
        }
        // Get top app protocols
        let mut app_protocol_map: HashMap<u16, ServiceDisplayInfo> = HashMap::new();
        let mut app_protocol_traffic_map: HashMap<u16, usize> = HashMap::new();
        self.remote_hosts.iter().for_each(|(_ip, host)| {
            host.protocol_stat.iter().for_each(|(port_key, traffic_info)| {
                let port: u16 = match port_key.split("-").next() {
                    Some(port_str) => {
                        match port_str.parse::<u16>() {
                            Ok(port) => port,
                            Err(e) => {
                                println!("get_overview parse port error: {:?}", e);
                                0
                            }
                        }
                    }
                    None => 0,
                };
                if port == 0 {
                    return;
                }
                if let Some(service_name) = service_db.tcp_map.get(&port) {
                    match app_protocol_traffic_map.get(&port) {
                        Some(traffic) => {
                            let mut traffic = traffic.clone();
                            traffic += traffic_info.bytes_sent;
                            traffic += traffic_info.bytes_received;
                            app_protocol_traffic_map.insert(port, traffic);
                        }
                        None => {
                            app_protocol_traffic_map.insert(port, traffic_info.bytes_sent + traffic_info.bytes_received);
                        }
                    }
                    match app_protocol_map.get(&port) {
                        Some(app_protocol) => {
                            let mut traffic = app_protocol.traffic.clone();
                            traffic.add_traffic(traffic_info);
                            app_protocol_map.insert(port, ServiceDisplayInfo {
                                port: port,
                                name: service_name.clone(),
                                traffic: traffic,
                            });
                        }
                        None => {
                            app_protocol_map.insert(port, ServiceDisplayInfo {
                                port: port,
                                name: service_name.clone(),
                                traffic: traffic_info.clone(),
                            });
                        }
                    }
                }                    
            }); 
        });
        let mut app_protocol_traffic_vec: Vec<(&u16, &usize)> = app_protocol_traffic_map.iter().collect();
        app_protocol_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (port, _) in app_protocol_traffic_vec.iter().take(4) {
            if let Some(app_protocol) = app_protocol_map.get(port) {
                overview.top_app_protocols.push(app_protocol.clone());
            }
        }
        overview
    }
}
