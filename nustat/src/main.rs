use std::{error::Error, time::Duration};
use argh::FromArgs;

mod app;
mod terminal;
mod ui;

use std::sync::{Arc, Mutex};
use std::thread;
use nustat_core::net::stat::NetStatStrage;

/// Newtwork utilization statistics
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "false")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);

    let mut threads: Vec<thread::JoinHandle<()>> = vec![];

    let netstat_strage: Arc<Mutex<NetStatStrage>> = Arc::new(Mutex::new(NetStatStrage::new()));

    let mut netstat_strage_pcap = netstat_strage.clone();
    let pcap_handler = thread::spawn(move || {
        match netstat_strage_pcap.lock() {
            Ok(mut netstat_strage) => {
                netstat_strage.load_ipdb();
            }
            Err(e) => {
                eprintln!("load_ipdb lock error: {:?}", e);
            }
        }
        match nustat_core::pcap::PacketCaptureOptions::default() {
            Ok(pcap_option) => {
                nustat_core::pcap::start_background_capture(pcap_option, &mut netstat_strage_pcap);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    });

    let mut netstat_strage_socket = netstat_strage.clone();
    let socket_handler = thread::spawn(move || {
        nustat_core::socket::start_socket_info_update(&mut netstat_strage_socket);
    });

    let mut netstat_strage_dns = netstat_strage.clone();
    let dns_handler = thread::spawn(move || {
        nustat_core::dns::start_dns_map_update(&mut netstat_strage_dns);
    });

    threads.push(pcap_handler);
    threads.push(socket_handler);
    threads.push(dns_handler);

    let mut netstat_strage_ui = netstat_strage.clone();
    /* let ui_handler = thread::spawn(move || {
        let _ = crate::terminal::run(tick_rate, cli.enhanced_graphics, &mut netstat_strage_ui);
    });
    threads.push(ui_handler); */
    crate::terminal::run(tick_rate, cli.enhanced_graphics, &mut netstat_strage_ui)?;
    Ok(())
}