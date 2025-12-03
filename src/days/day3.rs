use crate::args::Args;
use crate::utils::input::read_input_lines;
use std::collections::HashMap;

struct Battery {
    voltage: i32,
}

#[derive(Hash, Eq, PartialEq)]
struct DynamicProgrammingNode {
    remaining: usize,
    batteries_left: i32,
}

impl From<u32> for Battery {
    fn from(voltage: u32) -> Self {
        Battery {
            voltage: voltage as i32,
        }
    }
}

fn find_max_comb(batteries: &[Battery], comb_size: usize) -> Option<i64> {
    let mut cache: HashMap<DynamicProgrammingNode, i64> = HashMap::new();
    find_max_comb_impl(batteries, comb_size, &mut cache)
}

fn find_max_comb_impl(
    batteries: &[Battery],
    comb_size: usize,
    answers: &mut HashMap<DynamicProgrammingNode, i64>,
) -> Option<i64> {
    if comb_size == 0 {
        return Some(0);
    }
    if batteries.len() < comb_size {
        return None;
    }

    let node = DynamicProgrammingNode {
        remaining: batteries.len(),
        batteries_left: comb_size as i32,
    };
    if let Some(&answer) = answers.get(&node) {
        return Some(answer);
    }

    let not_taken = find_max_comb_impl(&batteries[1..], comb_size, answers);
    let taken = find_max_comb_impl(&batteries[1..], comb_size - 1, answers)
        .map(|v| 10i64.pow(comb_size as u32 - 1) * batteries[0].voltage as i64 + v);

    let values = [not_taken, taken];
    let result = values.iter().flatten().max().unwrap();
    answers.insert(node, *result);

    Some(*result)
}

pub fn main(args: &Args) {
    let combs = if args.part == 1 { 2 } else { 12 };

    let battery_lines: Vec<Vec<Battery>> =
        read_input_lines(args.day as u32, args.input_tag.as_deref())
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap().into())
                    .collect::<Vec<Battery>>()
            })
            .collect();

    println!("Battery count {}, combs: {}", battery_lines[0].len(), combs);

    let total_voltage: i64 = battery_lines
        .iter()
        .map(|batteries| find_max_comb(batteries, combs).unwrap())
        .sum();
    println!("Total voltage: {}", total_voltage);
}
