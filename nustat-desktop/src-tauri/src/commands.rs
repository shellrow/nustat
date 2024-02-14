use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use nustat_core::net::host::RemoteHostInfo;
use nustat_core::net::stat::NetStatStrage;
use nustat_core::net::traffic::TrafficInfo;
use nustat_core::process::{ProcessInfo, ProcessTrafficInfo};
use tauri::{Manager, State};
use nustat_core::socket::{LocalSocket, SocketInfo, SocketInfoOption};
use nustat_core::pcap::CaptureReport;
use nustat_core::net::packet::PacketFrame;
use nustat_core::net::stat::Overview;

#[tauri::command]
pub async fn start_packet_capture(app_handle: tauri::AppHandle) -> CaptureReport {
    let mut report = CaptureReport::new();
    let (tx, rx): (Sender<PacketFrame>, Receiver<PacketFrame>) = channel();
    let stop = Arc::new(Mutex::new(false));
    let stop_handle = stop.clone();
    let default_interface = default_net::get_default_interface().unwrap();
    let pcap_option = nustat_core::pcap::PacketCaptureOptions::from_interface(&default_interface);
    let pacp_handler = thread::spawn(move || {
        nustat_core::pcap::start_capture(pcap_option, tx, &stop, default_interface)
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
pub fn get_remote_hosts(netstat: State<'_, Arc<NetStatStrage>>) -> Vec<RemoteHostInfo> {
    let mut hosts: Vec<RemoteHostInfo> = Vec::new();
    for (_ip_addr, host) in netstat.get_remote_hosts() {
        hosts.push(host);
    }
    hosts
}

#[tauri::command]
pub fn get_process_info(netstat: State<'_, Arc<NetStatStrage>>) -> Vec<ProcessTrafficInfo> {
    let mut processes: Vec<ProcessTrafficInfo> = Vec::new();
    let mut process_traffic_map: HashMap<u32, TrafficInfo> = HashMap::new();
    let mut process_map: HashMap<u32, ProcessInfo> = HashMap::new();
    let connection_map = netstat.get_connection_map();
    let local_socket_map = netstat.get_local_socket_map();
    connection_map.iter().for_each(|(conn, traffic_info)| {
        let local_socket: LocalSocket = LocalSocket {
            interface_name: conn.interface_name.clone(),
            port: conn.local_port,
            protocol: conn.protocol,
        };
        match local_socket_map.get(&local_socket) {
            Some(socket_process) => {
                if let Some(process) = &socket_process.process {
                    match process_traffic_map.get(&process.pid) {
                        Some(traffic) => {
                            let mut traffic = traffic.clone();
                            traffic.add_traffic(traffic_info);
                            process_traffic_map.insert(process.pid, traffic);
                            
                        }
                        None => {
                            process_traffic_map.insert(process.pid, traffic_info.clone());
                        }
                    }
                    process_map.insert(process.pid, process.clone());
                }
            }
            None => {}
        }
    });
    process_map.iter().for_each(|(pid, process)| {
        match process_traffic_map.get(pid) {
            Some(traffic) => {
                processes.push(ProcessTrafficInfo {
                    process: process.clone(),
                    traffic: traffic.clone(),
                });
            }
            None => {
                processes.push(ProcessTrafficInfo {
                    process: process.clone(),
                    traffic: TrafficInfo::new(),
                });
            }
        }
    });
    processes
}

#[tauri::command]
pub fn get_overview(netstat: State<'_, Arc<NetStatStrage>>) -> Overview {
    let netstat_data = netstat.clone_data();
    netstat_data.get_overview()
}
