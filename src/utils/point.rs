#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let (x_str, y_str) = s.split_once(',').unwrap();
        Point {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        }
    }
}

impl Point {
    pub fn neighbours(
        &self,
        max_x_exclusive: usize,
        max_y_exclusive: usize,
    ) -> impl Iterator<Item = Point> {
        let mut neighbours: [Option<Point>; 8] = [None; 8];
        let mut cur: i64 = -1;
        for dx in -1i64..=1 {
            for dy in -1i64..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                cur += 1;
                let new_x = self.x as i64 + dx;
                let new_y = self.y as i64 + dy;
                if new_x >= 0
                    && new_x < max_x_exclusive as i64
                    && new_y >= 0
                    && new_y < max_y_exclusive as i64
                {
                    neighbours[cur as usize] = Some(Point {
                        x: new_x as usize,
                        y: new_y as usize,
                    });
                }
            }
        }
        neighbours.into_iter().flatten()
    }

    pub fn to_u64(&self) -> u64 {
        ((self.y as u64) << 32) | (self.x as u64)
    }

    pub fn from_u64(value: u64) -> Point {
        Point {
            y: (value >> 32) as usize,
            x: (value & 0xFFFFFFFF) as usize,
        }
    }

    pub fn from_u64_mod(value: u64, x_mod: usize) -> Point {
        Point {
            x: (value % (x_mod as u64)) as usize,
            y: (value / (x_mod as u64)) as usize,
        }
    }

    pub fn to_u64_mod(&self, x_mod: usize) -> u64 {
        (self.y as u64) * (x_mod as u64) + (self.x as u64)
    }

    pub fn adjacent_neighbours(
        &self,
        max_x_exclusive: usize,
        max_y_exclusive: usize,
    ) -> impl Iterator<Item = Point> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(move |(dx, dy)| {
                let new_x = self.x as i64 + dx;
                let new_y = self.y as i64 + dy;
                if new_x >= 0
                    && new_x < max_x_exclusive as i64
                    && new_y >= 0
                    && new_y < max_y_exclusive as i64
                {
                    Some(Point {
                        x: new_x as usize,
                        y: new_y as usize,
                    })
                } else {
                    None
                }
            })
    }

    pub fn walk_to(&self, other: &Point) -> impl Iterator<Item = Point> {
        let dx = if other.x > self.x {
            1
        } else if other.x < self.x {
            -1
        } else {
            0
        };
        let dy = if other.y > self.y {
            1
        } else if other.y < self.y {
            -1
        } else {
            0
        };
        let abs_dx = (self.x as i64 - other.x as i64).abs();
        let abs_dy = (self.y as i64 - other.y as i64).abs();
        let dx_deltas = (1..=abs_dx).map(move |i| Point {
            x: (self.x as i64 + dx * i) as usize,
            y: self.y,
        });
        let dy_deltas = (1..=abs_dy).map(move |j| Point {
            x: (self.x as i64 + dx * abs_dx) as usize,
            y: (self.y as i64 + dy * j) as usize,
        });
        dx_deltas.chain(dy_deltas)
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[test]
fn test_point() {
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 5, y: 7 };
    let walked: Vec<Point> = p1.walk_to(&p2).collect();
    let expected = vec![
        Point { x: 3, y: 3 },
        Point { x: 4, y: 3 },
        Point { x: 5, y: 3 },
        Point { x: 5, y: 4 },
        Point { x: 5, y: 5 },
        Point { x: 5, y: 6 },
        Point { x: 5, y: 7 },
    ];
    assert_eq!(walked, expected);
}

#[test]
fn test_point_reverse() {
    let p1 = Point { x: 5, y: 7 };
    let p2 = Point { x: 2, y: 3 };
    let walked: Vec<Point> = p1.walk_to(&p2).collect();
    let expected = vec![
        Point { x: 4, y: 7 },
        Point { x: 3, y: 7 },
        Point { x: 2, y: 7 },
        Point { x: 2, y: 6 },
        Point { x: 2, y: 5 },
        Point { x: 2, y: 4 },
        Point { x: 2, y: 3 },
    ];
    assert_eq!(walked, expected);
}

#[test]
fn test_adjacent_neighbours() {
    let p = Point { x: 1, y: 1 };
    let neighbours: Vec<Point> = p.adjacent_neighbours(3, 3).collect();
    let expected = vec![
        Point { x: 0, y: 1 },
        Point { x: 2, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 1, y: 2 },
    ];
    assert_eq!(neighbours, expected);
}

#[test]
fn test_adjacent_neighbours_near_edge() {
    let p = Point { x: 0, y: 0 };
    let neighbours: Vec<Point> = p.adjacent_neighbours(3, 3).collect();
    let expected = vec![Point { x: 1, y: 0 }, Point { x: 0, y: 1 }];
    assert_eq!(neighbours, expected);

    let p = Point { x: 2, y: 2 };
    let neighbours: Vec<Point> = p.adjacent_neighbours(3, 3).collect();
    let expected = vec![Point { x: 1, y: 2 }, Point { x: 2, y: 1 }];
    assert_eq!(neighbours, expected);
}

#[test]
fn test_from_to_mod() {
    let p = Point { x: 5, y: 10 };
    let value = p.to_u64_mod(100);
    let p_converted = Point::from_u64_mod(value, 100);
    assert_eq!(p, p_converted);
}
