use crate::args::Args;
use crate::utils::input::read_input_lines;
use range_collections::RangeSet2;
use std::ops::Range;

fn parse_range(s: &str) -> Range<i64> {
    let (start, end) = s.split_once('-').unwrap();

    start.parse::<i64>().unwrap()..(end.parse::<i64>().unwrap() + 1i64)
}

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());
    let ranges: Vec<_> = lines
        .iter()
        .filter(|s| s.contains('-'))
        .map(|s| parse_range(s))
        .collect();

    let ids: Vec<_> = lines
        .iter()
        .filter(|s| !s.contains('-'))
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let interval_tree = ranges.iter().fold(RangeSet2::<i64>::empty(), |mut acc, r| {
        acc.union_with(&RangeSet2::from(r.clone()));
        acc
    });

    match args.part {
        1 => {
            let fresh_count = ids.iter().filter(|&id| interval_tree.contains(id)).count();
            println!("{}", fresh_count);
        }
        2 => {
            let size = interval_tree
                .into_inner()
                .as_slice()
                .chunks(2)
                .map(|c| c[1] - c[0])
                .sum::<i64>();

            println!("{}", size);
        }
        _ => {
            println!("Part {} is not yet implemented", args.part);
        }
    }
}
