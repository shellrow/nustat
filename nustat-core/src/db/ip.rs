use std::{collections::HashMap, fs, net::{Ipv4Addr, Ipv6Addr}, path::PathBuf};
use rangemap::RangeInclusiveMap;
use serde::{Serialize, Deserialize};

use crate::ipinfo::{Ipv4Info, Ipv6Info};

pub use nustat_db_ipv4::Ipv4Info as DbIpv4Info;
pub use nustat_db_ipv4::db::IPV4_INFO_BIN_NAME;
use nustat_db_ipv4::db::IPV4_INFO_BIN;
pub use nustat_db_ipv6::Ipv6Info as DbIpv6Info;
pub use nustat_db_ipv6::db::IPV6_INFO_BIN_NAME;
use nustat_db_ipv6::db::IPV6_INFO_BIN;
pub use nustat_db_country::Country;
pub use nustat_db_country::db::COUNTRY_BIN_NAME;
use nustat_db_country::db::COUNTRY_BIN;
pub use nustat_db_as::AutonomousSystem;
pub use nustat_db_as::db::AS_BIN_NAME;
use nustat_db_as::db::AS_BIN;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AsnCountry {
    pub asn: u32,
    pub country_code: String,
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
    pub fn check_db_files() -> Result<(), Box<dyn std::error::Error>> {
        let ipv4_file_path: PathBuf = DbIpv4Info::bin_file_path().unwrap();
        if !ipv4_file_path.exists() {
            return Err("ipv4.bin file not found".into());
        }
        let ipv6_file_path: PathBuf = DbIpv6Info::bin_file_path().unwrap();
        if !ipv6_file_path.exists() {
            return Err("ipv6.bin file not found".into());
        }
        let country_file_path: PathBuf = Country::bin_file_path().unwrap();
        if !country_file_path.exists() {
            return Err("country.bin file not found".into());
        }
        let as_file_path: PathBuf = AutonomousSystem::bin_file_path().unwrap();
        if !as_file_path.exists() {
            return Err("as.bin file not found".into());
        }
        Ok(())
    }
    pub fn load() -> Result<IpDatabase, Box<dyn std::error::Error>> {
        let mut ip_db = IpDatabase::new();
        ip_db.load_ipv4()?;
        ip_db.load_ipv6()?;
        ip_db.load_country()?;
        ip_db.load_autonomous()?;
        Ok(ip_db)
    }
    pub fn load_from_crate() -> Result<IpDatabase, Box<dyn std::error::Error>> {
        let mut ip_db = IpDatabase::new();
        ip_db.load_ipv4_from_crate()?;
        ip_db.load_ipv6_from_crate()?;
        ip_db.load_country_from_crate()?;
        ip_db.load_autonomous_from_crate()?;
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
    pub fn load_ipv4_from_crate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let ipv4_info_vec: Vec<DbIpv4Info> = bincode::deserialize(IPV4_INFO_BIN).unwrap();
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
    pub fn load_ipv6_from_crate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let ipv6_info_vec: Vec<DbIpv6Info> = bincode::deserialize(IPV6_INFO_BIN).unwrap();
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
    pub fn load_country_from_crate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let country_vec: Vec<Country> = bincode::deserialize(COUNTRY_BIN).unwrap();
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
    pub fn load_autonomous_from_crate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let autonomous_vec: Vec<AutonomousSystem> = bincode::deserialize(AS_BIN).unwrap();
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
