use crate::args::Args;
use crate::utils::input::read_input_lines;
use crate::utils::point::Point;
use itertools::Itertools;

fn get_area(p1: &Point, p2: &Point) -> i64 {
    let width = (p1.x as i64 - p2.x as i64).abs() + 1;
    let height = (p1.y as i64 - p2.y as i64).abs() + 1;
    width * height
}

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());
    let points: Vec<_> = lines
        .into_iter()
        .map(|line| Point::from(line.as_str()))
        .collect::<Vec<_>>();

    let max_rectangle = points
        .iter()
        .combinations(2)
        .max_by_key(|x| get_area(x[0], x[1]))
        .unwrap();

    println!("{:?}", get_area(max_rectangle[0], max_rectangle[1]));
}
