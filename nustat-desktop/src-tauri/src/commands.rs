use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use nustat_core::net::host::RemoteHostInfo;
use nustat_core::net::stat::NetStatStrage;
use nustat_core::net::traffic::TrafficInfo;
use nustat_core::process::{ProcessInfo, ProcessTrafficInfo};
use tauri::{Manager, State};
use nustat_core::socket::{SocketInfo, SocketInfoOption};
use nustat_core::pcap::CaptureReport;
use nustat_core::net::packet::PacketFrame;

use crate::components::host::HostDisplayInfo;
use crate::components::overview::Overview;
use crate::components::process::ProcessDisplayInfo;
use crate::components::socket::ServiceDisplayInfo;

#[tauri::command]
pub async fn start_packet_capture(app_handle: tauri::AppHandle) -> CaptureReport {
    let mut report = CaptureReport::new();
    let (tx, rx): (Sender<PacketFrame>, Receiver<PacketFrame>) = channel();
    let stop = Arc::new(Mutex::new(false));
    let stop_handle = stop.clone();
    let pcap_option = nustat_core::pcap::PacketCaptureOptions::default();
    let pacp_handler = thread::spawn(move || {
        nustat_core::pcap::start_capture(pcap_option.unwrap(), tx, &stop)
    });
    let stop_pcap_event = app_handle.listen_global("stop_pcap", move |event| {
        println!("got stop_pcap with payload {:?}", event.payload());
        match stop_handle.lock() {
            Ok(mut stop) => {
                *stop = true;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    });
    let print_handler = thread::spawn(move || {
        while let Ok(frame) = rx.recv() {
            match app_handle.emit_all("packet_frame", frame) {
                Ok(_) => {

                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        app_handle.unlisten(stop_pcap_event);
    });
    match pacp_handler.join() {
        Ok(r) => {
            println!("pacp_handler: {:?}", r);
            report = r;
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
    match print_handler.join() {
        Ok(_) => {
            
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
    report
}

#[tauri::command]
pub fn get_netstat(opt: SocketInfoOption) -> Vec<SocketInfo> {
    nustat_core::socket::get_sockets_info(opt)
}

#[tauri::command]
pub fn get_remote_hosts(netstat: State<'_, Arc<Mutex<NetStatStrage>>>) -> Vec<RemoteHostInfo> {
    match netstat.try_lock() {
        Ok(netstat_strage) => {
            let mut hosts: Vec<RemoteHostInfo> = Vec::new();
            for host in netstat_strage.remote_hosts.values() {
                hosts.push(host.clone());
            }
            hosts
        }
        Err(e) => {
            println!("get_remote_hosts lock error: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn get_process_info(netstat: State<'_, Arc<Mutex<NetStatStrage>>>) -> Vec<ProcessTrafficInfo> {
    let mut processes: Vec<ProcessTrafficInfo> = Vec::new();
    let mut process_list: Vec<ProcessInfo> = Vec::new();
    let mut process_map: HashMap<u32, TrafficInfo> = HashMap::new();
    match netstat.try_lock() {
        Ok(netstat_strage) => {
            for (_conn, conn_info) in netstat_strage.connections.iter() {
                if let Some(proc) = &conn_info.process {
                    match process_map.get(&proc.pid) {
                        Some(traffic_info) => {
                            let mut traffic_info = traffic_info.clone();
                            traffic_info.add_traffic(&conn_info.traffic_info);
                            process_map.insert(proc.pid, traffic_info);
                        }
                        None => {
                            process_map.insert(proc.pid, conn_info.traffic_info.clone());
                            process_list.push(proc.clone());
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("get_remote_hosts lock error: {:?}", e);
            return processes;
        }
    }
    for proc in process_list {
        if let Some(traffic_info) = process_map.get(&proc.pid) {
            processes.push(ProcessTrafficInfo {
                process: proc,
                traffic: traffic_info.clone(),
            });
        }
    }
    processes
}

#[tauri::command]
pub fn get_overview(netstat: State<'_, Arc<Mutex<NetStatStrage>>>) -> Overview {
    let start = std::time::Instant::now();
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
    let service_db = match nustat_core::db::service::ServiceDatabase::load() {
        Ok(db) => db,
        Err(e) => {
            println!("get_overview load service db error: {:?}", e);
            nustat_core::db::service::ServiceDatabase::new()
        }
    };
    match netstat.try_lock() {
        Ok(netstat_strage) => {
            let mut host_traffic_map: HashMap<IpAddr, usize> = HashMap::new();
            // get total packet count
            netstat_strage.remote_hosts.iter().for_each(|(_ip, host)| {
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
                if let Some(host) = netstat_strage.remote_hosts.get(ip) {
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
            netstat_strage.connections.iter().for_each(|(_conn, conn_info)| {
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
            netstat_strage.remote_hosts.iter().for_each(|(_ip, host)| {
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
        }
        Err(e) => {
            println!("get_overview lock error: {:?}", e);
        }
    }
    println!("get_overview: {:?}", start.elapsed());
    overview
}
