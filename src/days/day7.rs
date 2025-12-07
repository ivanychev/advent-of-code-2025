use crate::args::Args;
use crate::utils::input::read_input_lines;
use crate::utils::point::Point;
use smallvec::SmallVec;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
enum Cell {
    Start,
    Splitter,
    Empty,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    start_pos: Point,
}

struct ProjectBeamResult {
    beams: Vec<Point>,
    splits: usize,
}

impl Grid {
    fn get_max_y(&self) -> usize {
        self.cells.len()
    }

    fn is_splitter(&self, point: Point) -> bool {
        self.cells[point.y][point.x] == Cell::Splitter
    }

    fn project_beams(&self, beams: &[Point], y: usize) -> ProjectBeamResult {
        let new_splits: BTreeMap<Point, usize> = beams
            .iter()
            .flat_map(|p| {
                let splitted: SmallVec<[Point; 2]> = if self.is_splitter(Point { x: p.x, y }) {
                    SmallVec::<[Point; 2]>::from_buf([
                        Point { x: p.x - 1, y },
                        Point { x: p.x + 1, y },
                    ])
                } else {
                    SmallVec::<[Point; 2]>::new()
                };
                splitted.into_iter()
            })
            .fold(BTreeMap::<Point, usize>::new(), |mut acc, p| {
                *acc.entry(p).or_insert(0) += 1;
                acc
            });
        let split_count: usize = new_splits.values().sum::<usize>() / 2usize; // each split creates two beams
        let mut final_beams: BTreeSet<Point> = beams
            .iter()
            .flat_map(|p| {
                if !self.is_splitter(Point { x: p.x, y }) {
                    Some(Point { x: p.x, y })
                } else {
                    None
                }
            })
            .collect();
        final_beams.extend(new_splits.into_keys());

        ProjectBeamResult {
            beams: final_beams.into_iter().collect(),
            splits: split_count,
        }
    }

    fn count_trajectories(&self, beam: Point) -> usize {
        let mut cache: HashMap<Point, usize> = HashMap::new();
        self.count_trajectories_impl(beam, &mut cache)
    }

    fn count_trajectories_impl(&self, beam: Point, cache: &mut HashMap<Point, usize>) -> usize {
        if beam.y == self.get_max_y() - 1 {
            return 1;
        }
        if let Some(&cached) = cache.get(&beam) {
            return cached;
        };
        let next_y = beam.y + 1;
        if self.is_splitter(Point {
            x: beam.x,
            y: next_y,
        }) {
            let left_beam = Point {
                x: beam.x - 1,
                y: next_y,
            };
            let right_beam = Point {
                x: beam.x + 1,
                y: next_y,
            };
            let left_count = self.count_trajectories_impl(left_beam, cache);
            let right_count = self.count_trajectories_impl(right_beam, cache);
            let total = left_count + right_count;
            cache.insert(beam, total);
            total
        } else {
            let straight_beam = Point {
                x: beam.x,
                y: next_y,
            };
            let total = self.count_trajectories_impl(straight_beam, cache);
            cache.insert(beam, total);
            total
        }
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Cell::Start,
            b'^' => Cell::Splitter,
            b'.' => Cell::Empty,
            _ => panic!("Invalid cell character: {}", value as char),
        }
    }
}

impl From<Vec<String>> for Grid {
    fn from(value: Vec<String>) -> Self {
        let grid: Vec<Vec<Cell>> = value
            .into_iter()
            .map(|row| row.bytes().map(Cell::from).collect())
            .collect();
        let start_pair = grid
            .first()
            .expect("Grid should have at least one element")
            .iter()
            .enumerate()
            .find(|&c| *c.1 == Cell::Start)
            .expect("Grid should have a start in the top row");
        let start_pos = Point {
            x: start_pair.0,
            y: 0,
        };

        Grid {
            cells: grid,
            start_pos,
        }
    }
}

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());
    let grid = Grid::from(lines);

    match args.part {
        1 => {
            let mut current_y = 1;
            let mut beams = vec![grid.start_pos];
            let mut splits = 0;
            while current_y < grid.get_max_y() {
                let beam_result = grid.project_beams(&beams, current_y);
                // println!("Beams at y={} : {:?}, splits={}", current_y, beam_result.beams, beam_result.splits);
                beams = beam_result.beams;
                splits += beam_result.splits;
                current_y += 1;
            }

            println!("Solution: {}", splits);
        }
        2 => {
            let total_trajectories = grid.count_trajectories(grid.start_pos);
            println!("Solution: {}", total_trajectories);
        }
        _ => {
            panic!("Part {} is not yet implemented", args.part);
        }
    }
}
