mod sys;
mod app;
mod terminal;
mod ui;
mod handler;

use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::error::Error;
use clap::{Arg, Command, ArgMatches};
use clap::{crate_name, crate_version, crate_description, value_parser};
use nustat_core::net::stat::NetStatStrage;
use nustat_core::config::AppConfig;
use nustat_core::thread_log;
use simplelog::WriteLogger;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let app: ArgMatches = get_app_settings();
    // Check update
    if let Some(update_app) = app.subcommand_matches("update") {
        println!("Checking for updates...");
        if update_app.contains_id("db") {
            println!("Checking for DB file updates...");
            handler::check_db_files()?;
        }
        return Ok(());
    }

    // Check .nustat directory
    match nustat_core::sys::get_config_dir_path() {
        Some(_config_dir) => {
            // Check DB files
            //handler::check_db_files()?;
        }
        None => {
            eprintln!("Error: Could not get config directory path");
            return Ok(());
        }
    }

    // Load AppConfig
    let mut config = AppConfig::load();

    if app.contains_id("tick_rate") {
        config.display.tick_rate = *app.get_one("tick_rate").unwrap_or(&1000);
    }

    // Init logger
    let log_file_path = if let Some(file_path) = &config.logging.file_path {
        // Convert to PathBuf
        Path::new(&file_path).to_path_buf()
    } else {
        nustat_core::sys::get_user_file_path(nustat_core::log::DEFAULT_LOG_FILE_PATH).unwrap()
    };
    let log_file: File = if log_file_path.exists() {
        File::options().write(true).open(&log_file_path)?
    } else {
        File::create(&log_file_path)?
    };
    WriteLogger::init(
        config.logging.level.to_level_filter(),
        Default::default(),
        log_file,
    )?;

    // Start threads
    let mut threads: Vec<thread::JoinHandle<()>> = vec![];

    let netstat_strage: Arc<NetStatStrage> = Arc::new(NetStatStrage::new());
    let mut netstat_strage_socket = Arc::clone(&netstat_strage);
    let mut netstat_strage_ui = Arc::clone(&netstat_strage);

    let usable_interfaces = nustat_core::net::interface::get_usable_interfaces();
    let mut pcap_thread_index = 0;
    let pcap_handlers = usable_interfaces
        .iter()
        .map(|iface| {
            let mut netstat_strage_pcap = Arc::clone(&netstat_strage);
            let iface = iface.clone();
            let pcap_option = nustat_core::pcap::PacketCaptureOptions::from_interface(&iface);
            let pcap_thread = thread::Builder::new().name(format!("pcap-thread-{}", iface.name.clone()));
            let pcap_handler = pcap_thread.spawn(move || {
                if pcap_thread_index == 0 {
                    netstat_strage_pcap.load_ipdb_from_crate();
                }
                nustat_core::pcap::start_background_capture(pcap_option, &mut netstat_strage_pcap, iface);
            });
            pcap_thread_index += 1;
            pcap_handler
        })
        .collect::<Vec<_>>();

    let socket_handler = thread::spawn(move || {
        nustat_core::socket::start_socket_info_update(&mut netstat_strage_socket);
    });

    for pcap_handler in pcap_handlers {
        match pcap_handler {
            Ok(handle) => {
                threads.push(handle);
            }
            Err(e) => {
                thread_log!(error, "Error: {:?}", e);
            }
        }
    }
    threads.push(socket_handler);

    if config.network.reverse_dns {
        let mut netstat_strage_dns = Arc::clone(&netstat_strage);
        let dns_handler = thread::spawn(move || {
            nustat_core::dns::start_dns_map_update(&mut netstat_strage_dns);
        });
        threads.push(dns_handler);
    }

    /* let ui_handler = thread::spawn(move || {
        let _ = crate::terminal::run(tick_rate, cli.enhanced_graphics, &mut netstat_strage_ui);
    });
    threads.push(ui_handler); */
    crate::terminal::run(config, app.contains_id("enhanced_graphics"), &mut netstat_strage_ui)?;
    Ok(())
}

fn get_app_settings() -> ArgMatches {
    let app: Command = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::new("tick_rate")
            .help("Time in ms between two ticks")
            .long("tick_rate")
            .value_name("duration_ms")
            .value_parser(value_parser!(u64))
        )
        .arg(Arg::new("enhanced_graphics")
            .help("Whether unicode symbols are used to improve the overall look of the app")
            .long("enhanced_graphics")
            .num_args(0)
        )
        // Sub-command for update db files
        .subcommand(Command::new("update")
            .about("Check update. nustat update --help for more information")
            .arg(Arg::new("db")
                .help("Update the database files")
                .long("db")
                .required(false)
                .num_args(0)
            )
        )
        ;
    app.get_matches()
}
