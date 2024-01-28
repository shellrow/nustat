use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::sys;

pub const TCP_SERVICE_BIN_NAME: &str = "tcp-service.bin";

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceDatabase {
    pub tcp_map: HashMap<u16, String>,
}

impl ServiceDatabase {
    pub fn new() -> Self {
        ServiceDatabase {
            tcp_map: HashMap::new(),
        }
    }
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut service_db = ServiceDatabase::new();
        service_db.load_tcp_service()?;
        Ok(service_db)
    }
    pub fn load_tcp_service(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path: PathBuf = TcpService::bin_file_path().unwrap();
        let f  = fs::read(file_path).unwrap();
        let tcp_services: Vec<TcpService> = bincode::deserialize(&f).unwrap();
        for tcp_service in tcp_services {
            self.tcp_map.insert(tcp_service.port, tcp_service.service_name);
        }
        Ok(())
    }
}
