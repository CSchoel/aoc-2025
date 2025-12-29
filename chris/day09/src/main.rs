//! Solves day 9 of Advent of Code 2025
use core::num::ParseIntError;
use std::{env::args, fs, path::Path, process::exit};

use log::info;

/// Represents a 2D carthesian coordinate of a tile
#[derive(Debug)]
struct Position2D {
    /// Position on x-axis
    x: usize,
    /// Position on y-axis
    y: usize,
}

/// Parses input for day 9
fn parse_input(content: &str) -> Result<Vec<Position2D>, String> {
    content
        .lines()
        .map(|line| {
            let Ok(numbers) = line
                .split(',')
                .map(str::parse::<usize>)
                .collect::<Result<Vec<usize>, ParseIntError>>()
            else {
                return Err(format!("Could not parse numbers in line {line}"));
            };
            let &[x, y] = numbers.as_slice() else {
                return Err(format!("Wrong number of coordinates: {numbers:?}"));
            };
            Ok(Position2D { x, y })
        })
        .collect()
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
    let input = match parse_input(&contents) {
        Ok(inp) => inp,
        Err(err) => {
            eprint!("Could not parse input! Reason:\n{err}");
            exit(1);
        }
    };
    info!("Parsed input: {input:?}");
    //let result = count_connected(&input, 1000);
    println!("Result: TBD");
}
