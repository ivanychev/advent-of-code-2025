use crate::args::Args;
use crate::utils::input::read_input_lines;
use good_lp::{
    Expression, IntoAffineExpression, ProblemVariables, Solution, SolverModel, Variable,
    constraint, default_solver, variable,
};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Light {
    On,
    Off,
}

impl Light {
    fn invert(&self) -> Light {
        match self {
            Light::On => Light::Off,
            Light::Off => Light::On,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct LightCompbination {
    lights: Vec<Light>,
}

impl LightCompbination {
    fn press_inplace(&mut self, button: &[usize]) {
        for &index in button {
            self.lights[index] = self.lights[index].invert()
        }
    }
}

impl From<char> for Light {
    fn from(c: char) -> Self {
        match c {
            '#' => Light::On,
            '.' => Light::Off,
            _ => panic!("Invalid light character: {}", c),
        }
    }
}

struct Machine {
    desired_lights: LightCompbination,
    buttons: Vec<Vec<usize>>,
    joltage_levels: Vec<usize>,
}

impl Machine {
    fn initial_lights(&self) -> LightCompbination {
        LightCompbination {
            lights: vec![Light::Off; self.desired_lights.lights.len()],
        }
    }

    fn min_joltage_presses(&self) -> usize {
        let mut problem = ProblemVariables::new();
        let variables = self
            .buttons
            .iter()
            .map(|_| problem.add(variable().integer().min(0)))
            .collect::<Vec<_>>();

        let expr: Expression = variables.iter().map(|v| v.into_expression()).sum();

        let constraints = self
            .joltage_levels
            .iter()
            .enumerate()
            .map(|(component_id, joltage)| {
                let component_vars: Vec<Variable> = variables
                    .iter()
                    .enumerate()
                    .filter_map(|(button_id, variable)| {
                        let button = &self.buttons[button_id];
                        if button.contains(&component_id) {
                            Some(*variable)
                        } else {
                            None
                        }
                    })
                    .collect();
                let sum_expr = component_vars
                    .iter()
                    .map(|v| v.into_expression())
                    .sum::<Expression>();
                constraint!(sum_expr == *joltage as i32)
            });
        let mut problem = problem.minimise(expr).using(default_solver);
        for constr in constraints {
            problem = problem.with(constr);
        }
        let solution = problem.solve().unwrap();
        println!("Joltage solution: {:?}", solution.value(variables[0]));
        variables
            .iter()
            .map(|var| solution.value(*var) as usize)
            .sum()
    }

    fn min_lights_presses(&self) -> usize {
        let mut presses = 1usize;
        loop {
            let any_comb = self
                .buttons
                .iter()
                .combinations(presses)
                .find(|combination| {
                    let mut lights = self.initial_lights();
                    for button in combination {
                        lights.press_inplace(button);
                    }
                    lights == self.desired_lights
                });
            if let Some(found) = any_comb {
                println!("Found combination with {} presses: {:?}", presses, found);
                return presses;
            } else {
                presses += 1;
            }
        }
    }
}

impl From<&str> for Machine {
    fn from(input: &str) -> Self {
        // Example:
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let parts: Vec<&str> = input.split_whitespace().collect();
        let lights: Vec<Light> = parts[0]
            .chars()
            .skip(1)
            .take(parts[0].chars().count().checked_sub(2).unwrap())
            .map(Light::from)
            .collect();
        let raw_buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|s| {
                s.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|num_str| num_str.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();
        let joltage_levels = parts
            .last()
            .unwrap()
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Machine {
            desired_lights: LightCompbination { lights },
            buttons: raw_buttons,
            joltage_levels,
        }
    }
}

pub fn main(args: &Args) {
    let input_lines = read_input_lines(args.day as u32, args.input_tag.as_deref());
    let machines: Vec<Machine> = input_lines
        .iter()
        .map(|line| Machine::from(line.as_str()))
        .collect();

    match args.part {
        1 => {
            let mut total_presses = 0usize;
            for (i, machine) in machines.iter().enumerate() {
                let min_presses = machine.min_lights_presses();
                println!("Machine {}: Minimum presses = {}", i + 1, min_presses);
                total_presses += min_presses;
            }

            println!("Total presses: {}", total_presses);
        }
        2 => {
            let mut total_presses = 0usize;
            for (i, machine) in machines.iter().enumerate() {
                let min_presses = machine.min_joltage_presses();
                println!(
                    "Machine {}: Minimum joltage presses = {}",
                    i + 1,
                    min_presses
                );
                total_presses += min_presses;
            }

            println!("Total joltage presses: {}", total_presses);
        }
        _ => {
            println!("Part {} is not implemented yet.", args.part);
        }
    }
}
