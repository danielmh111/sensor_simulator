use clap::{Parser, Subcommand, ValueEnum};

/// a command line tool for simulating data from environmental sensors
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
    duration: Option<u16>,

    /// number of readings that are generated. Should only be used with either interval or duration, but not both.
    #[arg(short, long)]
    number: Option<u16>,
}

impl TimingArgs {
    // only two out of interval, duration, and number should be specified
    // the tird one should be inferred from the two provided
    // it should be allowed to provide only one, and a sensible default should be set for the others.
    fn validate(&mut self) -> Result<(), &str> {
        let provided_args = vec![self.interval, self.duration, self.number];
        let provided_args: Vec<usize> = provided_args
            .iter()
            .enumerate()
            .filter(|(_i, arg)| arg.is_some())
            .map(|(i, _arg)| i)
            .collect();

        match provided_args.as_slice() {
            [] => {
                return Err(
                    "Did not provide any arguments to control the timing of data generated. Must provide at least one of: interval, duration, number.",
                );
            }
            [0] => {
                self.duration = Some(300); // set a sensible default - 5mins
                return Ok(());
            }
            [1] => {
                self.interval = Some(60); // set a sensible default - 1min 
                return Ok(());
            }
            [2] => {
                self.duration = Some(300); // set a sensible default - 5mins
                return Ok(());
            }
            [0, 1] => return Ok(()),
            [1, 2] => return Ok(()),
            [0, 2] => return Ok(()),
            [0, 1, 2] => {
                if self.number.unwrap() * self.interval.unwrap() == self.duration.unwrap() {
                    return Ok(());
                } else {
                    return Err(
                        "The provided timing arguments are not compatible together. It is recommended to only provide options out of interval, duration, and number. The third value will be fixed by the first two ",
                    );
                }
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Sensor {
    /// Simulate a temperature sensor
    Temperature {
        /// unit in which data is generated. Choose `Celsius` or `Kelvin`
        #[arg(short, long, ignore_case = true)]
        unit: TemperatureUnit,
    },
    /// Simulate a pressure sensor
    Pressure {
        /// unit in which data is generated. Choose `bar` or `Pascal`
        #[arg(short, long, ignore_case = true)]
        unit: PressureUnit,
    },
    /// Simulate a humidity sensor
    Humidity {
        /// unit in which data is generated. Choose `relative` or `absolute`
        #[arg(short, long, ignore_case = true)]
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
    let mut args = Args::parse();

    match args.timing_args.validate() {
        Ok(v) => v,
        Err(_) => panic!(
            "the timing arguments (interval, duration, and number) are not valid. Provide two valid options"
        ),
    }

    println!("sensor_type: {:?}", args.sensor_type);
    println!("interval: {:?}", args.timing_args.interval);
    println!("duration: {:?}", args.timing_args.duration);
    println!("number: {:?}", args.timing_args.number);
}
