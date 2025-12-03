use crate::args::Args;
use crate::utils::input::read_input_lines;
use itertools::Itertools;

struct Battery {
    voltage: i32,
}

impl From<&i32> for Battery {
    fn from(voltage: &i32) -> Self {
        Battery { voltage: *voltage }
    }
}

fn find_max_comb(batteries: &[Battery], comb_size: usize) -> Vec<&Battery> {
    let max_pair = batteries
        .iter()
        .combinations(comb_size)
        .max_by_key(|pair| pair.iter().fold(0i64, |acc, b| acc * 10 + b.voltage as i64))
        .unwrap();

    max_pair.to_vec()
}

pub fn main(args: &Args) {
    let combs = if args.part == 1 { 2 } else { 12 };

    let battery_lines: Vec<Vec<Battery>> =
        read_input_lines(args.day as u32, args.input_tag.as_deref())
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect_vec()
            })
            .map(|vec| vec.iter().map(Battery::from).collect_vec())
            .collect_vec();

    let total_voltage: i64 = battery_lines
        .iter()
        .map(|batteries| find_max_comb(batteries, combs))
        .map(|pair| pair.iter().fold(0i64, |acc, b| acc * 10 + b.voltage as i64))
        .sum();
    println!("Total voltage: {}", total_voltage);
}
