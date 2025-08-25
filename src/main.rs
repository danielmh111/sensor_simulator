use clap::{Parser, Subcommand, ValueEnum};
use std::io::Error;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// type of sensor - dictates the type of data generated
    #[clap(subcommand)]
    sensor_type: Sensor,

    #[clap(flatten)]
    timing_args: TimingArgs,
}

#[derive(Parser, Debug)]
struct TimingArgs {
    /// interval at which data is generated in seconds
    #[arg(short, long)]
    interval: Option<u16>,

    /// duration for which readings are generated in seconds
    #[arg(short, long)]
    duration: Option<u32>,

    /// number of readings that are generated. Should only be used with either interval or duration, but not both.
    #[arg(short, long)]
    number: Option<u16>,
}

impl TimingArgs {
    // only two out of interval, duration, and number should be specified
    // the tird one should be inferred from the two provided
    // it should be allowed to provide only one, and a sensible default should be set for the others.
    fn validate(&mut self) -> Result<(), &str> {
        let provided_interval: bool = self.interval.is_some();
        let provided_duration: bool = self.duration.is_some();
        let provided_number: bool = self.number.is_some();
        let provided_args: Vec<bool> = vec![provided_interval, provided_duration, provided_number];
        let total_provided: usize = provided_args.iter().filter(|&&i| i).count();

        if total_provided == 3 {
            return Err(
                "only provide two options out of interval, duration, and number. The third value will be fixed by the first two.",
            );
        } else if total_provided == 2 {
            return Ok(());
        } else if total_provided == 1 {
            if provided_interval {
                self.duration = Some(300);
            } else if provided_duration {
                self.interval = Some(60);
            } else if provided_number {
                self.interval = Some(60);
            } else {
                panic!()
            };
            return Ok(());
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Sensor {
    Temperature {
        /// unit in which data is generated
        #[arg(short, long)]
        unit: TemperatureUnit,
    },
    Pressure {
        /// unit in which data is generated
        #[arg(short, long)]
        unit: PressureUnit,
    },
    Humidity {
        /// unit in which data is generated
        #[arg(short, long)]
        unit: HumidityUnit,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TemperatureUnit {
    Celsius,
    Kelvin,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PressureUnit {
    Pascal,
    Bar,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum HumidityUnit {
    Absolute,
    Relative,
}

fn main() {
    let args = Args::parse();

    println!("sensor_type: {:?}", args.sensor_type);
    println!("interval: {:?}", args.timing_args.interval);
    println!("duration: {:?}", args.timing_args.duration);
    println!("number: {:?}", args.timing_args.number);
}
