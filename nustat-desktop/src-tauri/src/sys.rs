use std::thread;
use std::sync::{Arc, Mutex};
use nustat_core::db;
use nustat_core::pcap;
use tauri::Manager;

pub fn init(handle: tauri::AppHandle) {
    // For background packet capture
    let netstat_strage_state = handle.state::<Arc<Mutex<nustat_core::net::stat::NetStatStrage>>>();
    let mut netstat_strage_pcap = netstat_strage_state.inner().clone();
    // For DNS Map update
    let mut netstat_strage_dns = netstat_strage_state.inner().clone();
    // For socket info update
    let mut netstat_strage_socket = netstat_strage_state.inner().clone();
    thread::spawn(move || {
        println!("[start] background_capture");
        match nustat_core::pcap::PacketCaptureOptions::default() {
            Ok(pcap_option) => {
                pcap::start_background_capture(pcap_option, &mut netstat_strage_pcap);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    });
    thread::spawn(move || {
        println!("[start] socket_info_update");
        nustat_core::socket::start_socket_info_update(&mut netstat_strage_socket);
    });
    thread::spawn(move || {
        println!("[start] dns_map_update");
        nustat_core::dns::start_dns_map_update(&mut netstat_strage_dns);
    });
    match db::init_db() {
        Ok(_) => {
            println!("Database initialized");
            /* tauri::async_runtime::spawn(async move {
                
            }); */
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
    if db::ip_db_exists() {
        // For IP info update
        let mut netstat_strage_ipinfo = netstat_strage_state.inner().clone();
        thread::spawn(move || {
            println!("[start] ipinfo_update");
            nustat_core::ipinfo::start_ipinfo_update(&mut netstat_strage_ipinfo);
        });
    }
}

pub fn cleanup() {
    println!("Cleanup");
    match db::cleanup_db() {
        Ok(_) => {
            println!("Database cleaned up");
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
