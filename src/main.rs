mod d1;
mod d2;
mod d3;
mod d4;
mod utils;

use clap::Parser;

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
        1 => d1::d1(args.part),
        2 => d2::d2(args.part),
        3 => d3::d3(args.part),
        4 => d4::d4(args.part),
        _ => println!("Invalid day specified"),
    }
}