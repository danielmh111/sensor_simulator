use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

/// a command line tool for simulating data from environmental sensors
#[derive(Parser, Debug, Clone, Serialize)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// type of sensor - dictates the type of data generated
    #[clap(subcommand)]
    pub sensor_type: Sensor,

    #[clap(flatten)]
    pub timing_args: TimingArgs,

    #[clap(flatten)]
    pub output_args: OutputArgs,
}

#[derive(Parser, Debug, Clone, Serialize)]
pub struct OutputArgs {
    /// whether or not the output should be written to a file. Defaults to false.
    #[arg(
        short = 'f',
        long,
        default_value("false"),
        default_value_if("to_file", "true", ".")
    )]
    pub to_file: String,

    /// whether or not the output should be written to a database. Defaults to false.
    #[arg(short = 's', long, default_value("false"))]
    pub to_sql: String,
}

#[derive(Parser, Debug, Clone, Copy, Serialize)]
pub struct TimingArgs {
    /// interval at which data is generated in seconds
    #[arg(short, long)]
    pub interval: Option<u16>,

    /// duration for which readings are generated in seconds
    #[arg(short, long)]
    pub duration: Option<u16>,

    /// number of readings that are generated. Should only be used with either interval or duration, but not both.
    #[arg(short, long)]
    pub number: Option<u16>,
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

#[derive(Debug, Subcommand, Clone, Copy, Serialize)]
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

#[derive(Debug, Clone, ValueEnum, Copy, Serialize)]
pub enum TemperatureUnit {
    Celsius,
    Kelvin,
}

#[derive(Debug, Clone, ValueEnum, Copy, Serialize)]
pub enum PressureUnit {
    Pascal,
    Bar,
}

#[derive(Debug, Clone, ValueEnum, Copy, Serialize)]
pub enum HumidityUnit {
    Absolute,
    Relative,
}

pub fn parse_and_validate() -> Result<Args, String> {
    let mut args = Args::parse();

    args.timing_args.validate().map_err(|e| e.to_string())?;

    Ok(args)
}
