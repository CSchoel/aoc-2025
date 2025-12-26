//! Solves day 6 of Advent of Code 2025

use core::iter::zip;
use core::num::ParseIntError;
use std::{env::args, fs, path::Path, process::exit};

use log::info;

/// A math problem consisting of a list of numbers and an operator (+ or *)
type MathProblem = (Vec<u32>, char);

/// Parses input
/// This assumes that each line in the input except for the last one contains the
/// same amount of numbers while the last one contains the same number of '*' or '+' symbols.
fn parse_input(content: &str) -> Result<Vec<MathProblem>, String> {
    let mut result: Vec<(Vec<u32>, char)> = Vec::new();
    for line in content.lines() {
        enum LineType {
            /// Represents a line containing (unsigned) numbers
            Number,
            /// Represents a line containing the operators + and *
            Operator,
        }
        let elements = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let line_type = match elements.first() {
            None => continue, // ignore empty line
            Some(first_el) => match first_el.chars().next() {
                None => continue,
                Some('+' | '*') => LineType::Operator,
                Some(_) => LineType::Number,
            },
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
                    math_problem.1 = op_char;
                }
            }
            LineType::Number => {
                // add a new line of numbers
                let Ok(numbers) = elements
                    .iter()
                    .map(|x| x.parse::<u32>())
                    .collect::<Result<Vec<u32>, ParseIntError>>()
                else {
                    return Err(format!("Could not parse numbers from line {line}"));
                };
                // extend result array if it is empty
                if result.is_empty() {
                    for num in numbers {
                        result.push((vec![num], '#'));
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
    let Ok(input) = parse_input(&contents) else {
        eprintln!("Could not parse input {contents}");
        exit(1);
    };
    info!("Parsed input: {input:?}");
    // println!("Found {all_fresh} fresh ingredient IDs.");
}
