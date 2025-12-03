//! Solution for day 3 of Advent of Code 2025
use std::fs;
use std::path::Path;
use std::process::exit;

use log::debug;
use log::error;
use log::info;

/// Parses puzzle input for day 3
fn parse_input(text: &str) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    for bank in text.lines() {
        debug!("Bank: {bank}");
        let batteries = bank
            .chars()
            .filter_map(|chr| chr.to_digit(10))
            .filter_map(|digit| u8::try_from(digit).ok())
            .collect::<Vec<u8>>();
        result.push(batteries);
    }
    result
}

fn main() {
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
