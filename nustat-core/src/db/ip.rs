use std::{collections::HashMap, fs, net::{Ipv4Addr, Ipv6Addr}, path::PathBuf};

use rangemap::RangeInclusiveMap;
use serde::{Serialize, Deserialize};

use crate::sys;

pub const IPV4_INFO_BIN_NAME: &str = "ipv4_info.bin";
pub const IPV6_INFO_BIN_NAME: &str = "ipv6_info.bin";
pub const COUNTRY_BIN_NAME: &str = "country.bin";
pub const AS_BIN_NAME: &str = "as.bin";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ipv4Info {
    pub ip_addr: Ipv4Addr,
    pub country_code: String,
    pub country_name: String,
    pub asn: u32,
    pub as_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ipv6Info {
    pub ip_addr: Ipv6Addr,
    pub country_code: String,
    pub country_name: String,
    pub asn: u32,
    pub as_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DbIpv4Info {
    pub ip_from: u32,
    pub ip_to: u32,
    pub country_code: String,
    pub asn: u32,
}

impl DbIpv4Info {
    pub fn bin_file_path() -> Option<PathBuf> {
        match sys::get_config_dir_path() {
            Some(mut path) => {
                path.push(IPV4_INFO_BIN_NAME);
                Some(path)
            }
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DbIpv6Info {
    pub ip_from: u128,
    pub ip_to: u128,
    pub country_code: String,
    pub asn: u32,
}

impl DbIpv6Info {
    pub fn bin_file_path() -> Option<PathBuf> {
        match sys::get_config_dir_path() {
            Some(mut path) => {
                path.push(IPV6_INFO_BIN_NAME);
                Some(path)
            }
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Country {
    pub country_code: String,
    pub country_name: String,
}

impl Country {
    pub fn bin_file_path() -> Option<PathBuf> {
        match sys::get_config_dir_path() {
            Some(mut path) => {
                path.push(COUNTRY_BIN_NAME);
                Some(path)
            }
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AsnCountry {
    pub asn: u32,
    pub country_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AutonomousSystem {
    pub asn: u32,
    pub as_name: String,
}

impl AutonomousSystem {
    pub fn bin_file_path() -> Option<PathBuf> {
        match sys::get_config_dir_path() {
            Some(mut path) => {
                path.push(AS_BIN_NAME);
                Some(path)
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpDatabase {
    pub ipv4_map: RangeInclusiveMap<u32, AsnCountry>,
    pub ipv6_map: RangeInclusiveMap<u128, AsnCountry>,
    pub country_map: HashMap<String, String>,
    pub autonomous_map: HashMap<u32, String>,
}

impl IpDatabase {
    pub fn new() -> IpDatabase {
        IpDatabase {
            ipv4_map: RangeInclusiveMap::new(),
            ipv6_map: RangeInclusiveMap::new(),
            country_map: HashMap::new(),
            autonomous_map: HashMap::new(),
        }
    }
    pub fn load() -> Result<IpDatabase, Box<dyn std::error::Error>> {
        let mut ip_db = IpDatabase::new();
        ip_db.load_ipv4()?;
        ip_db.load_ipv6()?;
        ip_db.load_country()?;
        ip_db.load_autonomous()?;
        Ok(ip_db)
    }
    pub fn load_ipv4(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path: PathBuf = DbIpv4Info::bin_file_path().unwrap();
        let f  = fs::read(file_path).unwrap();
        let ipv4_info_vec: Vec<DbIpv4Info> = bincode::deserialize(&f).unwrap();
        for ipv4_info in ipv4_info_vec {
            let asn_country = AsnCountry {
                asn: ipv4_info.asn,
                country_code: ipv4_info.country_code,
            };
            self.ipv4_map.insert(ipv4_info.ip_from..=ipv4_info.ip_to, asn_country);
        }
        Ok(())
    }
    pub fn load_ipv6(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path: PathBuf = DbIpv6Info::bin_file_path().unwrap();
        let f  = fs::read(file_path).unwrap();
        let ipv6_info_vec: Vec<DbIpv6Info> = bincode::deserialize(&f).unwrap();
        for ipv6_info in ipv6_info_vec {
            let asn_country = AsnCountry {
                asn: ipv6_info.asn,
                country_code: ipv6_info.country_code,
            };
            self.ipv6_map.insert(ipv6_info.ip_from..=ipv6_info.ip_to, asn_country);
        }
        Ok(())
    }
    pub fn load_country(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path: PathBuf = Country::bin_file_path().unwrap();
        let f  = fs::read(file_path).unwrap();
        let country_vec: Vec<Country> = bincode::deserialize(&f).unwrap();
        for country in country_vec {
            self.country_map.insert(country.country_code, country.country_name);
        }
        Ok(())
    }
    pub fn load_autonomous(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path: PathBuf = AutonomousSystem::bin_file_path().unwrap();
        let f  = fs::read(file_path).unwrap();
        let autonomous_vec: Vec<AutonomousSystem> = bincode::deserialize(&f).unwrap();
        for autonomous in autonomous_vec {
            self.autonomous_map.insert(autonomous.asn, autonomous.as_name);
        }
        Ok(())
    }
    pub fn get_ipv4_info(&self, ip_addr: Ipv4Addr) -> Option<Ipv4Info> {
        match self.ipv4_map.get(&(ip_addr.into())) {
            Some(ipv4_info) => {
                Some(Ipv4Info {
                    ip_addr: ip_addr,
                    country_code: ipv4_info.country_code.clone(),
                    country_name: self.country_map.get(&ipv4_info.country_code).unwrap().clone(),
                    asn: ipv4_info.asn,
                    as_name: self.autonomous_map.get(&ipv4_info.asn).unwrap().clone(),
                })
            }
            None => None,
        }
    }
    pub fn get_ipv6_info(&self, ip_addr: Ipv6Addr) -> Option<Ipv6Info> {
        match self.ipv6_map.get(&(ip_addr.into())) {
            Some(ipv6_info) => {
                Some(Ipv6Info {
                    ip_addr: ip_addr,
                    country_code: ipv6_info.country_code.clone(),
                    country_name: self.country_map.get(&ipv6_info.country_code).unwrap().clone(),
                    asn: ipv6_info.asn,
                    as_name: self.autonomous_map.get(&ipv6_info.asn).unwrap().clone(),
                })
            }
            None => None,
        }
    }
}
