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
/// assert_eq!(max_joltage(&[1, 2, 3, 4]), 34);
/// assert_eq!(max_joltage(&[8, 2, 3, 9]), 89);
/// ```
fn max_joltage(bank: &[u8], num_active: u8) -> u64 {
    debug!("Calculating max joltage for {bank:?}");
    let mut offset = 0;
    let mut active: Vec<u8> = Vec::new();
    for remaining_batteries in (0..num_active).rev() {
        let search_range =
            bank.get(offset..bank.len().saturating_sub(usize::from(remaining_batteries)));
        debug!("Search range: {search_range:?}");
        let first_highest = search_range.and_then(|rng| {
            rng.iter()
                .enumerate()
                .rev()
                .max_by_key(|&(_, digit)| *digit)
        });
        let (highest_idx, highest_val) = match first_highest {
            Some((idx, val)) => (idx, val),
            None => return 0,
        };
        debug!("Select battery to activate: Index {highest_idx}, value {highest_val}");
        active.push(*highest_val);
        offset = offset.saturating_add(highest_idx).saturating_add(1);
    }
    debug!("Active battery values: {active:?}");
    let max_joltage = match active
        .iter()
        .map(u8::to_string)
        .collect::<String>()
        .parse::<u64>()
    {
        Ok(max_val) => max_val,
        Err(_) => return 0,
    };
    debug!("Found max joltage {max_joltage} for bank {bank:?}.");
    max_joltage
}

/// Compute max joltage sum
fn sum_max_joltages(input: &[Vec<u8>], num_active: u8) -> u64 {
    input.iter().map(|x| max_joltage(x, num_active)).sum()
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
    let result = sum_max_joltages(&input, 12);
    info!("Result: {result}");
}
