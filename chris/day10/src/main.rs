//! Solves day 10 of Advent of Code 2025

use core::num::ParseIntError;
use std::{env::args, fs, path::Path, process::exit};

use log::info;
use regex::Regex;

/// Represents an indicator light with the current and the desired state
#[derive(Debug)]
struct IndicatorLight {
    /// Wether the battery is currently active
    active: bool,
    /// Whether the battery should be active
    should_be_active: bool,
}

/// Represents a button wiring, defining which indicator lights are toggled by a button
#[derive(Debug)]
struct ButtonWiring {
    /// Indices (0-based) of indicator lights that are toggled by this button
    toggled_lights: Vec<usize>,
}

/// Represents a full factory machine with indicator lights, required joltage and buttons
#[derive(Debug)]
struct FactoryMachine {
    /// The indicator lights of the machine
    indicator_lights: Vec<IndicatorLight>,
    /// The required joltages of the machine
    required_joltage: Vec<u32>,
    /// The button wirings of the machine
    buttons: Vec<ButtonWiring>,
}

impl ButtonWiring {
    /// Parse a `ButtonWiring` from a string representation (e.g. `"(17, 8)"`)
    fn from_str(text: &str) -> Result<Vec<Self>, String> {
        let Ok(pattern) = Regex::new(r"\(((?:\d,?\s*)+)\)") else {
            return Err("Internal error: Invalid regex. This should never happen!".to_owned());
        };
        pattern
            .captures_iter(text)
            .map(|button_capture| {
                let button_str = button_capture
                    .get(1)
                    .ok_or_else(|| {
                        format!("Could not find capture group!\nCapture: {button_capture:?}")
                    })?
                    .as_str();
                let lst = parse_usize_list(button_str)
                    .map_err(|err| format!("Could not parse toogled buttons!\nReason: {err:?}"))?;
                Ok(Self {
                    toggled_lights: lst,
                })
            })
            .collect::<Result<Vec<Self>, String>>()
    }
}

impl IndicatorLight {
    /// Parses a `IndicatorLight` configuration from a string (e.g. `"#..##"`)
    fn from_str(text: &str) -> Vec<Self> {
        text.chars()
            .map(|chr| Self {
                active: false,
                should_be_active: chr == '#',
            })
            .collect()
    }
}

/// Parses a list of usize
fn parse_usize_list(text: &str) -> Result<Vec<usize>, ParseIntError> {
    text.split(',')
        .map(|button_str| button_str.trim().parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()
}

/// Parses a list of comma-separated numbers as u32
fn parse_u32_list(text: &str) -> Result<Vec<u32>, ParseIntError> {
    text.split(',')
        .map(|button_str| button_str.trim().parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
}

/// Parses input for day 10 (e.g. `"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"`)
fn parse_input(content: &str) -> Result<Vec<FactoryMachine>, String> {
    let Ok(pattern) = Regex::new(r"\[([.#]+)\] ((?:\((?:\d+,?\s*)+\)\s*)+) \{((?:\d+,?\s*)+)\}")
    else {
        return Err("Internal error in regex definition. This should never happen!".to_owned());
    };
    content
        .lines()
        .map(|line| {
            let Some(cap) = pattern.captures(line) else {
                return Err(format!("Line {line} did not match"));
            };
            let (_, [indicators_str, buttons_str, joltages_str]) = cap.extract();
            let indicators = IndicatorLight::from_str(indicators_str);
            let buttons = ButtonWiring::from_str(buttons_str);
            let joltages = parse_u32_list(joltages_str);
            let (Ok(buttons_ok), Ok(joltages_ok)) = (buttons, joltages) else {
                return Err("Could not parse buttons or joltages".to_owned());
            };
            Ok(FactoryMachine {
                indicator_lights: indicators,
                buttons: buttons_ok,
                required_joltage: joltages_ok,
            })
        })
        .collect::<Result<Vec<FactoryMachine>, String>>()
}

#[expect(clippy::print_stdout, reason = "This is a CLI function.")]
#[expect(clippy::print_stderr, reason = "This is a CLI function.")]
fn main() {
    env_logger::init();
    let input_path_str = args()
        .nth(1)
        .unwrap_or_else(|| "sample_input.txt".to_owned());
    let input_path = Path::new(&input_path_str);
    let contents: String = match fs::read_to_string(input_path) {
        Ok(str) => str,
        Err(err) => {
            let input_disp = input_path.display();
            eprintln!("Could not read {input_disp}!\nReason: Err({err})");
            exit(1);
        }
    };
    let Ok(input) = parse_input(&contents) else {
        eprintln!("Could not parse input {contents}");
        exit(1);
    };
    info!("Parsed input: {input:?}");
    println!("foo");
}
