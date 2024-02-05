use nustat_core::net::stat::NetStatStrage;
use nustat_core::pcap;
use std::sync::Arc;
use std::thread;

fn main() {
    let netstat_strage: Arc<NetStatStrage> = Arc::new(NetStatStrage::new());
    let netstat_strage_ui = Arc::clone(&netstat_strage);
    let usable_interfaces = nustat_core::net::interface::get_usable_interfaces();
    let pcap_thread_handlers = usable_interfaces
        .iter()
        .map(|iface| {
            let mut netstat_strage_pcap = Arc::clone(&netstat_strage);
            let iface = iface.clone();
            let iface_name = iface.name.clone();
            let pcap_option = nustat_core::pcap::PacketCaptureOptions::from_interface(&iface);
            let pcap_thread = thread::Builder::new().name(format!("pcap-thread-{}", iface_name));
            let pacp_handler = pcap_thread.spawn(move || {
                pcap::start_background_capture(pcap_option, &mut netstat_strage_pcap, iface);
            });
            println!("{}: OK", iface_name);
            (iface_name, pacp_handler)
        })
        .collect::<Vec<_>>();

    // Check netstat_strage_ui
    let ui_handle = thread::spawn(move || {
        // check netstat_strage_ui every 5 seconds
        loop {
            thread::sleep(std::time::Duration::from_secs(5));
            println!("netstat_strage_ui: {:?}", netstat_strage_ui);
        }
    });

    // Wait for all threads to finish
    for (iface_name, pacp_handler) in pcap_thread_handlers {
        match pacp_handler {
            Ok(handle) => {
                match handle.join() {
                    Ok(r) => {
                        println!("{}: {:?}", iface_name, r);
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
    ui_handle.join().expect("ui thread panicked");
    
}
