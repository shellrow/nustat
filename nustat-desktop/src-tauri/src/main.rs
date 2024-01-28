// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sys;
mod commands;
mod task;

use std::sync::{Arc, Mutex};
use nustat_core::net::stat::NetStatStrage;
use commands::{get_remote_hosts, get_netstat, get_process_info, start_packet_capture};

fn main() {
    let netstat_strage: Arc<Mutex<NetStatStrage>> = Arc::new(Mutex::new(NetStatStrage::new()));
    tauri::Builder::default()
        .manage(netstat_strage)
        .invoke_handler(tauri::generate_handler![
            get_remote_hosts,
            get_netstat,
            get_process_info,
            start_packet_capture,
            ])
        .setup(|app| {
            let app_handle = app.handle();
            sys::init(&app_handle);
            task::start_background_task(&app_handle);
            Ok(())
            })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Destroyed => {
                sys::cleanup();
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
