use std::env;
use std::fmt::Error;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;
use std::string;

fn parse_input(input: String) -> Vec<(char, u8)> {
    let mut vector: Vec<(char, u8)> = Vec::new();
    for l in input.lines() {
        let mut chars = l.trim().chars();
        let direction = match chars.next() {
            Some(c) => c,
            None => continue, // ignore empty lines
        };
        let number: u8 = match chars.collect::<String>().parse::<u8>() {
            Ok(x) => x,
            Err(error) => panic!("Could not parse line: {l}\nError: {error}"),
        };
        vector.push((direction, number));
    }
    vector
}

fn count_zero_rests(instructions: Vec<(char, u8)>) -> i16 {
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

fn main() {
    let input_path: &Path = Path::new("input.txt");
    let contents = fs::read_to_string(input_path).expect("Test");
    let data = parse_input(contents);
    let zeros = count_zero_rests(data);
    println!("{zeros} zero crossings found!")
}
