use std::collections::HashMap;
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
use nustat_core::net::stat::Overview;

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
    let mut process_list: Vec<ProcessInfo> = Vec::new();
    let mut process_map: HashMap<u32, TrafficInfo> = HashMap::new();
    let sockets_inner = match netstat.sockets.try_lock() {
        Ok(sockets) => {
            sockets
        }
        Err(e) => {
            eprintln!("[get_process_info] lock error: {}", e);
            return processes;
        }
    };
    for (conn, conn_info) in netstat.get_connections() {
        if let Some(proc) = &conn_info.process {
            match process_map.get(&proc.pid) {
                Some(traffic_info) => {
                    let mut traffic_info = traffic_info.clone();
                    match sockets_inner.get(&conn) {
                        Some(socket_traffic) => {
                            traffic_info.add_traffic(&socket_traffic);
                        }
                        None => {}
                    }
                    process_map.insert(proc.pid, traffic_info);
                }
                None => {
                    match sockets_inner.get(&conn) {
                        Some(socket_traffic) => {
                            process_map.insert(proc.pid, socket_traffic.clone());
                        }
                        None => {
                            process_map.insert(proc.pid, TrafficInfo::new());
                        }
                    }
                    process_list.push(proc.clone());
                }
            }
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
pub fn get_overview(netstat: State<'_, Arc<NetStatStrage>>) -> Overview {
    let netstat_data = netstat.clone_data();
    netstat_data.get_overview()
}
