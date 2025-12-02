//! Solves day 2 of Advent of Code 2025

use core::num::Saturating;
use core::ops::Div as _;
use std::fs;
use std::path::Path;

/// Parses puzzle input for day 2
fn parse_input(text: &str) -> Result<Vec<(u64, u64)>, &str> {
    let mut result: Vec<(u64, u64)> = Vec::new();
    for range in text.split(',') {
        println!("Range: {:?}", range);
        let mut split = range.split('-');
        let start = match split.next().and_then(|s| s.parse::<u64>().ok()) {
            Some(x) => x,
            None => return Err("Malformed line"),
        };
        println!("Start: {:?}", start);
        let end = match split.next().and_then(|s| s.parse::<u64>().ok()) {
            Some(x) => x,
            None => return Err("Malformed line"),
        };
        result.push((start, end));
        println!("End: {:?}", end);
    }
    Ok(result)
}

fn divisors(num: u32) -> Vec<u32> {
    (1..((num as f64).sqrt() as u32))
        .filter(|x| num % x == 0)
        .flat_map(|x| vec![x, num.div(x)])
        .collect::<Vec<u32>>()
}

/// Tests if an ID is invalid
fn is_invalid(num: u64, all_lengths: bool) -> bool {
    let str = num.to_string();
    let pattern_lengths = if !all_lengths {
        if str.len() % 2 == 0 {
            vec![2]
        } else {
            Vec::new()
        }
    } else {
        divisors(str.len() as u32)
    };
    for length in pattern_lengths {
        let chars = str.chars().collect::<Vec<char>>();
        let mut is_repeating = true;
        for start in (length as usize..str.len()).step_by(length as usize) {
            if !chars[start..(start + length as usize)].eq(&chars[0..length as usize]) {
                is_repeating = false;
                break;
            }
        }
        if is_repeating {
            return true;
        }
    }
    false
}

/// Find the sum of all invalid IDs
fn sum_invalid_ids(data: Vec<(u64, u64)>, all_lengths: bool) -> u64 {
    let mut invalid: Saturating<u64> = Saturating(0);
    for (start, end) in data {
        println!("Start: {}, End: {}", start, end);
        for i in start..end + 1 {
            if is_invalid(i, all_lengths) {
                invalid += i
            }
        }
    }
    invalid.0
}

/// Find the sum of all invalid IDs only considering IDs which consist of
/// two repeated patterns.
fn sum_invalid_ids_half_length(data: Vec<(u64, u64)>) -> u64 {
    sum_invalid_ids(data, false)
}

/// Find sum of all invalid IDs considering repeating patterns of any length.
fn sum_invalid_ids_all_lengths(data: Vec<(u64, u64)>) -> u64 {
    sum_invalid_ids(data, true)
}

/// Loads the file `input.txt` and prints the puzzle solution.
fn main() {
    let input_path: &Path = Path::new("input.txt");
    let contents: String = match fs::read_to_string(input_path) {
        Ok(str) => str,
        Err(err) => panic!("Malformed input {err:?}"),
    };
    let data = match parse_input(&contents) {
        Ok(x) => x,
        Err(msg) => panic!("Wrong input!"),
    };
    let result = sum_invalid_ids_all_lengths(data);
    println!("Sum: {:?}", result);
}
