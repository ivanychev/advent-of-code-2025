use crate::args::Args;
use crate::utils::input::read_input_lines;
use crate::utils::point::Point;
use itertools::Itertools;
use log::info;
use range_collections::RangeSet2;
use roaring::RoaringTreemap;
use smallvec::SmallVec;
use std::collections::VecDeque;
use std::fs::File;

const MOD: usize = 100_000;

fn get_area(p1: &Point, p2: &Point) -> i64 {
    let width = (p1.x as i64 - p2.x as i64).abs() + 1;
    let height = (p1.y as i64 - p2.y as i64).abs() + 1;
    width * height
}

pub fn main_part1(points: Vec<Point>) {
    let max_rectangle = points
        .iter()
        .combinations(2)
        .max_by_key(|x| get_area(x[0], x[1]))
        .unwrap();

    println!("{:?}", get_area(max_rectangle[0], max_rectangle[1]));
}

fn get_max_coord(points: &[Point], coord_getter: fn(&Point) -> usize) -> usize {
    points.iter().map(coord_getter).max().unwrap()
}

struct RecChecker {
    y_to_ranges: Vec<RangeSet2<usize>>,
}

impl RecChecker {
    fn from_red_green_se(red_green_se: &RoaringTreemap, max_y_exclusive: usize) -> Self {
        // let mut y_to_ranges: Vec<RangeSet2<usize>> = vec![RangeSet2::empty(); max_y_exclusive];
        let mut y_to_ranges: Vec<RangeSet2<usize>> =
            (0..max_y_exclusive).map(|_| RangeSet2::empty()).collect();
        let mut total = 0i64;

        red_green_se
            .iter()
            .map(|point| Point::from_u64_mod(point, MOD))
            .for_each(|p| {
                total += 1;
                if total % 25_000_000 == 0 {
                    info!(
                        "Processed {} % red-green cells for RecChecker...",
                        total as f64 / red_green_se.len() as f64 * 100.0
                    );
                }
                y_to_ranges[p.y] |= RangeSet2::from(p.x..p.x + 1);
            });

        let lenghts_debug: Vec<(usize, SmallVec<[usize; 2]>)> = y_to_ranges
            .iter()
            .enumerate()
            .map(|(idx, rs)| (idx, rs.clone().into_inner()))
            .collect::<Vec<_>>();

        for (idx, p) in lenghts_debug.iter().enumerate() {
            if idx % 100 != 0 {
                continue;
            }
            println!("Y={} has {} ranges: {:?}", p.0, p.1.len(), p.1);
            println!("{:?}", y_to_ranges[p.0]);
        }

        RecChecker { y_to_ranges }
    }

    fn is_rec_in(&self, p1: &Point, p2: &Point) -> bool {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);
        for y in min_y..=max_y {
            let range_set = &self.y_to_ranges[y];
            let query_range = RangeSet2::from(min_x..max_x + 1);
            if !query_range.is_subset(range_set) {
                return false;
            }
        }
        true
    }
}

fn rec_area(p1: &Point, p2: &Point) -> usize {
    let width = p1.x.abs_diff(p2.x) + 1;
    let height = p1.y.abs_diff(p2.y) + 1;
    width * height
}

#[test]
fn is_subset() {
    let s1 = RangeSet2::from(0..5);
    let s2 = RangeSet2::from(1..4);
    assert!(s2.is_subset(&s1));
}

fn build_red_green_se(
    visited_outer: &RoaringTreemap,
    max_x_exclusive: usize,
    max_y_exclusive: usize,
) -> RoaringTreemap {
    let mut total = 0i64;
    let should_process = (max_x_exclusive as i64) * (max_y_exclusive as i64);

    let red_green: RoaringTreemap = (0..max_x_exclusive)
        .cartesian_product(0..max_y_exclusive)
        .map(|(x, y)| Point { x, y }.to_u64_mod(MOD))
        .filter(|p_repr| {
            total += 1;
            if total % 25_000_000 == 0 {
                info!(
                    "Processed {}% cells for red-green set...",
                    total as f64 / should_process as f64 * 100.0
                );
            }
            !visited_outer.contains(*p_repr)
        })
        .collect();

    red_green
}

fn print_in_set(p: Point, visited_outer: &RoaringTreemap) {
    let is_in = visited_outer.contains(p.to_u64_mod(MOD));
    println!("Point {:?} is in visited_outer: {}", p, is_in);
}

