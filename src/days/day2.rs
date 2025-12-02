use crate::args::Args;
use crate::utils::input::read_input_lines;
use crate::utils::integers::{count_digits, divisors};

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct RangeIterator {
    cur: i64,
    end_inclusive: i64,
}

impl IntoIterator for &Range {
    type Item = i64;
    type IntoIter = RangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        RangeIterator {
            cur: self.start,
            end_inclusive: self.end,
        }
    }
}

impl Iterator for RangeIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur > self.end_inclusive {
            None
        } else {
            let result = self.cur;
            self.cur += 1;
            Some(result)
        }
    }
}

impl Range {
    fn from_str(s: &str) -> Range {
        let parts: Vec<&str> = s.split('-').collect();
        Range {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        }
    }

    fn overlaps_or_adjacent(&self, other: &Range) -> bool {
        !(self.end < other.start || self.start > other.end)
    }

    fn merge_adjacent(&self, other: &Range) -> Result<Range, String> {
        if self.overlaps_or_adjacent(other) {
            Ok(Range {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        } else {
            Err(format!(
                "Ranges {:?} and {:?} are not overlapping or adjacent",
                self, other
            ))
        }
    }
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut merged_ranges = Vec::<Range>::new();

    for range in ranges {
        if let Some(last_range) = merged_ranges.last_mut() {
            if let Ok(merged) = last_range.merge_adjacent(&range) {
                *last_range = merged;
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }
    merged_ranges
}

fn is_invalid_pt2(id: i64) -> bool {
    let digit_count = count_digits(id);
    for digit_divisor in divisors(digit_count) {
        if digit_divisor == digit_count {
            continue;
        }
        let chunk = id % 10_i64.pow(digit_divisor);
        let chunk_count_expected = digit_count / digit_divisor;
        let mut expected_id = 0;
        for _ in 0..chunk_count_expected {
            expected_id = expected_id * 10_i64.pow(digit_divisor) + chunk;
        }
        if expected_id == id {
            return true;
        }
    }
    false
}

#[test]
fn test_is_invalid_pt2() {
    assert!(is_invalid_pt2(1212));
    assert!(is_invalid_pt2(123123));
    assert!(is_invalid_pt2(999999));
    assert!(!is_invalid_pt2(1234));
    assert!(!is_invalid_pt2(12312));
    assert!(!is_invalid_pt2(1));
}

fn main_part1(ranges: &[Range]) {
    let mut invalid_ids_sum = 0;
    for range in ranges {
        for id in range.into_iter() {
            let id_str = id.to_string();
            if id_str.len() % 2 == 0 && id_str[0..id_str.len() / 2] == id_str[id_str.len() / 2..] {
                invalid_ids_sum += id;
            }
        }
    }

    println!("Invalid IDs sum: {invalid_ids_sum}");
}

fn main_part2(ranges: &[Range]) {
    let mut invalid_ids_sum = 0;
    for range in ranges {
        for id in range.into_iter() {
            if is_invalid_pt2(id) {
                invalid_ids_sum += id;
            }
        }
    }
    println!("Invalid IDs sum for part 2: {invalid_ids_sum}");
}

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());
    let mut ranges: Vec<Range> = lines[0]
        .split(',')
        .map(Range::from_str)
        .collect::<Vec<Range>>();

    ranges = merge_ranges(ranges);
    if args.part == 1 {
        main_part1(&ranges);
    } else {
        main_part2(&ranges);
    }
}
