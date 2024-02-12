use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::sys;

pub const TCP_SERVICE_BIN_NAME: &str = "tcp-service.bin";
pub const TCP_SERVICE_BIN: &[u8] = include_bytes!("../../../nustat-db/nustat-db-service/resources/tcp-service.bin");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TcpService {
    pub port: u16, 
    pub service_name: String, 
}

impl TcpService {
    pub fn bin_file_path() -> Option<PathBuf> {
        match sys::get_config_dir_path() {
            Some(mut path) => {
                path.push(TCP_SERVICE_BIN_NAME);
                Some(path)
            }
            None => None,
        }
    }
}

pub fn get_bundled_tcp_service() -> HashMap<u16, String> {
    let mut tcp_map: HashMap<u16, String> = HashMap::new();
    let tcp_services: Vec<TcpService> = bincode::deserialize(TCP_SERVICE_BIN).unwrap_or(vec![]);
    for port_info in tcp_services {
        tcp_map.insert(port_info.port, port_info.service_name);
    }
    tcp_map
}

fn get_tcp_service() -> Result<HashMap<u16, String>, Box<dyn std::error::Error>> {
    let file_path: PathBuf = TcpService::bin_file_path().unwrap();
    let f  = fs::read(file_path).unwrap();
    let tcp_services: Vec<TcpService> = bincode::deserialize(&f).unwrap();
    let mut tcp_map: HashMap<u16, String> = HashMap::new();
    for port_info in tcp_services {
        tcp_map.insert(port_info.port, port_info.service_name);
    }
    Ok(tcp_map)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceDatabase {
    pub tcp_map: HashMap<u16, String>,
}

impl ServiceDatabase {
    /// Create a new ServiceDatabase with bundled tcp services.
    pub fn new() -> Self {
        ServiceDatabase {
            tcp_map: get_bundled_tcp_service(),
        }
    }
    /// Load ServiceDatabase from the file system (user's config directory).
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        match get_tcp_service() {
            Ok(tcp_map) => {
                Ok(ServiceDatabase {
                    tcp_map,
                })
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                Err(e)
            }
        }
    }
}
