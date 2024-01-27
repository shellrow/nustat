pub mod table;
//pub mod stat;
//use std::env;
use std::path::PathBuf;
use rusqlite::{Connection, Result, Transaction, params};

use crate::sys;

use self::table::{Ipv4Info, Ipv6Info};

pub const DB_NAME: &str = "nustat.db";
pub const IP_DB_NAME: &str = "ip.db";

pub fn connect_db(db_name: &str) -> Result<Connection, rusqlite::Error> {
    let mut path: PathBuf = sys::get_config_dir_path().unwrap();
    path.push(db_name);
    /* let mut path: PathBuf = env::current_exe().unwrap();
    path.pop();
    path.push(db_name); */
    
    /* if !path.exists() {
        copy_db();
    } */
    let conn = Connection::open(path)?;
    rusqlite::vtab::array::load_module(&conn)?;
    Ok(conn)
}

pub fn init_db() -> Result<usize, rusqlite::Error> {
    let mut affected_row_count: usize = 0;
    let mut conn: Connection = match connect_db(DB_NAME) {
        Ok(c)=> c, 
        Err(e) => return Err(e),
    };
    let tran: Transaction = conn.transaction().unwrap();
    // packet_frame
    /* match table::DbPacketFrame::create(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    }
    match table::DbPacketFrame::truncate(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    }
    // remote_host
    match table::DbRemoteHost::create(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    }
    // process_info
    match table::DbProcessInfo::create(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    }
    // socket_info
    match table::DbSocketInfo::create(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    }
    // user_info
    match table::DbUserInfo::create(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    }
    // user_group
    match table::DbUserGroup::create(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    }
    // group_info
    match table::DbGroupInfo::create(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    } */
    // watch_list
    match table::WatchList::create(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    }
    match tran.commit() {
        Ok(_) => {
            return Ok(affected_row_count);
        },
        Err(e) => {
            return Err(e);
        }
    } 
}

// check if ip.db exists in config directory. 
// if exists, check if ip.db has ipv4_info and ipv6_info tables.
pub fn ip_db_exists() -> bool {
    let mut path: PathBuf = sys::get_config_dir_path().unwrap();
    path.push(IP_DB_NAME);
    if path.exists() {
        if Ipv4Info::table_exists() && Ipv6Info::table_exists() {
            return true;
        } else {
            return false;
        }
    }
    false
}

pub fn cleanup_db() -> Result<usize, rusqlite::Error> {
    /* let mut affected_row_count: usize = 0;
    let mut conn: Connection = match connect_db(DB_NAME) {
        Ok(c)=> c, 
        Err(e) => return Err(e),
    };
    let tran: Transaction = conn.transaction().unwrap();
    // packet_frame
    match table::DbPacketFrame::truncate(&tran) {
        Ok(count) => {
            affected_row_count += count;
        },
        Err(e) => {
            return Err(e);
        }
    }
    match tran.commit() {
        Ok(_) => {
            return Ok(affected_row_count);
        },
        Err(e) => {
            return Err(e);
        }
    }  */
    Ok(0)
}

// EXEC VACUUM;
pub fn optimize_db() -> Result<usize, rusqlite::Error> {
    let conn: Connection = match connect_db(DB_NAME) {
        Ok(c)=> c, 
        Err(e) => return Err(e),
    };
    conn.execute("VACUUM;", params![])
}
