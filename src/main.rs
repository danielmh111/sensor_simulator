use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// type of sensor - dictates the type of data generated
    #[arg(short, long)]
    sensor_type: String,

    /// interval at which data is generated in seconds
    #[arg(short, long, default_value_t = 60)]
    interval: u16,

    /// duration for which readings are generated in seconds
    #[arg(short, long, default_value_t = 300)]
    duration: u32,

    /// number of readings that are generated. Should only be used with either interval or duration, but not both.
    #[arg(short, long)]
    number: u16,

    /// the symbol that should indicate the units of the data
    #[arg(short, long)]
    unit: String,
}

fn main() {
    let args = Args::parse();

    println!("sensor_type: {}", args.sensor_type);
    println!("interval: {}", args.interval);
    println!("duration: {}", args.duration);
    println!("number: {}", args.number);
    println!("unit: {}", args.unit);
}
