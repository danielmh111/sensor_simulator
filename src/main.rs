mod args;

use std::hash::RandomState;

use crate::args::{Args, HumidityUnit, PressureUnit, Sensor, TemperatureUnit, parse_and_validate};
use time::Time;

struct SensorOutput {
    id: String,
    timestamp: Time,
    value: f32,
    unit: Unit,
}

pub enum Unit {
    TemperatureUnit,
    PressureUnit,
    HumidityUnit,
}

pub struct SensorID {}

enum EnvironmentalSensor {
    TemperatureSensor(TemperatureSensor),
    PressureSensor(PressureSensor),
    HumiditySensor(HumiditySensor),
}

struct TemperatureSensor {
    category: String,
    id: String,
    random_seed: u16,
    outputs: Vec<SensorOutput>,
    unit: TemperatureUnit,
    unit_symbol: char,
}

struct PressureSensor {
    category: String,
    id: String,
    random_seed: u16,
    outputs: Vec<SensorOutput>,
    unit: PressureUnit,
    unit_symbol: char,
}

struct HumiditySensor {
    category: String,
    id: String,
    random_seed: u16,
    outputs: Vec<SensorOutput>,
    unit: HumidityUnit,
    unit_symbol: char,
}

fn build_temp_sensor(args: Args) -> TemperatureSensor {
    let temperature_sensor: TemperatureSensor = TemperatureSensor {
        category: "temperature".to_string(),
        id: "1".to_string(),
        random_seed: 42,
        outputs: vec![],
        unit: match &args.sensor_type {
            Sensor::Temperature { unit } => unit.clone(),
            _ => panic!("shouldn't be constructing a temp sensor with a pressure or humidity unit"),
        },
        unit_symbol: match &args.sensor_type {
            Sensor::Temperature {
                unit: TemperatureUnit::Celsius,
            } => 'C',
            Sensor::Temperature {
                unit: TemperatureUnit::Kelvin,
            } => 'K',
            _ => panic!("shouldn't be constructing a temp sensor with a pressure or humidity unit"),
        },
    };

    temperature_sensor
}

fn build_pressure_sensor(_args: Args) -> PressureSensor {
    panic!("not implemented")
}

fn build_humidity_sensor(_args: Args) -> HumiditySensor {
    panic!("not implemented")
}

fn main() {
    let args = parse_and_validate().expect("invalid arguments");

    println!("sensor_type: {:?}", args.sensor_type);
    println!("interval: {:?}", args.timing_args.interval);
    println!("duration: {:?}", args.timing_args.duration);
    println!("number: {:?}", args.timing_args.number);

    let sensor = match args.sensor_type {
        Sensor::Temperature { unit } => build_temp_sensor(args),
        Sensor::Pressure { unit } => build_pressure_sensor(args),
        Sensor::Humidity { unit } => build_humidity_sensor(args),
        _ => panic!("only temperature, pressure, and humidity sensors have been implemented"),
    };
}
