pub mod ip;

pub const DB_NAME: &str = "nustat.db";
pub const IP_DB_NAME: &str = "ip.db";

/* pub fn connect_db(db_name: &str) -> Result<Connection, rusqlite::Error> {
    let mut path: PathBuf = sys::get_config_dir_path().unwrap();
    path.push(db_name);
    /* let mut path: PathBuf = env::current_exe().unwrap();
    path.pop();
    path.push(db_name); */
    let conn = Connection::open(path)?;
    rusqlite::vtab::array::load_module(&conn)?;
    Ok(conn)
} */

/* pub fn init_db() -> Result<usize, rusqlite::Error> {
    let mut affected_row_count: usize = 0;
    let mut conn: Connection = match connect_db(DB_NAME) {
        Ok(c)=> c, 
        Err(e) => return Err(e),
    };
    let tran: Transaction = conn.transaction().unwrap();
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
} */

/* pub fn optimize_db() -> Result<usize, rusqlite::Error> {
    let conn: Connection = match connect_db(DB_NAME) {
        Ok(c)=> c, 
        Err(e) => return Err(e),
    };
    conn.execute("VACUUM;", params![])
} */