use aoc_2025::args::Args;
use clap::Parser;
use aoc_2025::days;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Args::parse();
    match args.day {
        1 => days::day1::main(&args),
        _ => panic!("Day {} is not yet implemented", args.day),
    }


}