use aoc_2025::args::Args;
use aoc_2025::days;
use clap::Parser;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Args::parse();
    match args.day {
        1 => days::day1::main(&args),
        2 => days::day2::main(&args),
        3 => days::day3::main(&args),
        4 => days::day4::main(&args),
        5 => days::day5::main(&args),
        6 => days::day6::main(&args),
        7 => days::day7::main(&args),
        8 => days::day8::main(&args),
        _ => panic!("Day {} is not yet implemented", args.day),
    }
}
