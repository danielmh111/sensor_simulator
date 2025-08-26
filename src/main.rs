mod args;

// use std::hash::RandomState;

use crate::args::{Args, HumidityUnit, PressureUnit, Sensor, TemperatureUnit, parse_and_validate};
use time::Time;

struct SensorOutput {
    id: String,
    timestamp: Time,
    value: f32,
    unit: Unit,
}

pub enum Unit {
    TemperatureUnit(TemperatureUnit),
    PressureUnit(PressureUnit),
    HumidityUnit(HumidityUnit),
}

pub struct SensorID {}

enum SensorType {
    Temperature(String),
    Pressure(String),
    Humidity(String),
}

struct EnvironmentalSensor {
    category: SensorType,
    id: String,
    random_seed: u16,
    outputs: Vec<SensorOutput>,
    unit: Unit,
    unit_symbol: &'static str,
}

fn build_temp_sensor(args: Args) -> EnvironmentalSensor {
    let temperature_sensor: EnvironmentalSensor = EnvironmentalSensor {
        category: SensorType::Temperature("temperature".to_string()),
        id: "1".to_string(),
        random_seed: 42,
        outputs: vec![],
        unit: match &args.sensor_type {
            Sensor::Temperature { unit } => Unit::TemperatureUnit(unit.clone()),
            _ => panic!("shouldn't be constructing a temp sensor with a pressure or humidity unit"),
        },
        unit_symbol: match &args.sensor_type {
            Sensor::Temperature {
                unit: TemperatureUnit::Celsius,
            } => "C",
            Sensor::Temperature {
                unit: TemperatureUnit::Kelvin,
            } => "K",
            _ => panic!("shouldn't be constructing a temp sensor with a pressure or humidity unit"),
        },
    };

    temperature_sensor
}

fn build_pressure_sensor(args: Args) -> EnvironmentalSensor {
    let pressure_sensor: EnvironmentalSensor = EnvironmentalSensor {
        category: SensorType::Pressure("pressure".to_string()),
        id: "1".to_string(),
        random_seed: 42,
        outputs: vec![],
        unit: match &args.sensor_type {
            Sensor::Pressure { unit } => Unit::PressureUnit(unit.clone()),
            _ => panic!("shouldn't be constructing a pressure sensor with a temp or humidity unit"),
        },
        unit_symbol: match &args.sensor_type {
            Sensor::Pressure {
                unit: PressureUnit::Bar,
            } => "bar",
            Sensor::Pressure {
                unit: PressureUnit::Pascal,
            } => "Pa",
            _ => panic!("shouldn't be constructing a pressure sensor with a temp or humidity unit"),
        },
    };

    pressure_sensor
}

fn build_humidity_sensor(args: Args) -> EnvironmentalSensor {
    let humidity_sensor: EnvironmentalSensor = EnvironmentalSensor {
        category: SensorType::Humidity("humidity".to_string()),
        id: "1".to_string(),
        random_seed: 42,
        outputs: vec![],
        unit: match &args.sensor_type {
            Sensor::Humidity { unit } => Unit::HumidityUnit(unit.clone()),
            _ => panic!("shouldn't be constructing a humidity sensor with a pressure or temp unit"),
        },
        unit_symbol: match &args.sensor_type {
            Sensor::Humidity {
                unit: HumidityUnit::Absolute,
            } => "g/m^3",
            Sensor::Humidity {
                unit: HumidityUnit::Relative,
            } => "%",
            _ => panic!("shouldn't be constructing a humidity sensor with a pressure or temp unit"),
        },
    };

    humidity_sensor
}

fn main() {
    let args = parse_and_validate().expect("invalid arguments");

    println!("sensor_type: {:?}", args.sensor_type);
    println!("interval: {:?}", args.timing_args.interval);
    println!("duration: {:?}", args.timing_args.duration);
    println!("number: {:?}", args.timing_args.number);

    let sensor: EnvironmentalSensor = match args.sensor_type {
        Sensor::Temperature { .. } => build_temp_sensor(args),
        Sensor::Pressure { .. } => build_pressure_sensor(args),
        Sensor::Humidity { .. } => build_humidity_sensor(args),
    };
}
