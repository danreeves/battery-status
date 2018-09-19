extern crate battery_status;
extern crate structopt;

use battery_status::{BatteryState, BatteryStatus};
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Cli {
    #[structopt(
        short = "f",
        long = "format",
        required = false,
        default_value = "p s"
    )]
    format: String,

    #[structopt(
        short = "p",
        long = "percent",
        conflicts_with = "format",
        conflicts_with = "status"
    )]
    percent: bool,

    #[structopt(
        short = "s",
        long = "status",
        conflicts_with = "format",
        conflicts_with = "percent"
    )]
    status: bool,
}

fn main() {
    let args = Cli::from_args();
    let batt = BatteryStatus::new();

    if args.percent {
        match batt.percent {
            Some(percent) => {
                println!("{}", percent);
                process::exit(0);
            }
            None => {
                println!("Error: Couldn't get the battery level");
                process::exit(1);
            }
        }
    }

    if args.status {
        let (state, exit_code) = match batt.status {
            BatteryState::Unknown => ("unknown", 1),
            BatteryState::Discharging => ("discharging", 0),
            BatteryState::Charging => ("charging", 0),
            BatteryState::ACPower => ("ac_power", 0),
        };
        println!("{}", state);
        process::exit(exit_code);
    }

    if args.format.len() > 0 {
        unimplemented!()
    } else {
        println!("");
    }
}
