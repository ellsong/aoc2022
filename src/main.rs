mod d1;
mod utils;

use clap::Parser;
use d1::d1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    /// #[arg(short, long)]
    day: i32,
    /// Part of day
    /// #[arg(short, long)]
    part: i32,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => d1(args.part),
        _ => println!("Invalid day specified"),
    }
}