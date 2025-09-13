mod args;
mod sensor;
mod utils;

use crate::args::{Sensor, parse_and_validate};
use crate::sensor::{
    EnvironmentalSensor, build_humidity_sensor, build_pressure_sensor, build_temp_sensor,
};
use std::process;

fn main() {
    let args = parse_and_validate().expect("invalid arguments");

    println!("sensor_type: {:?}", args.sensor_type);
    println!("interval: {:?}", args.timing_args.interval);
    println!("duration: {:?}", args.timing_args.duration);
    println!("number: {:?}", args.timing_args.number);

    let mut sensor: EnvironmentalSensor = match &args.sensor_type {
        Sensor::Temperature { .. } => build_temp_sensor(&args),
        Sensor::Pressure { .. } => build_pressure_sensor(&args),
        Sensor::Humidity { .. } => build_humidity_sensor(&args),
    };

    let interval: i32 = args.timing_args.interval.unwrap().clone() as i32;
    let duration: i32 = args.timing_args.duration.unwrap().clone() as i32;

    match sensor.run_sensor(&interval, &duration) {
        Ok(..) => println!("process complete"),
        Err(e) => {
            println!("an error was encountered: {}", e);
            process::exit(1);
        }
    };
}
