use crate::args::Args;
use crate::utils::input::read_input_lines;

struct Example {
    height: usize,
    width: usize,
    counts: Vec<usize>,
}

impl Example {
    fn fits(&self) -> bool {
        let needed = self.counts.iter().sum::<usize>() * 9;
        let available = self.height * self.width;
        needed <= available
    }
}

impl From<&str> for Example {
    fn from(s: &str) -> Self {
        // 36x36: 32 22 20 20 22 27
        let (p1, p2) = s.split_once(':').unwrap();
        let (width_str, height_str) = p1.split_once('x').unwrap();
        let width = width_str.trim().parse::<usize>().unwrap();
        let height = height_str.trim().parse::<usize>().unwrap();
        let counts: Vec<usize> = p2
            .split_whitespace()
            .map(|part| part.parse::<usize>().unwrap())
            .collect();
        Example {
            height,
            width,
            counts,
        }
    }
}

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());

    let examples = lines
        .iter()
        .filter(|l| l.contains(": "))
        .map(|l| Example::from(l.as_str()))
        .collect::<Vec<Example>>();

    let fit_count = examples.iter().filter(|e| e.fits()).count();
    println!("Number of examples that fit: {}", fit_count);
}