pub fn main_part2(mut points: Vec<Point>) {
    let (max_field_x_exclusive, max_field_y_exclusive) = (
        get_max_coord(&points, |p| p.x) + 3usize,
        get_max_coord(&points, |p| p.y) + 3usize,
    );

    let mut colored_points = RoaringTreemap::new();
    let first_point = points[0];
    points.push(first_point);

    points.windows(2).for_each(|w| {
        colored_points.insert(w[0].to_u64_mod(MOD));
        let greens_between = w[0].manhattan_distance(&w[1]).checked_sub(1).unwrap();
        w[0].walk_to(&w[1]).take(greens_between).for_each(|p| {
            colored_points.insert(p.to_u64_mod(MOD));
        });
    });

    let mut visited_outer: RoaringTreemap = RoaringTreemap::new();
    let file_outer_path = "/Users/iv/Desktop/visited_outer2.roaring";
    let file_inner_path = "/Users/iv/Desktop/visited_inner2.roaring";
    // let file_outer_path = "/Users/iv/Desktop/visited_outer_test.roaring";
    // let file_inner_path = "/Users/iv/Desktop/visited_inner_test.roaring";

    // // Building the visited_outer set
    {
        let mut q = VecDeque::from([Point { x: 0, y: 0 }]);
        let mut iterations: i64 = 0;
        while let Some(point) = q.pop_front() {
            iterations += 1;
            if iterations % 25_000_000 == 0 {
                println!(
                    "Iterations: {}, queue size: {}, visited size: {}",
                    iterations,
                    q.len(),
                    visited_outer.len()
                );
            }
            if visited_outer.contains(point.to_u64_mod(MOD)) {
                continue;
            }
            visited_outer.insert(point.to_u64_mod(MOD));
            let new_points = point
                .adjacent_neighbours(max_field_x_exclusive + 1, max_field_y_exclusive + 1)
                .filter(|p| {
                    !colored_points.contains(p.to_u64_mod(MOD))
                        && !visited_outer.contains(p.to_u64_mod(MOD))
                });
            q.extend(new_points);
        }

        let mut f = File::create(file_outer_path).unwrap();
        info!("Writing visited_outer to {} ...", file_outer_path);
        visited_outer.serialize_into(&mut f).unwrap();
    }
    {
        let f = File::open(file_outer_path).unwrap();
        info!("Deserializing visited_outer from file...");
        visited_outer = RoaringTreemap::deserialize_from(f).unwrap()
    }

    println!(
        "Total cells: {}",
        (max_field_x_exclusive) as i64 * (max_field_y_exclusive) as i64
    );
    println!("Colored cells: {}", colored_points.len() as i64);
    println!("Outer empty cells: {}", visited_outer.len() as i64);

    let mut red_green_se = RoaringTreemap::new();
    {
        info!(
            "Building red-green set, initial len: {} ...",
            red_green_se.len()
        );
        red_green_se =
            build_red_green_se(&visited_outer, max_field_x_exclusive, max_field_y_exclusive);

        let mut f = File::create(file_inner_path).unwrap();
        info!("Writing red-green set to {} ...", file_inner_path);
        red_green_se.serialize_into(&mut f).unwrap();
        info!("Wrote red-green set to {} ...", file_inner_path);
    }
    {
        let f = File::open(file_inner_path).unwrap();
        info!("Deserializing red-green from file...");
        red_green_se = RoaringTreemap::deserialize_from(f).unwrap()
    }
    info!(
        "Red-green set built with {} cells. Building checker ...",
        red_green_se.len()
    );
    // print_in_set(Point{x: 90000, y: 90000}, &red_green_se);
    for i in 0..100 {
        print_in_set(
            Point {
                x: i * 1000,
                y: 90000,
            },
            &visited_outer,
        );
    }
    let rec_checker = RecChecker::from_red_green_se(&red_green_se, max_field_y_exclusive);
    let mut max_area: usize = 0;
    let mut best_pair: Option<(Point, Point)> = None;

    points.iter().combinations(2).for_each(|pair| {
        let p1 = pair[0];
        let p2 = pair[1];
        let area = rec_area(p1, p2);
        if area > max_area && rec_checker.is_rec_in(p1, p2) {
            max_area = area;
            println!(
                "New max area: {} for points {:?} and {:?}",
                max_area, p1, p2
            );
            best_pair = Some((*p1, *p2));
        }
    });
    println!("Best pair: {:?} with area {}", best_pair, max_area);
}

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());
    let points: Vec<_> = lines
        .into_iter()
        .map(|line| Point::from(line.as_str()))
        .collect::<Vec<_>>();
    match args.part {
        1 => main_part1(points),
        2 => main_part2(points),
        _ => panic!("Part {} not implemented", args.part),
    }
}
