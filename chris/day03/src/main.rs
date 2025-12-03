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
    debug!("Calculating max joltage for {bank:?}");
    // find first digit
    let first_range = bank.get(0..bank.len().saturating_sub(1));
    let first = first_range.and_then(|rng| {
        rng.iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, digit)| *digit)
    });
    let (first_idx, first_val) = match first {
        Some((idx, val)) => (idx, val),
        None => return 0,
    };
    debug!("First index: {first_idx}, First value: {first_val}");
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
    // let test = max_joltage(&[
    //     1, 1, 2, 2, 3, 2, 8, 3, 7, 7, 2, 2, 7, 5, 6, 5, 2, 9, 4, 5, 7, 3, 5, 2, 5, 6, 6, 8, 2, 4,
    //     6, 2, 5, 2, 9, 2, 5, 1, 6, 4, 2, 4, 6, 2, 8, 7, 9, 8, 7, 7, 8, 4, 1, 2, 2, 6, 9, 3, 9, 8,
    //     3, 8, 6, 9, 9, 2, 2, 2, 2, 2, 5, 7, 6, 4, 7, 3, 2, 3, 8, 2, 2, 2, 3, 6, 7, 7, 2, 5, 2, 2,
    //     9, 2, 9, 2, 4, 7, 6, 7, 7, 6,
    // ]);
    // info!("Test result: {test}");
    let result = sum_max_joltages(&input);
    info!("Result: {result}");
}
