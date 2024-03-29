use std::thread;
use std::sync::Arc;
use nustat_core::pcap;
use tauri::Manager;

pub fn start_background_task(handle: &tauri::AppHandle) {
    
    let netstat_strage = handle.state::<Arc<nustat_core::net::stat::NetStatStrage>>();
    // For background packet capture
    let mut netstat_strage_pcap = Arc::clone(&netstat_strage);
    // For socket info update
    let mut netstat_strage_socket = Arc::clone(&netstat_strage);
    // For DNS Map update
    let mut netstat_strage_dns = Arc::clone(&netstat_strage);
    // For IP Info update
    //let mut netstat_strage_ipinfo = Arc::clone(&netstat_strage);
    thread::spawn(move || {
        netstat_strage_pcap.load_ipdb_from_crate();
        println!("[start] background_capture");
        match default_net::get_default_interface() {
            Ok(iface) => {
                let pcap_option = nustat_core::pcap::PacketCaptureOptions::from_interface(&iface);
                pcap::start_background_capture(pcap_option, &mut netstat_strage_pcap, iface);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
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
    /* thread::spawn(move || {
        println!("[start] ipinfo_update");
        nustat_core::ipinfo::start_ipinfo_update(&mut netstat_strage_ipinfo);
    }); */
}
