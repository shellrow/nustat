use default_net::{mac::MacAddr, Interface};
use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, HashSet}, net::{IpAddr, SocketAddr}, sync::{Arc, Mutex}};
use super::{host::{HostDisplayInfo, RemoteHostInfo}, packet::PacketFrame, service::ServiceDisplayInfo, traffic::{Direction, TrafficInfo}};
use super::interface;
use crate::{db::ip::IpDatabase, notification::Notification, process::{ProcessDisplayInfo, ProcessInfo}, socket::{AddressFamily, ProtocolPort, ProtocolSocketAddress, SocketConnection, SocketConnectionInfo, SocketTrafficInfo, TransportProtocol}};
use crate::db::service::ServiceDatabase;

#[derive(Debug, Clone)]
pub struct NetStatStrage {
    pub interface: Arc<Mutex<Interface>>,
    pub traffic: Arc<Mutex<TrafficInfo>>,
    pub remote_hosts: Arc<Mutex<HashMap<IpAddr, RemoteHostInfo>>>,
    pub sockets: Arc<Mutex<HashMap<ProtocolSocketAddress, TrafficInfo>>>,
    pub local_ports: Arc<Mutex<HashMap<ProtocolPort, TrafficInfo>>>,
    pub connections: Arc<Mutex<HashMap<SocketConnection, SocketConnectionInfo>>>,
    pub reverse_dns_map: Arc<Mutex<HashMap<IpAddr, String>>>,
    pub local_ips: Arc<Mutex<HashSet<IpAddr>>>,
    pub ipdb: Arc<Mutex<IpDatabase>>,
}

