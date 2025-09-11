use crate::args::{Args, FileFormat, HumidityUnit, PressureUnit, Sensor, TemperatureUnit};
use crate::utils::{create_id, setup_db};
use csv;
use rand::{self, Rng};
use rand_distr::{Distribution, Normal};
use serde::Serialize;
use serde_json;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use time::UtcDateTime;

const MAX_BATCHES_PER_FILE: usize = 10;
const APPEND_BATCH_SIZE: usize = 250;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
pub struct EnvironmentalSensor {
    category: SensorType,
    id: String,
    random_seed: u16,
    outputs: Vec<SensorOutput>,
    unit: Unit,
    unit_symbol: &'static str,
    base_value: f64,
    drift_std: f64,
    file_path: Option<String>,
    file_format: FileFormat,
    current_file_partition: usize,
    batches_in_current_file: usize,
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
    pub fn run_sensor(&mut self, interval: &i32, duration: &i32) -> Result<()> {
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

            if self.outputs.len() % APPEND_BATCH_SIZE == 0 {
                self.log_data()?;
            }
        }

        match self.file_path {
            Some(..) => self.write_all_to_file()?,
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
        let result = match self.file_format {
            FileFormat::CSV => self.write_all_to_csv(),
            FileFormat::Json => self.write_all_to_json(),
        };

        result
    }
    fn write_all_to_csv(&self) -> Result<()> {
        let path = std::path::Path::new(self.file_path.as_ref().unwrap()).join("output.csv");
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
        let path = std::path::Path::new(self.file_path.as_ref().unwrap()).join("output.json");

        let mut file = File::create(path)?;

        let outputs_json = serde_json::to_string(&self.outputs)?;

        file.write_all(outputs_json.as_bytes())?;

        Ok(())
    }
    fn append_to_file(&mut self) -> Result<()> {
        let mut filename: String = self.id.clone();
        filename.push_str("_output_");
        filename.push_str(&self.current_file_partition.to_string().clone());
        filename.push_str(".csv");

        let path = std::path::Path::new(self.file_path.as_ref().unwrap()).join(filename);

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)?;

        let mut writer: csv::Writer<std::fs::File> = if file.metadata()?.len() > 0 {
            csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(file)
        } else {
            csv::Writer::from_writer(file)
        };

        for reading in &self.outputs {
            writer.serialize(reading)?;
        }

        writer.flush()?;

        Ok(())
    }
    fn log_data(&mut self) -> Result<()> {
        // add logic for rotating files when they reach max size here

        if self.batches_in_current_file == MAX_BATCHES_PER_FILE {
            self.current_file_partition += 1;
            self.batches_in_current_file = 0;
        }

        for attempt in 0..5 {
            match self.flush_outputs() {
                Ok(..) => return Ok(()),
                Err(_) if attempt < 5 => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
    fn flush_outputs(&mut self) -> Result<()> {
        let mut filename: String = self.id.clone();
        filename.push_str("_output_");
        filename.push_str(&self.current_file_partition.to_string().clone());
        filename.push_str(".csv");
        let path: std::path::PathBuf =
            std::path::Path::new(self.file_path.as_ref().unwrap()).join(&filename);

        let mut temp_file: String = filename.clone();
        temp_file.push_str("temp");
        let temp_file_path: std::path::PathBuf =
            std::path::Path::new(self.file_path.as_ref().unwrap()).join(&temp_file);

        if path.exists() {
            std::fs::copy(&path, &temp_file_path)?;
        }

        let result = self.append_to_file();

        match result {
            Ok(..) => {
                self.batches_in_current_file += 1;
                self.outputs.clear();
                _ = std::fs::remove_file(temp_file_path);
                // im letting the error be ignored here if it occurs bc the transaction is already successful and i dont want the transaction to return error once the file is written and the vector is clear - its complete.
                // once there are partitioned logs, if this is failing often then there could be an accumulation of temp files. cant be bothered to handle that any time soon
            }
            Err(e) => {
                std::fs::copy(&temp_file_path, &path)?;
                return Err(e);
            }
        }

        Ok(())
    }
}

pub fn build_temp_sensor(args: &Args) -> EnvironmentalSensor {
    let file_path: Option<String> = if args.output_args.to_file == "false".to_string() {
        None
    } else {
        Some(args.output_args.to_file.clone())
    };

    let mut id = "TMP".to_string();
    id.push_str(&create_id());

    let temperature_sensor: EnvironmentalSensor = EnvironmentalSensor {
        category: SensorType::Temperature("temperature".to_string()),
        id: id,
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
        file_format: args.output_args.format,
        current_file_partition: 0,
        batches_in_current_file: 0,
    };

    temperature_sensor
}

pub fn build_pressure_sensor(args: &Args) -> EnvironmentalSensor {
    let file_path: Option<String> = if args.output_args.to_file == "false".to_string() {
        None
    } else {
        Some(args.output_args.to_file.clone())
    };

    let mut id = "PRS".to_string();
    id.push_str(&create_id());

    let pressure_sensor: EnvironmentalSensor = EnvironmentalSensor {
        category: SensorType::Pressure("pressure".to_string()),
        id: id,
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
        file_format: args.output_args.format,
        current_file_partition: 0,
        batches_in_current_file: 0,
    };

    pressure_sensor
}

pub fn build_humidity_sensor(args: &Args) -> EnvironmentalSensor {
    let file_path: Option<String> = if args.output_args.to_file == "false".to_string() {
        None
    } else {
        Some(args.output_args.to_file.clone())
    };

    let mut id = "HMD".to_string();
    id.push_str(&create_id());

    let humidity_sensor: EnvironmentalSensor = EnvironmentalSensor {
        category: SensorType::Humidity("humidity".to_string()),
        id: id,
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
        file_format: args.output_args.format,
        current_file_partition: 0,
        batches_in_current_file: 0,
    };

    humidity_sensor
}
