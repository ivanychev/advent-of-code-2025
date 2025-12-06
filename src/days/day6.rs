use crate::args::Args;
use crate::utils::input::read_input_lines;
use std::cmp::min;

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
        }
    }

    fn identity(&self) -> i64 {
        match self {
            Operator::Add => 0,
            Operator::Mul => 1,
        }
    }
}

impl From<&str> for Operator {
    fn from(c: &str) -> Self {
        match c.trim() {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => panic!("Invalid operator character: {}", c),
        }
    }
}

#[derive(Debug)]
struct Operation {
    operands: Vec<i64>,
    operator: Operator,
}

impl Operation {
    fn execute(&self) -> i64 {
        self.operands
            .iter()
            .fold(self.operator.identity(), |acc, &op| {
                self.operator.apply(acc, op)
            })
    }

    // 123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   +
    //
    // The rightmost problem is 4 + 431 + 623 = 1058
    // The second problem from the right is 175 * 581 * 32 = 3253600
    // The third problem from the right is 8 + 248 + 369 = 625
    // Finally, the leftmost problem is 356 * 24 * 1 = 8544
    fn from_part2(value: &[&str]) -> Self {
        let operator = Operator::from(value[value.len() - 1]);
        let max_len_operands: usize = value[..value.len() - 1]
            .iter()
            .map(|&s| s.len())
            .max()
            .unwrap_or(0);
        let padded_operands: Vec<String> = value[..value.len() - 1]
            .iter()
            .map(|&s| format!("{:width$}", s, width = max_len_operands))
            .collect();

        let operands: Vec<_> = (0..padded_operands[0].len())
            .map(|i| {
                padded_operands
                    .iter()
                    .map(|s| s.chars().nth(i).unwrap())
                    .filter(|&s| s != ' ')
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap_or_else(|_| {
                        panic!("Invalid operand at position {}: '{:?}'", i, value);
                    })
            })
            .collect();
        Operation { operands, operator }
    }

    fn from_part1(value: &[&str]) -> Self {
        let operands: Vec<_> = value[..value.len() - 1]
            .iter()
            .map(|&s| {
                s.trim().parse::<i64>().unwrap_or_else(|_| {
                    panic!("Invalid operand: '{}', {:?}", s, value);
                })
            })
            .collect();
        let operator = Operator::from(value[value.len() - 1]);
        Operation { operands, operator }
    }
}

fn parse_tokens(rows: Vec<&str>) -> Vec<Vec<String>> {
    let lenghts: Vec<Vec<usize>> = rows
        .iter()
        .map(|&line| {
            let row_token_lenghts: Vec<usize> = line.split_whitespace().map(|s| s.len()).collect();
            row_token_lenghts
        })
        .collect();
    let column_sizes = (0..lenghts[0].len())
        .map(|i| lenghts.iter().map(|row| row[i]).max().unwrap_or(0))
        .collect::<Vec<usize>>();

    let tokens: Vec<Vec<String>> = rows
        .iter()
        .map(|&row| {
            let mut start = 0;
            let row_size = row.len();
            let row_tokens: Vec<String> = column_sizes
                .iter()
                .map(|&size| {
                    let token = &row[start..min(start + size, row_size)];
                    start += size + 1; // +1 for the space
                    format!("{:width$}", token, width = size)
                })
                .collect();
            row_tokens
        })
        .collect();
    tokens
}

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());

    let tokens = parse_tokens(lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>());

    let mut raw_operation: Vec<&str> = vec![""; tokens.len()];
    let mut operations: Vec<Operation> = Vec::with_capacity(tokens[0].len());

    let operation_factory = match args.part {
        1 => |v: &[&str]| Operation::from_part1(v),
        2 => |v: &[&str]| Operation::from_part2(v),
        _ => panic!("Part {} is not yet implemented", args.part),
    };

    #[allow(clippy::needless_range_loop)]
    for i in 0..tokens[0].len() {
        for j in 0..raw_operation.len() {
            raw_operation[j] = &tokens[j][i];
        }
        operations.push(operation_factory(raw_operation.as_slice()));
    }

    let result: i64 = operations.iter().map(|op| op.execute()).sum::<i64>();

    println!("Answer: {}", result);
}
