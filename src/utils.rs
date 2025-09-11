use rand::{self, Rng};
use rusqlite;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn setup_db() -> Result<rusqlite::Connection> {
    let conn = rusqlite::Connection::open_in_memory()?;

    let _result = conn.execute(
        "create or replace table
            readings
        (
            id varchar,
            timestamp timestamp primary key,
            value float,
            unit varchar,
            symbol varchar,
        )",
        (),
    )?;

    Ok(conn)
}

pub fn create_id() -> String {
    let chars = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let code: String = (0..3)
        .map(|_| chars[rand::rng().random_range(0..62)] as char)
        .collect();
    code
}
