mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod d9b;
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
        5 => d5::d5(args.part),
        6 => d6::d6(args.part),
        7 => d7::d7(args.part),
        8 => d8::d8(args.part),
        9 => d9::d9(args.part),
        _ => println!("Invalid day specified"),
    }
}