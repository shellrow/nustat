use nustat_core::net::packet::PacketFrame;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let (tx, rx): (Sender<PacketFrame>, Receiver<PacketFrame>) = channel();
    let stop = Arc::new(Mutex::new(false));
    let stop_handle = stop.clone();
    let default_interface = default_net::get_default_interface().unwrap();
    let pcap_option = nustat_core::pcap::PacketCaptureOptions::from_interface(&default_interface);
    let pacp_handler = thread::spawn(move || {
        nustat_core::pcap::start_capture(pcap_option, tx, &stop, default_interface)
    });
    let print_handler = thread::spawn(move || {
        let mut count: usize = 0;
        while let Ok(frame) = rx.recv() {
            println!("frame: {:?}", frame);
            count += 1;
        }
        println!("count: {}", count);
    });
    thread::sleep(std::time::Duration::from_secs(30));
    match stop_handle.lock() {
        Ok(mut stop) => {
            *stop = true;
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }    
    
    match pacp_handler.join() {
        Ok(r) => {
            println!("pacp_handler: {:?}", r);
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
}