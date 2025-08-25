mod args;

use crate::args::parse_and_validate;

fn main() {
    let args = parse_and_validate().expect("invalid arguments");

    println!("sensor_type: {:?}", args.sensor_type);
    println!("interval: {:?}", args.timing_args.interval);
    println!("duration: {:?}", args.timing_args.duration);
    println!("number: {:?}", args.timing_args.number);
}
