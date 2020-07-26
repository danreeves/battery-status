extern crate battery;
extern crate structopt;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Cli {
    #[structopt(short = "f", long = "format", required = false, default_value = "")]
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
    let manager = battery::Manager::new().expect("Couldn't find any batteries...");

    let battery = match manager
        .batteries()
        .expect("Couldn't find any batteries...")
        .next()
    {
        Some(Ok(battery)) => battery,
        Some(Err(_e)) => {
            println!("Error: {}", _e);
            return;
        }
        None => {
            println!("Unable to find any batteries");
            return;
        }
    };

    if args.percent {
        println!("{:?}", battery.state_of_charge())
    }

    if args.format.len() > 0 {
        println!("100% Charging")
    }
}
