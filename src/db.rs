use anyhow::Result;
use rusqlite::Connection;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn init() -> Result<DB> {
        let conn = Connection::open("doorsensor.db")?;

        conn.execute(
            "create table if not exists log (
			id integer primary key,
			is_door_open BOOLEAN not null,
			timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
		)",
            (),
        )?;

        Ok(DB { conn })
    }
}
