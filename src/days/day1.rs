use log::info;
use crate::args::Args;
use crate::utils::read_input_lines;


pub enum Rotation {
    Left(u32),
    Right(u32),
}

pub struct Safe {
    pos: u32,
    size: u32,
}

impl Safe {
    pub fn new() -> Safe {
        Safe { pos: 50u32, size: 100u32 }
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left(x) => {
                self.pos = (self.pos as i32 - x as i32).rem_euclid(self.size as i32) as u32;
            }
            Rotation::Right(x) => {
                self.pos = (self.pos + x) % self.size;
            }
        }
    }

    pub fn get_position(&self) -> u32 {
        self.pos
    }

    pub fn is_at_start(&self) -> bool {
        self.pos == 0
    }
}

pub fn read_rotations(input: &Vec<String>) -> Vec<Rotation> {
    input
        .iter()
        .map(|line| {
            let amount = line[1..].parse::<u32>().unwrap();
            match line {
                l if l.starts_with("L") => {
                    Rotation::Left(amount)
                }
                l if l.starts_with("R") => {
                    Rotation::Right(amount)
                }
                _ => panic!("Invalid rotation line: {}", line),
            }
        })
        .collect()
}

pub fn main(args: &Args) {
    let lines = read_input_lines(1, args.input_tag.as_deref());
    let rotations = read_rotations(&lines);
    let mut safe = Safe::new();
    let mut zero_count = 0;

    for rotation in rotations {
        safe.rotate(rotation);
        info!("Current position: {}", safe.get_position());
        zero_count += if safe.is_at_start() { 1 } else { 0 };
    }
    println!("The passcode is {}.", zero_count);
}