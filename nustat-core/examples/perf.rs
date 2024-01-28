use std::{sync::{Arc, Mutex}, thread};
use nustat_core::{net::stat::NetStatStrage, pcap};

fn main() {
    let netstat_strage: Arc<Mutex<NetStatStrage>> = Arc::new(Mutex::new(NetStatStrage::new()));

    // Cloning the Arc is not necessary
    let mut netstat_strage_pcap = Arc::clone(&netstat_strage);
    let mut netstat_strage_dns = Arc::clone(&netstat_strage);
    let mut netstat_strage_socket = Arc::clone(&netstat_strage);
    let mut netstat_strage_ipinfo = Arc::clone(&netstat_strage);

    // Collect JoinHandles for threads
    let pcap_handle = thread::spawn(move || {
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

    let socket_handle = thread::spawn(move || {
        println!("[start] socket_info_update");
        nustat_core::socket::start_socket_info_update(&mut netstat_strage_socket);
    });

    let dns_handle = thread::spawn(move || {
        println!("[start] dns_map_update");
        nustat_core::dns::start_dns_map_update(&mut netstat_strage_dns);
    });

    let ipinfo_handle = thread::spawn(move || {
        println!("[start] ipinfo_update");
        nustat_core::ipinfo::start_ipinfo_update(&mut netstat_strage_ipinfo);
    });

    // Wait for all threads to finish
    pcap_handle.join().expect("pcap thread panicked");
    socket_handle.join().expect("socket thread panicked");
    dns_handle.join().expect("dns thread panicked");
    ipinfo_handle.join().expect("ipinfo thread panicked");
}
