use clap::{Parser, Subcommand, ValueEnum};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// type of sensor - dictates the type of data generated
    #[clap(subcommand)]
    sensor_type: Sensor,

    /// interval at which data is generated in seconds
    #[arg(short, long, default_value_t = 60)]
    interval: u16,

    /// duration for which readings are generated in seconds
    #[arg(short, long, default_value_t = 300)]
    duration: u32,

    /// number of readings that are generated. Should only be used with either interval or duration, but not both.
    #[arg(short, long)]
    number: u16,
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
    println!("interval: {}", args.interval);
    println!("duration: {}", args.duration);
    println!("number: {}", args.number);
}