impl NetStatStrage {
    pub fn new() -> Self {
        let default_interface = match default_net::get_default_interface() {
            Ok(iface) => iface,
            Err(e) => {
                eprintln!("NetStatStrage get_default_interface error: {:?}", e);
                Interface::dummy()
            }
        };
        let local_ips = interface::get_interface_local_ips(&default_interface);
        NetStatStrage {
            interface: Arc::new(Mutex::new(default_interface)),
            traffic: Arc::new(Mutex::new(TrafficInfo::new())),
            remote_hosts: Arc::new(Mutex::new(HashMap::new())),
            sockets: Arc::new(Mutex::new(HashMap::new())),
            local_ports: Arc::new(Mutex::new(HashMap::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
            reverse_dns_map: Arc::new(Mutex::new(HashMap::new())),
            local_ips: Arc::new(Mutex::new(local_ips)),
            ipdb: Arc::new(Mutex::new(IpDatabase::new())),
        }
    }
    // Set interface
    pub fn set_interface(&self, new_interface: Interface) {
        match self.interface.lock() {
            Ok(mut iface) => {
                *iface = new_interface;
            }
            Err(e) => {
                eprintln!("set_interface error: {:?}", e);
            }
        }
    }
    // Get the interface index
    pub fn get_if_index(&self) -> u32 {
        match self.interface.lock() {
            Ok(iface) => {
                iface.index
            }
            Err(e) => {
                eprintln!("get_if_index error: {:?}", e);
                0
            }
        }
    }
    // Get the interface name
    pub fn get_if_name(&self) -> String {
        match self.interface.lock() {
            Ok(iface) => {
                iface.name.clone()
            }
            Err(e) => {
                eprintln!("get_if_name error: {:?}", e);
                String::new()
            }
        }
    }
    /// Get the traffic info. (thread safe clone)
    fn get_trrafic(&self) -> TrafficInfo {
        match self.traffic.lock() {
            Ok(traffic) => {
                traffic.clone()
            }
            Err(e) => {
                eprintln!("get_trrafic error: {:?}", e);
                TrafficInfo::new()
            }
        }
    }
    /// Set the traffic info. (thread safe set)
    /* fn set_trrafic(&mut self, new_traffic: TrafficInfo) {
        match self.traffic.lock() {
            Ok(mut traffic) => {
                *traffic = new_traffic;
            }
            Err(e) => {
                eprintln!("set_trrafic error: {:?}", e);
            }
        }
    } */
    /// Get the remote hosts. (thread safe clone)
    pub fn get_remote_hosts(&self) -> HashMap<IpAddr, RemoteHostInfo> {
        match self.remote_hosts.lock() {
            Ok(remote_hosts) => {
                remote_hosts.clone()
            }
            Err(e) => {
                eprintln!("get_remote_hosts error: {:?}", e);
                HashMap::new()
            }
        }
    }
    /// Set the remote hosts. (thread safe set)
    /* fn set_remote_hosts(&mut self, new_remote_hosts: HashMap<IpAddr, RemoteHostInfo>) {
        match self.remote_hosts.lock() {
            Ok(mut remote_hosts) => {
                *remote_hosts = new_remote_hosts;
            }
            Err(e) => {
                eprintln!("set_remote_hosts error: {:?}", e);
            }
        }
    } */
    /// Get the sockets. (thread safe clone)
    fn get_sockets(&self) -> HashMap<ProtocolSocketAddress, TrafficInfo> {
        match self.sockets.lock() {
            Ok(sockets) => {
                sockets.clone()
            }
            Err(e) => {
                eprintln!("get_sockets error: {:?}", e);
                HashMap::new()
            }
        }
    }
    /// Get the local ports. (thread safe clone)
    fn get_local_ports(&self) -> HashMap<ProtocolPort, TrafficInfo> {
        match self.local_ports.lock() {
            Ok(local_ports) => {
                local_ports.clone()
            }
            Err(e) => {
                eprintln!("get_local_ports error: {:?}", e);
                HashMap::new()
            }
        }
    }
    /// Get the connections. (thread safe clone)
    pub fn get_connections(&self) -> HashMap<SocketConnection, SocketConnectionInfo> {
        match self.connections.lock() {
            Ok(connections) => {
                connections.clone()
            }
            Err(e) => {
                eprintln!("get_connections error: {:?}", e);
                HashMap::new()
            }
        }
    }
    /// Set the connections. (thread safe set)
    /* fn set_connections(&self, new_connections: HashMap<SocketConnection, SocketConnectionInfo>) {
        match self.connections.lock() {
            Ok(mut connections) => {
                *connections = new_connections;
            }
            Err(e) => {
                eprintln!("set_connections error: {:?}", e);
            }
        }
    } */
    /// Get the reverse dns map. (thread safe clone)
    /* fn get_reverse_dns_map(&self) -> HashMap<IpAddr, String> {
        match self.reverse_dns_map.lock() {
            Ok(reverse_dns_map) => {
                reverse_dns_map.clone()
            }
            Err(e) => {
                eprintln!("get_reverse_dns_map error: {:?}", e);
                HashMap::new()
            }
        }
    } */
    /// Set the reverse dns map. (thread safe set)
    /* fn set_reverse_dns_map(&self, new_reverse_dns_map: HashMap<IpAddr, String>) {
        match self.reverse_dns_map.lock() {
            Ok(mut reverse_dns_map) => {
                *reverse_dns_map = new_reverse_dns_map;
            }
            Err(e) => {
                println!("set_reverse_dns_map error: {:?}", e);
            }
        }
    } */
    /// Get the local ips. (thread safe clone)
    /* fn get_local_ips(&self) -> HashSet<IpAddr> {
        match self.local_ips.lock() {
            Ok(local_ips) => {
                local_ips.clone()
            }
            Err(e) => {
                eprintln!("get_local_ips error: {:?}", e);
                HashSet::new()
            }
        }
    } */
    /// Set the local ips. (thread safe set)
    fn set_local_ips(&self, new_local_ips: HashSet<IpAddr>) {
        match self.local_ips.lock() {
            Ok(mut local_ips) => {
                *local_ips = new_local_ips;
            }
            Err(e) => {
                eprintln!("set_local_ips error: {:?}", e);
            }
        }
    }
    /// Get the ipdb. (thread safe clone)
    /* fn get_ipdb(&self) -> IpDatabase {
        match self.ipdb.lock() {
            Ok(ipdb) => {
                ipdb.clone()
            }
            Err(e) => {
                eprintln!("get_ipdb error: {:?}", e);
                IpDatabase::new()
            }
        }
    } */
    /// Set the ipdb. (thread safe set)
    /* fn set_ipdb(&self, new_ipdb: IpDatabase) {
        match self.ipdb.lock() {
            Ok(mut ipdb) => {
                *ipdb = new_ipdb;
            }
            Err(e) => {
                eprintln!("set_ipdb error: {:?}", e);
            }
        }
    } */
    fn clear_trraffic(&self) {
        match self.traffic.lock() {
            Ok(mut traffic) => {
                *traffic = TrafficInfo::new();
            }
            Err(e) => {
                eprintln!("clear_trraffic error: {:?}", e);
            }
        }
    }
    fn clear_remote_hosts(&self) {
        match self.remote_hosts.lock() {
            Ok(mut remote_hosts) => {
                remote_hosts.clear();
            }
            Err(e) => {
                eprintln!("clear_remote_hosts error: {:?}", e);
            }
        }
    }
    fn clear_sockets(&self) {
        match self.sockets.lock() {
            Ok(mut sockets) => {
                sockets.clear();
            }
            Err(e) => {
                eprintln!("clear_sockets error: {:?}", e);
            }
        }
    }
    fn clear_local_ports(&self) {
        match self.local_ports.lock() {
            Ok(mut local_ports) => {
                local_ports.clear();
            }
            Err(e) => {
                eprintln!("clear_local_ports error: {:?}", e);
            }
        }
    }
    fn clear_connections(&self) {
        match self.connections.lock() {
            Ok(mut connections) => {
                connections.clear();
            }
            Err(e) => {
                eprintln!("clear_connections error: {:?}", e);
            }
        }
    }
    fn clear_reverse_dns_map(&self) {
        match self.reverse_dns_map.lock() {
            Ok(mut reverse_dns_map) => {
                reverse_dns_map.clear();
            }
            Err(e) => {
                eprintln!("clear_reverse_dns_map error: {:?}", e);
            }
        }
    }
    /* fn clear_local_ips(&self) {
        match self.local_ips.lock() {
            Ok(mut local_ips) => {
                *local_ips = interface::get_default_local_ips();
            }
            Err(e) => {
                eprintln!("clear_local_ips error: {:?}", e);
            }
        }
    } */
    /* fn clear_ipdb(&self) {
        match self.ipdb.lock() {
            Ok(mut ipdb) => {
                *ipdb = IpDatabase::new();
            }
            Err(e) => {
                eprintln!("clear_ipdb error: {:?}", e);
            }
        }
    } */
    pub fn reset(&self) {
        self.clear_trraffic();
        self.clear_remote_hosts();
        self.clear_sockets();
        self.clear_local_ports();
        self.clear_connections();
        self.clear_reverse_dns_map();
    }
    pub fn reset_data(&self) {
        self.clear_trraffic();
        self.clear_remote_hosts();
        self.clear_sockets();
        self.clear_local_ports();
        self.clear_connections();
    }
    pub fn clone_and_reset(&self) -> Self {
        let clone = self.clone();
        self.reset();
        clone
    }
    pub fn clone_data_and_reset(&self) -> NetStatData {
        let mut clone: NetStatData = NetStatData::new();
        clone.if_index = self.get_if_index();
        clone.if_name = self.get_if_name();
        clone.traffic = self.get_trrafic();
        clone.remote_hosts = self.get_remote_hosts();
        clone.sockets = self.get_sockets();
        clone.local_ports = self.get_local_ports();
        clone.connections = self.get_connections();
        self.reset_data();
        clone
    }
    pub fn clone_data(&self) -> NetStatData {
        let mut clone: NetStatData = NetStatData::new();
        clone.if_index = self.get_if_index();
        clone.if_name = self.get_if_name();
        clone.traffic = self.get_trrafic();
        clone.remote_hosts = self.get_remote_hosts();
        clone.local_ports = self.get_local_ports();
        clone.sockets = self.get_sockets();
        clone.connections = self.get_connections();
        clone
    }
    pub fn change_interface(&self, interface: &Interface) {
        //self.reset();
        self.set_interface(interface.clone());
        self.set_local_ips(interface::get_interface_local_ips(interface));
    }
    pub fn interface_changed(&self, if_index: u32) -> bool {
        if if_index != self.get_if_index() {
            return true;
        }
        false
    }
    pub fn load_ipdb(&self) {
        match IpDatabase::load() {
            Ok(ipdb) => {
                let mut ipdb_mutex = self.ipdb.lock().unwrap();
                *ipdb_mutex = ipdb;
            }
            Err(e) => {
                eprintln!("load_ipdb error: {:?}", e);
            }
        }
    }
    pub fn load_ipdb_from_crate(&self) {
        match IpDatabase::load_from_crate() {
            Ok(ipdb) => {
                let mut ipdb_mutex = self.ipdb.lock().unwrap();
                *ipdb_mutex = ipdb;
            }
            Err(e) => {
                eprintln!("load_ipdb_from_crate error: {:?}", e);
            }
        }
    }
    pub fn update(&self, frame: PacketFrame) {
        let local_ips_inner = match self.local_ips.lock() {
            Ok(inner) => inner,
            Err(e) => {
                eprintln!("Failed to lock local_ips: {:?}", e);
                return;
            }
        };
        // Lock traffic field
        let mut traffic_inner = match self.traffic.lock() {
            Ok(inner) => inner,
            Err(e) => {
                // Handle lock error (e.g., log and return or panic)
                eprintln!("Failed to lock traffic: {:?}", e);
                return;
            }
        };
        // Lock remote_hosts field
        let mut remote_hosts_inner = match self.remote_hosts.lock() {
            Ok(inner) => inner,
            Err(e) => {
                // Handle lock error (e.g., log and return or panic)
                eprintln!("Failed to lock remote_hosts: {:?}", e);
                return;
            }
        };
        // Lock socket field
        let mut sockets_inner = match self.sockets.lock() {
            Ok(inner) => inner,
            Err(e) => {
                // Handle lock error (e.g., log and return or panic)
                eprintln!("Failed to lock socket: {:?}", e);
                return;
            }
        };
        // Lock local_ports field
        let mut local_ports_inner = match self.local_ports.lock() {
            Ok(inner) => inner,
            Err(e) => {
                // Handle lock error (e.g., log and return or panic)
                eprintln!("Failed to lock local_ports: {:?}", e);
                return;
            }
        };
        // Lock ipdb field
        let ipdb_inner = match self.ipdb.lock() {
            Ok(inner) => inner,
            Err(e) => {
                eprintln!("Failed to lock ipdb: {:?}", e);
                return;
            }
        }; 
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
            if local_ips_inner.contains(&IpAddr::V4(ipv4.source)) {
                Direction::Egress
            } else if local_ips_inner.contains(&IpAddr::V4(ipv4.destination)) {
                Direction::Ingress
            } else {
                return;
            }
        } else if let Some(ipv6) = &ip_layer.ipv6 {
            if local_ips_inner.contains(&IpAddr::V6(ipv6.source)) {
                Direction::Egress
            } else if local_ips_inner.contains(&IpAddr::V6(ipv6.destination)) {
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
                traffic_inner.packet_sent += 1;
                traffic_inner.bytes_sent += frame.packet_len;
            },
            Direction::Ingress => {
                traffic_inner.packet_received += 1;
                traffic_inner.bytes_received += frame.packet_len;
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
        let _local_ip_addr: IpAddr = match direction {
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
        let remote_host: &mut RemoteHostInfo = remote_hosts_inner.entry(remote_ip_addr).or_insert(RemoteHostInfo::new(
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
                if let Some(ipv4_info) = ipdb_inner.get_ipv4_info(ipv4) {
                    remote_host.country_code = ipv4_info.country_code;
                    remote_host.country_name = ipv4_info.country_name;
                    remote_host.asn = ipv4_info.asn;
                    remote_host.as_name = ipv4_info.as_name;
                }
            },
            IpAddr::V6(ipv6) => {
                if let Some(ipv6_info) = ipdb_inner.get_ipv6_info(ipv6) {
                    remote_host.country_code = ipv6_info.country_code;
                    remote_host.country_name = ipv6_info.country_name;
                    remote_host.asn = ipv6_info.asn;
                    remote_host.as_name = ipv6_info.as_name;
                }
            },
        }
        // Update SocketInfo if the packet is TCP or UDP.
        if let Some(transport) = frame.transport {
            if let Some(_tcp) = transport.tcp {
                // Update Remote SocketInfo
                let socket_connection: ProtocolSocketAddress = ProtocolSocketAddress {
                    socket: SocketAddr::new(remote_ip_addr, remote_port),
                    protocol: TransportProtocol::TCP,
                };
                let socket_traffic: &mut TrafficInfo = sockets_inner.entry(socket_connection).or_insert(TrafficInfo::new());
                match direction {
                    Direction::Egress => {
                        socket_traffic.packet_sent += 1;
                        socket_traffic.bytes_sent += frame.packet_len;
                    },
                    Direction::Ingress => {
                        socket_traffic.packet_received += 1;
                        socket_traffic.bytes_received += frame.packet_len;
                    },
                }
                // Update Local Port Info
                let local_port_connection: ProtocolPort = ProtocolPort {
                    port: local_port,
                    protocol: TransportProtocol::TCP,
                };
                let local_port_traffic: &mut TrafficInfo = local_ports_inner.entry(local_port_connection).or_insert(TrafficInfo::new());
                match direction {
                    Direction::Egress => {
                        local_port_traffic.packet_sent += 1;
                        local_port_traffic.bytes_sent += frame.packet_len;
                    },
                    Direction::Ingress => {
                        local_port_traffic.packet_received += 1;
                        local_port_traffic.bytes_received += frame.packet_len;
                    },
                }
            }
            if let Some(_udp) = transport.udp {
                // Update Remote SocketInfo
                let socket_connection: ProtocolSocketAddress = ProtocolSocketAddress {
                    socket: SocketAddr::new(remote_ip_addr, remote_port),
                    protocol: TransportProtocol::UDP,
                };
                let socket_traffic: &mut TrafficInfo = sockets_inner.entry(socket_connection).or_insert(TrafficInfo::new());
                match direction {
                    Direction::Egress => {
                        socket_traffic.packet_sent += 1;
                        socket_traffic.bytes_sent += frame.packet_len;
                    },
                    Direction::Ingress => {
                        socket_traffic.packet_received += 1;
                        socket_traffic.bytes_received += frame.packet_len;
                    },
                }
                // Update Local Port Info
                let local_port_connection: ProtocolPort = ProtocolPort {
                    port: local_port,
                    protocol: TransportProtocol::UDP,
                };
                let local_port_traffic: &mut TrafficInfo = local_ports_inner.entry(local_port_connection).or_insert(TrafficInfo::new());
                match direction {
                    Direction::Egress => {
                        local_port_traffic.packet_sent += 1;
                        local_port_traffic.bytes_sent += frame.packet_len;
                    },
                    Direction::Ingress => {
                        local_port_traffic.packet_received += 1;
                        local_port_traffic.bytes_received += frame.packet_len;
                    },
                }
            }
        }
        // Drop the locks
        drop(traffic_inner);
        drop(remote_hosts_inner);
        drop(sockets_inner);
        drop(ipdb_inner);
    }
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
    pub sockets: HashMap<ProtocolSocketAddress, TrafficInfo>,
    pub local_ports: HashMap<ProtocolPort, TrafficInfo>,
    pub connections: HashMap<SocketConnection, SocketConnectionInfo>,
}

impl NetStatData {
    pub fn new() -> Self {
        NetStatData {
            if_index: 0,
            if_name: String::new(),
            traffic: TrafficInfo::new(),
            remote_hosts: HashMap::new(),
            local_ports: HashMap::new(),
            sockets: HashMap::new(),
            connections: HashMap::new(),
        }
    }
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
        // Update SocketInfo
        other.sockets.iter().for_each(|(conn, traffic_info)| {
            match self.sockets.entry(*conn) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    let traffic_info_entry = entry.get_mut();
                    traffic_info_entry.add_traffic(traffic_info);
                },
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(traffic_info.clone());
                },
            }
        });
        // Update LocalPortInfo
        other.local_ports.iter().for_each(|(port, traffic_info)| {
            match self.local_ports.entry(*port) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    let traffic_info_entry = entry.get_mut();
                    traffic_info_entry.add_traffic(traffic_info);
                },
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(traffic_info.clone());
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
        self.connections.iter().for_each(|(conn, conn_info)| {
            let protocol_socket: ProtocolSocketAddress = ProtocolSocketAddress {
                socket: conn.remote_socket,
                protocol: conn.protocol,
            };
            let protocol_port: ProtocolPort = ProtocolPort {
                port: conn.local_socket.port(),
                protocol: conn.protocol,
            };
            if let Some(proc) = &conn_info.process {
                match process_traffic_map.get(&proc.pid) {
                    Some(traffic) => {
                        let mut traffic = traffic.clone();
                        match self.sockets.get(&protocol_socket) {
                            Some(socket_traffic) => {
                                traffic += socket_traffic.bytes_sent;
                                traffic += socket_traffic.bytes_received;
                                process_traffic_map.insert(proc.pid, traffic);
                            }
                            None => {
                                // Check local port traffic
                                match self.local_ports.get(&protocol_port) {
                                    Some(local_port_traffic) => {
                                        traffic += local_port_traffic.bytes_sent;
                                        traffic += local_port_traffic.bytes_received;
                                        process_traffic_map.insert(proc.pid, traffic);
                                    }
                                    None => {
                                        process_traffic_map.insert(proc.pid, traffic);
                                    }
                                }
                            },
                        }
                        process_traffic_map.insert(proc.pid, traffic);
                    }
                    None => {
                        match self.sockets.get(&protocol_socket) {
                            Some(traffic) => {
                                process_traffic_map.insert(proc.pid, traffic.bytes_sent + traffic.bytes_received);
                            }
                            None => {
                                // Check local port traffic
                                match self.local_ports.get(&protocol_port) {
                                    Some(local_port_traffic) => {
                                        process_traffic_map.insert(proc.pid, local_port_traffic.bytes_sent + local_port_traffic.bytes_received);
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                }
                match process_map.get(&proc.pid) {
                    Some(app_protocol) => {
                        let mut traffic = app_protocol.traffic.clone();
                        match self.sockets.get(&protocol_socket) {
                            Some(socket_traffic) => {
                                traffic.add_traffic(socket_traffic);
                                process_map.insert(proc.pid, ProcessDisplayInfo {
                                    pid: proc.pid,
                                    name: proc.name.clone(),
                                    traffic: traffic,
                                });
                            }
                            None => {
                                // Check local port traffic
                                match self.local_ports.get(&protocol_port) {
                                    Some(local_port_traffic) => {
                                        traffic.add_traffic(local_port_traffic);
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
                                            traffic: traffic,
                                        });
                                    }
                                }
                            },
                        }
                    }
                    None => {
                        match self.sockets.get(&protocol_socket) {
                            Some(traffic) => {
                                process_map.insert(proc.pid, ProcessDisplayInfo {
                                    pid: proc.pid,
                                    name: proc.name.clone(),
                                    traffic: traffic.clone(),
                                });
                            }
                            None => {
                                // Check local port traffic
                                match self.local_ports.get(&protocol_port) {
                                    Some(local_port_traffic) => {
                                        process_map.insert(proc.pid, ProcessDisplayInfo {
                                            pid: proc.pid,
                                            name: proc.name.clone(),
                                            traffic: local_port_traffic.clone(),
                                        });
                                    }
                                    None => {}
                                }
                            },
                        }
                    }
                }
            }
        });
        let mut process_traffic_vec: Vec<(&u32, &usize)> = process_traffic_map.iter().collect();
        process_traffic_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut top_processes: Vec<ProcessDisplayInfo> = Vec::new();
        for (pid, _traffic) in process_traffic_vec.iter().take(10) {
            if let Some(proc) = process_map.get(pid) {
                top_processes.push(proc.clone());
            }
        }
        top_processes
    }

    pub fn get_top_connections(&self) -> Vec<SocketTrafficInfo> {
        let mut connection_traffic_map: HashMap<SocketConnection, TrafficInfo> = HashMap::new();
        self.connections.iter().for_each(|(conn, _conn_info)| {
            let protocol_socket: ProtocolSocketAddress = ProtocolSocketAddress {
                socket: conn.remote_socket,
                protocol: conn.protocol,
            };
            let protocol_port: ProtocolPort = ProtocolPort {
                port: conn.local_socket.port(),
                protocol: conn.protocol,
            };
            match connection_traffic_map.get(&conn) {
                Some(traffic) => {
                    let mut traffic = traffic.clone();
                    match self.sockets.get(&protocol_socket) {
                        Some(socket_traffic) => {
                            traffic.add_traffic(&socket_traffic);
                        }
                        None => {
                            // Check local port traffic
                            match self.local_ports.get(&protocol_port) {
                                Some(local_port_traffic) => {
                                    traffic.add_traffic(&local_port_traffic);
                                }
                                None => {}
                            }
                        },
                    }
                    connection_traffic_map.insert(*conn, traffic);
                }
                None => {
                    match self.sockets.get(&protocol_socket) {
                        Some(socket_traffic) => {
                            connection_traffic_map.insert(*conn, socket_traffic.clone());
                        }
                        None => {
                            // Check local port traffic
                            match self.local_ports.get(&protocol_port) {
                                Some(local_port_traffic) => {
                                    connection_traffic_map.insert(*conn, local_port_traffic.clone());
                                }
                                None => {}
                            }
                        },
                    }
                }
            }
        });
        let mut connection_bytes_map: HashMap<SocketConnection, usize> = HashMap::new();
        connection_traffic_map.iter().for_each(|(conn, traffic_info)| {
            connection_bytes_map.insert(*conn, traffic_info.bytes_sent + traffic_info.bytes_received);
        });
        let mut connection_bytes_vec: Vec<(&SocketConnection, &usize)> = connection_bytes_map.iter().collect();
        connection_bytes_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut top_connections: Vec<SocketTrafficInfo> = Vec::new();
        for (conn, _) in connection_bytes_vec.iter().take(10) {
            if let Some(traffic) = connection_traffic_map.get(conn) {
                let process: Option<ProcessInfo> = match self.connections.get(conn) {
                    Some(conn_info) => conn_info.process.clone(),
                    None => None,
                };
                let socket_traffic_info: SocketTrafficInfo = SocketTrafficInfo {
                    local_ip_addr: conn.local_socket.ip(),
                    local_port: conn.local_socket.port(),
                    remote_ip_addr: Some(conn.remote_socket.ip()),
                    remote_port: Some(conn.remote_socket.port()),
                    protocol: conn.protocol,
                    ip_version: if conn.local_socket.is_ipv4() {
                        AddressFamily::IPv4
                    } else {
                        AddressFamily::IPv6
                    },
                    process: process,
                    traffic: traffic.clone(),
                };
                top_connections.push(socket_traffic_info);
            }
        }
        top_connections
    }

    pub fn get_top_app_protocols(&self) -> Vec<ServiceDisplayInfo> {
        let service_db: ServiceDatabase = crate::db::service::ServiceDatabase::new();
        let mut app_protocol_map: HashMap<u16, ServiceDisplayInfo> = HashMap::new();
        let mut app_protocol_traffic_map: HashMap<u16, usize> = HashMap::new();
        self.sockets.iter().for_each(|(conn, traffic_info)| {
            let port: u16 = conn.socket.port();
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
        overview.top_remote_hosts = self.get_top_remote_hosts();
        // Get top processes
        overview.top_processes = self.get_top_processes();
        // Get top app protocols
        overview.top_app_protocols = self.get_top_app_protocols();
        overview
    }
}
