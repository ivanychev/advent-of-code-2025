#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn neighbours(&self, max_x: usize, max_y: usize) -> impl Iterator<Item = Point> {
        let mut neighbours: [Option<Point>; 8] = [None; 8];
        let mut cur: i32 = -1;
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                cur += 1;
                let new_x = self.x as i32 + dx;
                let new_y = self.y as i32 + dy;
                if new_x >= 0 && new_x < max_x as i32 && new_y >= 0 && new_y < max_y as i32 {
                    neighbours[cur as usize] = Some(Point {
                        x: new_x as usize,
                        y: new_y as usize,
                    });
                }
            }
        }
        neighbours.into_iter().flatten()
    }
}
