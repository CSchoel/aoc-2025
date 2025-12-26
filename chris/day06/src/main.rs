//! Solves day 6 of Advent of Code 2025

use core::iter::zip;
use core::num::ParseIntError;
use std::{env::args, fs, path::Path, process::exit};

use log::info;

/// An operator for combining multiple numbers
#[derive(Debug, Clone, Copy)]
enum Operator {
    /// Addition
    Add,
    /// Multiplication
    Mul,
}

/// A math problem consisting of a list of numbers and an operator (+ or *)
type MathProblem = (Vec<u64>, Operator);

/// Type of an input line
enum LineType {
    /// Represents a line containing (unsigned) numbers
    Number,
    /// Represents a line containing the operators + and *
    Operator,
}

/// Finds out the line type of a line in the input
fn line_type(line: &str) -> Option<LineType> {
    match line.split_ascii_whitespace().next()?.chars().next()? {
        '+' | '*' => Some(LineType::Operator),
        _ => Some(LineType::Number),
    }
}

/// Parses input
/// This assumes that each line in the input except for the last one contains the
/// same amount of numbers while the last one contains the same number of '*' or '+' symbols.
fn parse_input(content: &str) -> Result<Vec<MathProblem>, String> {
    let mut result: Vec<MathProblem> = Vec::new();
    for line in content.lines() {
        let elements = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let Some(line_type) = line_type(line) else {
            continue;
        };
        match line_type {
            LineType::Operator => {
                // set the operators
                for (op_as_str, math_problem) in zip(elements, result.iter_mut()) {
                    let Some(op_char) = op_as_str.chars().next() else {
                        return Err(format!(
                            "Encountered empty op_char for problem {math_problem:?}"
                        ));
                    };
                    math_problem.1 = match op_char {
                        '+' => Operator::Add,
                        '*' => Operator::Mul,
                        _ => return Err(format!("Unknown operator {op_char:?} encountered!")),
                    }
                }
            }
            LineType::Number => {
                // add a new line of numbers
                let Ok(numbers) = elements
                    .iter()
                    .map(|x| x.parse::<u64>())
                    .collect::<Result<Vec<u64>, ParseIntError>>()
                else {
                    return Err(format!("Could not parse numbers from line {line}"));
                };
                // extend result array if it is empty
                if result.is_empty() {
                    for num in numbers {
                        result.push((vec![num], Operator::Add));
                    }
                } else {
                    for (num, math_problem) in zip(numbers, result.iter_mut()) {
                        math_problem.0.push(num);
                    }
                }
            }
        }
    }
    Ok(result)
}

/// Parses input for part 2
fn parse_input2(content: &str) -> Result<Vec<MathProblem>, String> {
    let random_access = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let rows = random_access.len();
    let cols = random_access.iter().map(Vec::len).max().unwrap_or(0);
    let mut result = Vec::new();
    let empty = Vec::new();
    let mut current_operator = Operator::Add;
    let mut current_numbers: Vec<u64> = Vec::new();
    for col in 0..=cols {
        let digits = (0..rows.saturating_sub(1_usize))
            .map(|digit_row| {
                random_access
                    .get(digit_row)
                    .unwrap_or(&empty)
                    .get(col)
                    .unwrap_or(&' ')
            })
            .collect::<String>();
        let operator = random_access
            .get(rows.saturating_sub(1_usize))
            .unwrap_or(&empty)
            .get(col)
            .unwrap_or(&' ');
        if operator != &' ' {
            current_operator = match *operator {
                '+' => Operator::Add,
                '*' => Operator::Mul,
                _ => return Err(format!("Unknown operator {operator}!")),
            };
        }
        // Check if we have an empty column
        if digits.trim().is_empty() && operator == &' ' {
            result.push((current_numbers.clone(), current_operator));
            current_numbers = Vec::new();
        } else {
            let Ok(number) = digits.trim().parse::<u64>() else {
                return Err(format!("Could not parse number from {digits}"));
            };
            current_numbers.push(number);
        }
    }
    Ok(result)
}

/// Solves all math problems in a list
fn solve_math_problems(problems: &[MathProblem]) -> u64 {
    problems
        .iter()
        .map(|problem| match problem.1 {
            Operator::Add => problem.0.iter().sum::<u64>(),
            Operator::Mul => problem.0.iter().product(),
        })
        .sum()
}

#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "This is a CLI function."
)]
fn main() {
    env_logger::init();
    let input_path_str = args()
        .nth(1)
        .unwrap_or_else(|| "sample_input.txt".to_owned());
    let input_path = Path::new(&input_path_str);
    let contents: String = match fs::read_to_string(input_path) {
        Ok(str) => str,
        Err(err) => {
            let input_disp = input_path.display();
            eprintln!("Could not read {input_disp}!\nReason: Err({err})");
            exit(1);
        }
    };
    let Ok(input) = parse_input2(&contents) else {
        eprintln!("Could not parse input {contents}");
        exit(1);
    };
    info!("Parsed input: {input:?}");
    let result = solve_math_problems(&input);
    println!("Result: {result}");
}
