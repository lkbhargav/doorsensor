use anyhow::Result;
use rusqlite::Connection;

const DB_NAME: &str = "doorsensor.db";
const TABLE_NAME: &str = "logs";

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn init(db_path: &str) -> Result<DB> {
        let mut path = db_path.to_string();

        if !db_path.ends_with("/") {
            path = format!("{db_path}/");
        }

        let conn = Connection::open(format!("{path}{DB_NAME}"))?;

        conn.execute(
            format!(
                "create table if not exists {TABLE_NAME} (
					id INTEGER primary key,
					is_door_open BOOLEAN not null,
					timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
					notes TEXT
				)"
            )
            .as_str(),
            (),
        )?;

        Ok(DB { conn })
    }

    pub fn log(&mut self, is_door_open: bool) -> Result<()> {
        let _res = &self.conn.execute(
            format!("insert into {TABLE_NAME}(is_door_open) values(?1)").as_str(),
            &[&is_door_open],
        )?;

        Ok(())
    }
}
