//! Solves day 8 of Advent of Code 2025

use std::{env::args, fs, num::ParseIntError, path::Path, process::exit};

use log::info;

/// Represents a position in 3D carthesian coordinates
#[derive(Debug)]
struct Position3D {
    /// Position on x axis
    x: usize,
    /// Position on y axis
    y: usize,
    /// Position on z axis
    z: usize,
}

fn parse_input(content: &str) -> Result<Vec<Position3D>, String> {
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
            let &[x, y, z] = numbers.as_slice() else {
                return Err(format!("Wrong number of coordinates: {numbers:?}"));
            };
            Ok(Position3D { x, y, z })
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
    let input = parse_input(&contents);
    info!("Parsed input: {input:?}");
    println!("Result: TBD");
}
