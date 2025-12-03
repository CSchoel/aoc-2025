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

/// Compute max joltage
///
/// # Examples
///
/// ```
/// assert_eq!(max_joltage(&[1, 2, 3, 4]), 34)
/// assert_eq!(max_joltage(&[8, 2, 3, 9]), 89)
/// ```
fn max_joltage(bank: &[u8]) -> u8 {
    // find first digit
    let first_range = bank.get(0..bank.len().saturating_sub(1));
    let first = first_range.and_then(|rng| rng.iter().enumerate().max_by_key(|&(_, digit)| *digit));
    let (first_idx, first_val) = match first {
        Some((idx, val)) => (idx, val),
        None => return 0,
    };
    let second_range = bank.get(first_idx.saturating_add(1)..bank.len());
    let second = second_range.and_then(|rng| rng.iter().max());
    let second_val = match second {
        Some(val) => *val,
        None => return 0,
    };
    debug!("Found max joltage {first_val}{second_val} for bank {bank:?}.");
    first_val.saturating_mul(10_u8).saturating_add(second_val)
}
/// Compute max joltage sum
fn sum_max_joltages(input: &[Vec<u8>]) -> u16 {
    input.iter().map(|x| u16::from(max_joltage(x))).sum()
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
    let result = sum_max_joltages(&input);
    info!("Result: {result}");
}
