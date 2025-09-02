mod args;

use crate::args::{Args, HumidityUnit, PressureUnit, Sensor, TemperatureUnit, parse_and_validate};
use rand::{self, Rng};
use rand_distr::{Distribution, Normal};
use serde::Serialize;
use serde_json;
use std::fmt;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::process;
use time::UtcDateTime;

#[derive(Debug, Serialize)]
struct SensorOutput {
    id: String,
    #[serde(serialize_with = "serialize_datetime")]
    timestamp: UtcDateTime,
    value: f32,
    unit: Unit,
    symbol: String,
}

impl fmt::Display for SensorOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:02}:{:02}:{:02}] Sensor {}: {:.2}{}",
            self.timestamp.hour(),
            self.timestamp.minute(),
            self.timestamp.second(),
            self.id,
            self.value,
            self.symbol
        )
    }
}

fn serialize_datetime<S>(
    datetime: &UtcDateTime,
    serializer: S,
) -> core::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut w: Vec<u8> = Vec::new();

    match write!(
        w,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        datetime.year(),
        datetime.month() as u8,
        datetime.day(),
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
    ) {
        Ok(_) => {
            let s = String::from_utf8(w).unwrap();
            serializer.serialize_str(&s)
        }
        Err(e) => {
            println!("error writing datetime to string: {}", e);
            Err(serde::ser::Error::custom(e))
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum Unit {
    TemperatureUnit(TemperatureUnit),
    PressureUnit(PressureUnit),
    HumidityUnit(HumidityUnit),
}

#[derive(Debug, Serialize)]
pub struct SensorID {}

#[derive(Debug, Serialize)]
enum SensorType {
    Temperature(String),
    Pressure(String),
    Humidity(String),
}

#[derive(Debug)]
struct EnvironmentalSensor {
    category: SensorType,
    id: String,
    random_seed: u16,
    outputs: Vec<SensorOutput>,
    unit: Unit,
    unit_symbol: &'static str,
    base_value: f64,
    drift_std: f64,
    file_path: Option<String>,
}

impl EnvironmentalSensor {
    fn generate_output(&mut self) {
        let timestamp: UtcDateTime = time::UtcDateTime::now();
        // let mean = self.base_value.clone();
        let mean: f64 = 0.0;
        let std: f64 = self.drift_std.clone();

        let change: f32 = Normal::new(mean, std).unwrap().sample(&mut rand::rng()) as f32;
        let value: f32 = match self.outputs.last() {
            Some(v) => v.value + change,
            None => self.base_value as f32 + change,
        };

        let output: SensorOutput = SensorOutput {
            id: (self.id.clone()),
            timestamp: (timestamp),
            value: (value),
            unit: (self.unit.clone()),
            symbol: (self.unit_symbol.to_string()),
        };

        self.outputs.push(output);
    }
    fn run_sensor(&mut self, interval: &i32, duration: &i32) -> Result<()> {
        let duration: i64 = *duration as i64;
        let interval: i64 = *interval as i64;

        let mut duration = time::Duration::new(duration, 0);

        while duration.as_seconds_f32() > 0.0 {
            // might need to checkpoint the timestamp here and use time since this point for the interval to avoid adding time taken for the loop to run being added to the interval time - relevent for very short intervals and very long runs (will add up if set to run for a week)

            self.generate_output();

            // call a function here that formats and prints the output - implement repr on SensorOutput?
            self.read_out();

            duration = duration - time::Duration::new(interval, 0);

            // wait for the interval
            std::thread::sleep(std::time::Duration::new(interval as u64, 0));
        }

        match self.file_path {
            Some(..) => {
                self.write_all_to_file()?;
            }
            None => (),
        }
        Ok(())
    }
    fn read_out(&self) {
        let mut outputs_copy: Vec<&SensorOutput> = Vec::from_iter(self.outputs.iter().clone());
        let most_recent_reading: Option<&SensorOutput> = outputs_copy.pop();
        println!("{}", most_recent_reading.unwrap())
    }
    fn write_all_to_file(&self) -> Result<()> {
        let mut path: String = self.file_path.clone().unwrap();
        path.push_str("\\output.csv");
        let mut writer: csv::Writer<std::fs::File> = csv::Writer::from_path(path)?;

        for reading in &self.outputs {
            match writer.serialize(reading) {
                Ok(..) => (),
                Err(e) => println!("error during csv serialization: {}", e),
            };
        }

        writer.flush()?;

        Ok(())
    }
    fn write_all_to_json(&self) -> Result<()> {
        let mut path: String = self.file_path.clone().unwrap();
        path.push_str("\\output.json");

        let mut file = File::create(path)?;

        let outputs_json = serde_json::to_string(&self.outputs)?;

        file.write_all(outputs_json.as_bytes())?;

        Ok(())
    }
    // fn append_to_file(&mut self) -> Result<()> {
    //     let file_path = match self.file_path {
    //         Some(f) => &f,
    //         None => panic!("append to file has been called but no output destination has been set"),
    //     } as &str;

    //     let mut file = OpenOptions::new()
    //         .create(true)
    //         .write(true)
    //         .append(true)
    //         .open(file_path)?;

    //     file.write_all(contents.as_bytes())
    // }
}

fn build_temp_sensor(args: &Args) -> EnvironmentalSensor {
    let file_path: Option<String> = if args.output_args.to_file == "false".to_string() {
        None
    } else {
        Some(args.output_args.to_file.clone())
    };

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
            } => "°C",
            Sensor::Temperature {
                unit: TemperatureUnit::Kelvin,
            } => "K",
            _ => panic!("shouldn't be constructing a temp sensor with a pressure or humidity unit"),
        },
        base_value: rand::rng().random_range(10.0..30.0),
        drift_std: 0.1,
        file_path: file_path,
    };

    temperature_sensor
}

fn build_pressure_sensor(args: &Args) -> EnvironmentalSensor {
    let file_path: Option<String> = if args.output_args.to_file == "false".to_string() {
        None
    } else {
        Some(args.output_args.to_file.clone())
    };
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
        base_value: rand::rng().random_range(0.9..1.1),
        drift_std: 0.1,
        file_path: file_path,
    };

    pressure_sensor
}

fn build_humidity_sensor(args: &Args) -> EnvironmentalSensor {
    let file_path: Option<String> = if args.output_args.to_file == "false".to_string() {
        None
    } else {
        Some(args.output_args.to_file.clone())
    };
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
        base_value: rand::rng().random_range(40.0..60.0),
        drift_std: 0.3,
        file_path: file_path,
    };

    humidity_sensor
}

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
