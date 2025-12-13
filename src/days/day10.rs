// --- Day 10: Factory ---
//
// Just across the hall, you find a large factory. Fortunately, the Elves here have plenty of time to decorate. Unfortunately, it's because the factory machines are all offline, and none of the Elves can figure out the initialization procedure.
//
// The Elves do have the manual for the machines, but the section detailing the initialization procedure was eaten by a Shiba Inu. All that remains of the manual are some indicator light diagrams, button wiring schematics, and joltage requirements for each machine.
//
// For example:
//
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
// The manual describes one machine per line. Each line contains a single indicator light diagram in [square brackets], one or more button wiring schematics in (parentheses), and joltage requirements in {curly braces}.
//
// To start a machine, its indicator lights must match those shown in the diagram, where . means off and # means on. The machine has the number of indicator lights shown, but its indicator lights are all initially off.
//
// So, an indicator light diagram like [.##.] means that the machine has four indicator lights which are initially off and that the goal is to simultaneously configure the first light to be off, the second light to be on, the third to be on, and the fourth to be off.
//
// You can toggle the state of indicator lights by pushing any of the listed buttons. Each button lists which indicator lights it toggles, where 0 means the first light, 1 means the second light, and so on. When you push a button, each listed indicator light either turns on (if it was off) or turns off (if it was on). You have to push each button an integer number of times; there's no such thing as "0.5 presses" (nor can you push a button a negative number of times).
//
// So, a button wiring schematic like (0,3,4) means that each time you push that button, the first, fourth, and fifth indicator lights would all toggle between on and off. If the indicator lights were [#.....], pushing the button would change them to be [...##.] instead.
//
// Because none of the machines are running, the joltage requirements are irrelevant and can be safely ignored.
//
// You can push each button as many times as you like. However, to save on time, you will need to determine the fewest total presses required to correctly configure all indicator lights for all machines in your list.
//
// There are a few ways to correctly configure the first machine:
//
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// You could press the first three buttons once each, a total of 3 button presses.
// You could press (1,3) once, (2,3) once, and (0,1) twice, a total of 4 button presses.
// You could press all of the buttons except (1,3) once each, a total of 5 button presses.
// However, the fewest button presses required is 2. One way to do this is by pressing the last two buttons ((0,2) and (0,1)) once each.
//
// The second machine can be configured with as few as 3 button presses:
//
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// One way to achieve this is by pressing the last three buttons ((0,4), (0,1,2), and (1,2,3,4)) once each.
//
// The third machine has a total of six indicator lights that need to be configured correctly:
//
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
// The fewest presses required to correctly configure it is 2; one way to do this is by pressing buttons (0,3,4) and (0,1,2,4,5) once each.
//
// So, the fewest button presses required to correctly configure the indicator lights on all of the machines is 2 + 3 + 2 = 7.
//
// Analyze each machine's indicator light diagram and button wiring schematics. What is the fewest button presses required to correctly configure the indicator lights on all of the machines?
//
// Your puzzle answer was 542.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
//
// All of the machines are starting to come online! Now, it's time to worry about the joltage requirements.
//
// Each machine needs to be configured to exactly the specified joltage levels to function properly. Below the buttons on each machine is a big lever that you can use to switch the buttons from configuring the indicator lights to increasing the joltage levels. (Ignore the indicator light diagrams.)
//
// The machines each have a set of numeric counters tracking its joltage levels, one counter per joltage requirement. The counters are all initially set to zero.
//
// So, joltage requirements like {3,5,4,7} mean that the machine has four counters which are initially 0 and that the goal is to simultaneously configure the first counter to be 3, the second counter to be 5, the third to be 4, and the fourth to be 7.
//
// The button wiring schematics are still relevant: in this new joltage configuration mode, each button now indicates which counters it affects, where 0 means the first counter, 1 means the second counter, and so on. When you push a button, each listed counter is increased by 1.
//
// So, a button wiring schematic like (1,3) means that each time you push that button, the second and fourth counters would each increase by 1. If the current joltage levels were {0,1,2,3}, pushing the button would change them to be {0,2,2,4}.
//
// You can push each button as many times as you like. However, your finger is getting sore from all the button pushing, and so you will need to determine the fewest total presses required to correctly configure each machine's joltage level counters to match the specified joltage requirements.
//
// Consider again the example from before:
//
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
// Configuring the first machine's counters requires a minimum of 10 button presses. One way to do this is by pressing (3) once, (1,3) three times, (2,3) three times, (0,2) once, and (0,1) twice.
//
// Configuring the second machine's counters requires a minimum of 12 button presses. One way to do this is by pressing (0,2,3,4) twice, (2,3) five times, and (0,1,2) five times.
//
// Configuring the third machine's counters requires a minimum of 11 button presses. One way to do this is by pressing (0,1,2,3,4) five times, (0,1,2,4,5) five times, and (1,2) once.
//
// So, the fewest button presses required to correctly configure the joltage level counters on all of the machines is 10 + 12 + 11 = 33.
//
// Analyze each machine's joltage requirements and button wiring schematics. What is the fewest button presses required to correctly configure the joltage level counters on all of the machines?

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
