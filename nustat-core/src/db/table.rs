use std::net::{Ipv4Addr, Ipv6Addr};

use serde::{Serialize, Deserialize};
use rusqlite::{params, Connection, Result, Statement, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WatchList {
    pub id: u64,
    pub name: String,
    pub ip_addr: String,
    pub hostname: String,
    pub port: u16,
    pub protocol: String,
    pub updated_at: String,
}

impl WatchList {
    pub fn new() -> Self {
        WatchList {
            id: 0,
            name: String::new(),
            ip_addr: String::new(),
            hostname: String::new(),
            port: 0,
            protocol: String::new(),
            updated_at: String::new(),
        }
    }
    pub fn create(tran: &Transaction) -> Result<usize, rusqlite::Error> {
        tran.execute(
            "CREATE TABLE IF NOT EXISTS watch_list (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                ip_addr TEXT NOT NULL,
                hostname TEXT NOT NULL,
                port INTEGER NOT NULL,
                protocol TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(ip_addr, port, protocol)
            )",
            [],
        )
    }
    pub fn insert(self, tran: &Transaction) -> Result<usize, rusqlite::Error> {
        tran.execute(
            "INSERT INTO watch_list (
                name,
                ip_addr,
                hostname,
                port,
                protocol,
                updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                self.name,
                self.ip_addr,
                self.hostname,
                self.port,
                self.protocol,
                self.updated_at
            ],
        )
    }
    pub fn delete(tran: &Transaction, id: u64) -> Result<usize, rusqlite::Error> {
        tran.execute(
            "DELETE FROM watch_list WHERE id = ?1",
            params![id],
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ipv4Info {
    pub ip_from_int: u32,
    pub ip_to_int: u32,
    pub ip_from: String,
    pub ip_to: String,
    pub cidr: String,
    pub country_code: String,
    pub country_name: String,
    pub registry: String,
    pub status: String,
    pub asn: String,
    pub as_name: String,
}

impl Ipv4Info {
    pub fn create(tran:&Transaction) -> Result<usize,rusqlite::Error> {
        tran.execute(
            "CREATE TABLE IF NOT EXISTS ipv4_info (
                ip_from_int INTEGER NOT NULL,
                ip_to_int INTEGER NOT NULL,
                ip_from TEXT NOT NULL,
                ip_to TEXT NOT NULL,
                cidr TEXT NOT NULL,
                country_code TEXT NOT NULL,
                country_name TEXT NOT NULL,
                registry TEXT NOT NULL,
                status TEXT NOT NULL, 
                asn TEXT NOT NULL,
                as_name TEXT NOT NULL,
                PRIMARY KEY(ip_from_int, ip_to_int) 
            )",
            params![],
        )
    }
    pub fn drop(tran:&Transaction) -> Result<usize,rusqlite::Error> {
        tran.execute(
            "DROP TABLE IF EXISTS ipv4_info;",
            params![],
        )
    }
    pub fn truncate(tran:&Transaction) -> Result<usize,rusqlite::Error> {
        tran.execute(
            "DELETE FROM ipv4_info;",
            params![],
        )
    }
    pub fn get_ipv4_info(conn: &Connection, ipv4_addr: Ipv4Addr) -> Option<Ipv4Info> {
        let ip_addr_int = crate::net::ip::ipv4_to_int(ipv4_addr);
        //let conn = super::connect_db(super::IP_DB_NAME).unwrap();
        let sql: &str = "SELECT 
            ip_from_int,
            ip_to_int,
            ip_from,
            ip_to,
            cidr,
            country_code,
            country_name,
            registry,
            status, 
            asn,
            as_name 
        FROM ipv4_info 
        WHERE ?1 BETWEEN ip_from_int AND ip_to_int;";
        let params_vec: &[&dyn rusqlite::ToSql] = params![ip_addr_int];   
        let mut stmt: Statement = conn.prepare(sql).unwrap();
        let result_iter = stmt.query_map(params_vec, |row| {
            Ok(Ipv4Info{
                ip_from_int: row.get(0)?,
                ip_to_int: row.get(1)?,
                ip_from: row.get(2)?,
                ip_to: row.get(3)?,
                cidr: row.get(4)?,
                country_code: row.get(5)?,
                country_name: row.get(6)?,
                registry: row.get(7)?,
                status: row.get(8)?,
                asn: row.get(9)?,
                as_name: row.get(10)?,
            })
        }).unwrap();
    
        match result_iter.last() {
            Some(r) => {
                match r {
                    Ok(r) => {
                        return Some(r);
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        return None;
                    }
                }
            },
            None => {
                return None;
            }
        }
    }
    pub fn table_exists() -> bool {
        let conn = match super::connect_db(super::IP_DB_NAME){
            Ok(c) => c,
            Err(e) => {
                println!("Error: {}", e);
                return false;
            }
        };
        let sql: &str = "SELECT name FROM sqlite_master WHERE type='table' AND name='ipv4_info';";
        let params_vec: &[&dyn rusqlite::ToSql] = params![];   
        let mut stmt: Statement = conn.prepare(sql).unwrap();
        // exec query and check record count is 1.
        let result_iter = stmt.query_map(params_vec, |row| {
            Ok(row.get::<_, String>(0)?)
        }).unwrap();
        match result_iter.last() {
            Some(r) => {
                match r {
                    Ok(_) => {
                        return true;
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        return false;
                    }
                }
            },
            None => {
                return false;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ipv6Info {
    pub ip_from_dec: String,
    pub ip_to_dec: String,
    //pub ip_from_dec: u128,
    //pub ip_to_dec: u128,
    pub ip_from: String,
    pub ip_to: String,
    pub cidr: String,
    pub country_code: String,
    pub country_name: String,
    pub registry: String,
    pub status: String,
    pub asn: String,
    pub as_name: String,
}

impl Ipv6Info {
    pub fn create(tran:&Transaction) -> Result<usize,rusqlite::Error> {
        tran.execute(
            "CREATE TABLE IF NOT EXISTS ipv6_info (
                ip_from_dec TEXT NOT NULL,
                ip_to_dec TEXT NOT NULL,
                ip_from TEXT NOT NULL,
                ip_to TEXT NOT NULL,
                cidr TEXT NOT NULL,
                country_code TEXT NOT NULL,
                country_name TEXT NOT NULL,
                registry TEXT NOT NULL,
                status TEXT NOT NULL, 
                asn TEXT NOT NULL,
                as_name TEXT NOT NULL,
                PRIMARY KEY(ip_from_dec, ip_to_dec) 
            )",
            params![],
        )
    }
    pub fn drop(tran:&Transaction) -> Result<usize,rusqlite::Error> {
        tran.execute("DROP TABLE IF EXISTS ipv6_info;", params![])
    }
    pub fn truncate(tran:&Transaction) -> Result<usize,rusqlite::Error> {
        tran.execute("DELETE FROM ipv6_info;", params![])
    }
    pub fn get_ipv6_info(conn: &Connection, ipv6_addr: Ipv6Addr) -> Option<Ipv6Info> {
        let ip_addr_dec = crate::net::ip::ipv6_to_dec(ipv6_addr);
        //let conn = super::connect_db(super::IP_DB_NAME).unwrap();
        let sql: &str = "SELECT 
            ip_from_dec,
            ip_to_dec,
            ip_from,
            ip_to,
            cidr,
            country_code,
            country_name,
            registry,
            status, 
            asn,
            as_name 
        FROM ipv6_info 
        WHERE ?1 BETWEEN ip_from_dec AND ip_to_dec;";
        let params_vec: &[&dyn rusqlite::ToSql] = params![ip_addr_dec.to_string()];   
        let mut stmt: Statement = conn.prepare(sql).unwrap();
        let result_iter = stmt.query_map(params_vec, |row| {
            Ok(Ipv6Info{
                ip_from_dec: row.get(0)?,
                ip_to_dec: row.get(1)?,
                ip_from: row.get(2)?,
                ip_to: row.get(3)?,
                cidr: row.get(4)?,
                country_code: row.get(5)?,
                country_name: row.get(6)?,
                registry: row.get(7)?,
                status: row.get(8)?,
                asn: row.get(9)?,
                as_name: row.get(10)?,
            })
        }).unwrap();
    
        match result_iter.last() {
            Some(r) => {
                match r {
                    Ok(r) => {
                        return Some(r);
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        return None;
                    }
                }
            },
            None => {
                return None;
            }
        }
    }
    pub fn table_exists() -> bool {
        let conn = match super::connect_db(super::IP_DB_NAME){
            Ok(c) => c,
            Err(e) => {
                println!("Error: {}", e);
                return false;
            }
        };
        let sql: &str = "SELECT name FROM sqlite_master WHERE type='table' AND name='ipv6_info';";
        let params_vec: &[&dyn rusqlite::ToSql] = params![];   
        let mut stmt: Statement = conn.prepare(sql).unwrap();
        // exec query and check record count is 1.
        let result_iter = stmt.query_map(params_vec, |row| {
            Ok(row.get::<_, String>(0)?)
        }).unwrap();
        match result_iter.last() {
            Some(r) => {
                match r {
                    Ok(_) => {
                        return true;
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        return false;
                    }
                }
            },
            None => {
                return false;
            }
        }
    }
}
