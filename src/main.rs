extern crate battery_status;
extern crate structopt;

use battery_status::{BatteryState, BatteryStatus};
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

    #[structopt(short = "p", long = "percent")]
    percent: bool,

    #[structopt(short = "s", long = "status")]
    status: bool,
}

fn main() {
    let batt = BatteryStatus::new();
    match batt.percent {
        Some(percent) => println!("Battery level: {}%", percent),
        None => println!("Could not get the percent"),
    }
    let state = match batt.status {
        BatteryState::Unknown => "unknown",
        BatteryState::Discharging => "discharging",
        BatteryState::Charging => "charging up",
        BatteryState::ACPower => "on the line",
    };
    println!("Battery is: {}", state);
}
