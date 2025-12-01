use std::fs;
use std::path::Path;

fn parse_input(input: String) -> Vec<(char, u16)> {
    let mut vector: Vec<(char, u16)> = Vec::new();
    for l in input.lines() {
        let mut chars = l.trim().chars();
        let direction = match chars.next() {
            Some(c) => c,
            None => continue, // ignore empty lines
        };
        let number: u16 = match chars.collect::<String>().parse::<u16>() {
            Ok(x) => x,
            Err(error) => panic!("Could not parse line: {l}\nError: {error}"),
        };
        vector.push((direction, number));
    }
    vector
}

fn count_zero_crossings(instructions: Vec<(char, u16)>) -> i16 {
    let mut dial: i16 = 50;
    let mut zero_crossings = 0;
    for (d, x) in instructions {
        let delta = match d {
            'R' => x as i16,
            'L' => -(x as i16),
            _ => panic!("Incorrect direction character: {d}"),
        };
        // case 1: crossings due to a full turn
        zero_crossings += delta.abs() / 100;
        // case 2: crossings due to a partial turn
        // Partial turns starting at 0 can never lead to a crossing
        // Otherwise, we can detect a crossing if they end up exactly
        // at zero or outside the [0, 99] range
        let remaining = delta % 100; // remaining partial turn
        if dial != 0 && (dial + remaining <= 0 || dial + remaining >= 100) {
            zero_crossings += 1;
        }
        dial = (dial + remaining).rem_euclid(100);
        println!("Dial: {dial}, Zero crossings: {zero_crossings}");
    }
    zero_crossings
}

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

fn main() {
    let input_path: &Path = Path::new("input.txt");
    let contents = fs::read_to_string(input_path).expect("Test");
    let data = parse_input(contents);
    let zeros = count_zero_crossings(data);
    println!("{zeros} zero crossings found!")
}
