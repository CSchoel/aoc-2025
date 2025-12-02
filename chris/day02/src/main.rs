//! Solves day 2 of Advent of Code 2025

use std::{
    collections::btree_map::Range, fs, num::Saturating, ops::Div, path::Path, process::exit,
};

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

/// Find the sum of all invalid IDs
fn sum_invalid_ids(data: Vec<(u64, u64)>) -> u64 {
    let mut invalid: Saturating<u64> = Saturating(0);
    for (start, end) in data {
        println!("Start: {}, End: {}", start, end);
        for i in start..end + 1 {
            let str = i.to_string();
            if str.len() % 2 != 0 {
                continue;
            }
            let (left, right) = str.split_at(str.len().div(2));
            if left.eq(right) {
                invalid += i;
            }
        }
    }
    invalid.0
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
    let result = sum_invalid_ids(data);
    println!("Sum: {:?}", result);
}
