use anyhow::Result;
use rusqlite::Connection;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn init() -> Result<DB> {
        let conn = Connection::open("doorsensor.db")?;
        Ok(DB { conn })
    }
}
