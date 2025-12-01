//! Solves day 1 of Advent of Code 2025
use core::num::Saturating;
use core::num::Wrapping;
use core::ops::Div as _;
use std::fs;
use std::path::Path;

/// Parses input for day1 puzzles into a vector containing the direction
/// character (`'L'` or `'R'`) and the number of ticks.
fn parse_input(input: &str) -> Vec<(char, u16)> {
    let mut vector: Vec<(char, u16)> = Vec::new();
    for line in input.lines() {
        let mut chars = line.trim().chars();
        let direction = match chars.next() {
            Some(chr) => chr,
            None => continue, // ignore empty lines
        };
        let number: u16 = match chars.collect::<String>().parse::<u16>() {
            Ok(x) => x,
            Err(error) => panic!("Could not parse line: {line}\nError: {error}"),
        };
        vector.push((direction, number));
    }
    vector
}

/// Counts zero crossings for part two of the puzzle.
/// More specifically: This counts how often a tick reaches zero when the dial
/// is turned, also including multiple 360° turns.
fn count_zero_crossings(instructions: Vec<(char, u16)>) -> Saturating<u16> {
    let mut dial: Wrapping<i16> = Wrapping(50);
    let mut zero_crossings: Saturating<u16> = Saturating(0);
    for (d, x) in instructions {
        let delta = match d {
            'R' => Wrapping(x.cast_signed()),
            'L' => -Wrapping(x.cast_signed()),
            _ => panic!("Incorrect direction character: {d}"),
        };
        // case 1: crossings due to a full turn
        zero_crossings += delta.0.unsigned_abs().div(100);
        // case 2: crossings due to a partial turn
        // Partial turns starting at 0 can never lead to a crossing
        // Otherwise, we can detect a crossing if they end up exactly
        // at zero or outside the [0, 99] range
        let remaining = Wrapping(delta.0.wrapping_rem(100)); // remaining partial turn
        if dial != Wrapping(0)
            && (dial + remaining <= Wrapping(0) || dial + remaining >= Wrapping(100))
        {
            zero_crossings += 1;
        }
        dial = Wrapping((dial + remaining).0.rem_euclid(100));
        // println!("Dial: {dial}, Zero crossings: {zero_crossings}");
    }
    zero_crossings
}

/// Counts how often the dial ends up at zero after a turn instruction.
#[allow(dead_code)]
fn count_zero_rests(instructions: Vec<(char, u16)>) -> i16 {
    let mut dial: i16 = 50;
    let mut zeros = 0;
    for (d, x) in instructions {
        dial += match d {
            'R' => x as i16,
            'L' => -(x as i16),
            _ => panic!("Incorrect direction character: {d}"),
        };
        dial = dial.rem_euclid(100);
        if dial == 0 {
            zeros += 1;
        }
        println!("Dial: {dial}");
    }
    zeros
}

/// Loads the file `input.txt` and prints the puzzle solution.
fn main() {
    let input_path: &Path = Path::new("input.txt");
    let contents = fs::read_to_string(input_path).expect("Test");
    let data = parse_input(&contents);
    let zeros = count_zero_crossings(data);
    println!("{zeros} zero crossings found!")
}
