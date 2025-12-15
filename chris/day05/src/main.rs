//! Solves day 5 of Advent of Code 2025

use core::num::ParseIntError;
use std::{fs, num::Saturating, path::Path, process::exit};

use log::info;

/// Ranges of fresh ingredients
type FreshRanges = Vec<(u64, u64)>;

/// Parses input for day 5
fn parse_input(content: &str) -> Result<(FreshRanges, Vec<u64>), String> {
    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    for line in content.lines() {
        if line.contains('-') {
            // parse new fresh range
            let Ok(range) = line
                .split('-')
                .map(str::parse::<u64>)
                .collect::<Result<Vec<u64>, ParseIntError>>()
            else {
                return Err(format!("Could not parse fresh range from {line}"));
            };
            let Ok([start, end]): Result<[u64; 2], _> = range.try_into() else {
                return Err(format!("Could not unpack fresh range from {line}"));
            };
            fresh_ranges.push([start, end].into());
        } else if !line.is_empty() {
            // parse new ingredient
            let Ok(ingredient) = line.parse::<u64>() else {
                return Err(format!("Could not parse u64 from {line}"));
            };
            ingredients.push(ingredient);
        } else {
            // Skip empty line
        }
    }
    Ok((fresh_ranges, ingredients))
}

/// Counts the number of ingredient IDs that are fresh
fn count_fresh_ingredients(fresh_ranges: &FreshRanges, ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| {
            fresh_ranges
                .iter()
                .any(|&(start, end)| ingredient >= &&start && ingredient <= &&end)
        })
        .count()
}

/// Counts the number of ingredient IDs in any fresh range
fn count_all_fresh_ids(fresh_ranges: &FreshRanges) -> u64 {
    let mut sorted_ranges = fresh_ranges.clone();
    sorted_ranges.sort_by(|&(start1, _), &(start2, _)| start1.cmp(&start2));
    // add sentinel
    sorted_ranges.push((u64::MAX, u64::MAX - 1));
    let mut current_start = 0;
    let mut current_end = 0;
    let mut fresh_ids: Saturating<u64> = Saturating(0);
    for (start, end) in sorted_ranges {
        if start <= current_end {
            // start is still inside the current range => only update end
            current_end = end.max(current_end);
        } else {
            if (current_start, current_end) != (0, 0) {
                let increment = Saturating(current_end) - Saturating(current_start) + Saturating(1);
                fresh_ids += increment;
                info!(
                    "Found {increment} new fresh IDs in merged range {current_start}-{current_end}."
                );
            }
            current_start = start;
            current_end = end;
        }
    }
    fresh_ids.0
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
    let Ok((fresh_ranges, ingredients)) = parse_input(&contents) else {
        eprintln!("Could not parse input {contents}");
        exit(1);
    };
    info!("Parsed input: {fresh_ranges:?}, {ingredients:?}");
    let fresh = count_fresh_ingredients(&fresh_ranges, &ingredients);
    println!("Found {fresh} ingredients with fresh ingredient IDs.");
    let all_fresh = count_all_fresh_ids(&fresh_ranges);
    println!("Found {all_fresh} fresh ingredient IDs.");
}
