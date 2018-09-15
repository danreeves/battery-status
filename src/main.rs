extern crate battery_status;
extern crate structopt;

use battery_status::BatteryStatus;
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
    let args = Cli::from_args();
    let batt = BatteryStatus::new();
    match batt.percent {
        Some(percent) => println!("{}%", percent),
        None => println!("Could not get the percent"),
    }
}
