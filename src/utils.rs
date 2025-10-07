use crate::args::{HumidityUnit, PressureUnit, TemperatureUnit};
use crate::sensor::Unit;
use rand::{self, Rng};
use rusqlite;
use std::io::prelude::*;
use time::UtcDateTime;

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

pub fn serialize_unit(unit: &Unit) -> &str {
    let unit_str = match &unit {
        Unit::TemperatureUnit(TemperatureUnit::Celsius) => "temperature_celsius",
        Unit::TemperatureUnit(TemperatureUnit::Kelvin) => "temperature_kelvin",
        Unit::PressureUnit(PressureUnit::Pascal) => "pressure_pascal",
        Unit::PressureUnit(PressureUnit::Bar) => "pressure_bar",
        Unit::HumidityUnit(HumidityUnit::Absolute) => "humidity_absolute",
        Unit::HumidityUnit(HumidityUnit::Relative) => "humidity_relative",
    };

    unit_str
}

pub fn serialize_timestamp(datetime: &UtcDateTime) -> Result<String> {
    let mut w: Vec<u8> = Vec::new();

    write!(
        w,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        datetime.year(),
        datetime.month() as u8,
        datetime.day(),
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
    )?;

    let s = String::from_utf8(w)?;

    Ok(s)
}

pub fn create_id() -> String {
    let chars = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let code: String = (0..3)
        .map(|_| chars[rand::rng().random_range(0..62)] as char)
        .collect();
    code
}
