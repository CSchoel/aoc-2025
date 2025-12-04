//! Solve day 4 of Advent of Code 2025
use std::{fs, path::Path, process::exit};

use log::{debug, error, info};

/// Parses puzzle input for day 4
fn parse_input(text: &str) -> Vec<Vec<char>> {
    text.lines()
        .map(|row| row.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn main() {
    env_logger::init();
    let input_path: &Path = Path::new("input.txt");
    let contents: String = match fs::read_to_string(input_path) {
        Ok(str) => str,
        Err(err) => {
            let input_disp = input_path.display();
            error!("Could not read {input_disp}!\nReason: Err({err})");
            exit(1)
        }
    };
    let input = parse_input(&contents);
    info!("Parsed input: {input:?}");
}
