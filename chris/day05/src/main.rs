//! Solves day 5 of Advent of Code 2025

use core::num::ParseIntError;
use std::{fs, path::Path, process::exit};

use log::info;

/// Ranges of fresh ingredients
type FreshRanges = Vec<(u32, u32)>;

/// Parses input for day 5
fn parse_input(content: &str) -> Result<(FreshRanges, Vec<u32>), String> {
    let mut fresh_ranges: Vec<(u32, u32)> = Vec::new();
    let mut ingredients: Vec<u32> = Vec::new();
    for line in content.lines() {
        if line.contains('-') {
            // parse new fresh range
            let Ok(range) = line
                .split('-')
                .map(str::parse::<u32>)
                .collect::<Result<Vec<u32>, ParseIntError>>()
            else {
                return Err(format!("Could not parse fresh range from {line}"));
            };
            let Ok([start, end]): Result<[u32; 2], _> = range.try_into() else {
                return Err(format!("Could not unpack fresh range from {line}"));
            };
            fresh_ranges.push([start, end].into());
        } else if !line.is_empty() {
            // parse new ingredient
            let Ok(ingredient) = line.parse::<u32>() else {
                return Err(format!("Could not parse u32 from {line}"));
            };
            ingredients.push(ingredient);
        } else {
            // Skip empty line
        }
    }
    Ok((fresh_ranges, ingredients))
}

#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "This is a CLI function."
)]
fn main() {
    env_logger::init();
    let input_path: &Path = Path::new("input.txt");
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
    println!("Hello, world!");
}
