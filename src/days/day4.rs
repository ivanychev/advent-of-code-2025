use crate::args::Args;
use crate::utils::input::read_input_lines;
use crate::utils::point::Point;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Cell {
    Empty,
    Paper,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '@' => Cell::Paper,
            _ => panic!("Invalid cell character: {}", c),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct PointMetadata {
    point: Point,
    cell: Cell,
    neighbour_count: usize,
}

fn count_neighbours(grid: &[Vec<Cell>], point: &Point) -> usize {
    let max_y = grid.len();
    let max_x = grid[0].len();
    point
        .neighbours(max_x, max_y)
        .filter(|p| grid[p.y][p.x] == Cell::Paper)
        .count()
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    cell_meta: HashMap<Point, PointMetadata>,
}

impl Grid {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        let mut cell_meta = HashMap::new();
        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let point = Point { x, y };
                let neighbour_count = count_neighbours(&cells, &point);
                cell_meta.insert(
                    point,
                    PointMetadata {
                        point,
                        cell: *cell,
                        neighbour_count,
                    },
                );
            }
        }

        Grid { cells, cell_meta }
    }

    fn clone_with_removed(&self, points: &[Point]) -> Self {
        let mut new_cells = self.cells.clone();
        for point in points {
            new_cells[point.y][point.x] = Cell::Empty;
        }
        Grid::new(new_cells)
    }

    fn find_pickable_papers(&self) -> Vec<Point> {
        self.cell_meta
            .values()
            .filter(|p| p.neighbour_count < 4 && p.cell == Cell::Paper)
            .map(|p| p.point)
            .collect()
    }
}

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref())
        .iter()
        .map(|line| line.chars().map(Cell::from).collect::<Vec<Cell>>())
        .collect::<Vec<Vec<Cell>>>();

    let mut grid = Grid::new(lines);

    if args.part == 1 {
        println!("Answer: {}", grid.find_pickable_papers().len());
    } else if args.part == 2 {
        let mut removed = 0;
        let mut last_removed = 1;
        while last_removed > 0 {
            let pickable = grid.find_pickable_papers();
            last_removed = pickable.len();
            removed += last_removed;
            grid = grid.clone_with_removed(&pickable);
        }

        println!("Answer part 2: {}", removed);
    } else {
        panic!("Invalid part: {}", args.part);
    }
}
