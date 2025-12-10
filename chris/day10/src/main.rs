//! Solves day 10 of Advent of Code 2025

use std::{fs, num::ParseIntError, path::Path, process::exit};

use log::info;
use regex::Regex;

#[derive(Debug)]
struct IndicatorLight {
    active: bool,
    should_be_active: bool,
}

#[derive(Debug)]
struct ButtonWiring {
    toggled_lights: Vec<usize>,
}

#[derive(Debug)]
struct FactoryMachine {
    indicator_lights: Vec<IndicatorLight>,
    required_joltage: Vec<u32>,
    buttons: Vec<ButtonWiring>,
}

fn parse_usize_list(text: &str) -> Result<Vec<usize>, ParseIntError> {
    text.split(',')
        .map(|button_str| button_str.trim().parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()
}

fn parse_u32_list(text: &str) -> Result<Vec<u32>, ParseIntError> {
    text.split(',')
        .map(|button_str| button_str.trim().parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
}

impl ButtonWiring {
    fn from_str(text: &str) -> Result<Vec<Self>, &str> {
        let Ok(pattern) = Regex::new(r"\(((?:\d,?\s*)+)\)") else {
            return Err("Internal error: Invalid regex");
        };
        pattern
            .captures_iter(text)
            .map(|button_str| {
                button_str
                    .get(1)
                    .ok_or("Could not find capture group!")
                    .and_then(|cap| {
                        parse_usize_list(cap.as_str())
                            .map_err(|_| "Could not parse toogled buttons!")
                            .and_then(|lst| {
                                Ok(ButtonWiring {
                                    toggled_lights: lst,
                                })
                            })
                    })
            })
            .collect::<Result<Vec<ButtonWiring>, &str>>()
    }
}

impl IndicatorLight {
    fn from_str(text: &str) -> Vec<Self> {
        text.chars()
            .map(|chr| IndicatorLight {
                active: false,
                should_be_active: chr == '#',
            })
            .collect()
    }
}

fn parse_input(content: &str) -> Result<Vec<FactoryMachine>, &str> {
    let pattern =
        Regex::new(r"\[([.#]+)\] ((?:\((?:\d+,?\s*)+\)\s*)+) \{((?:\d+,?\s*)+)\}").unwrap();
    content
        .lines()
        .map(|line| {
            let Some(cap) = pattern.captures(line) else {
                return Err("Line {line} did not match");
            };
            let (_, [indicators_str, buttons_str, joltages_str]) = cap.extract();
            let indicators = IndicatorLight::from_str(indicators_str);
            let buttons = ButtonWiring::from_str(buttons_str);
            let joltages = parse_u32_list(joltages_str);
            let (Ok(buttons_ok), Ok(joltages_ok)) = (buttons, joltages) else {
                return Err("Could not parse buttons or joltages");
            };
            Ok(FactoryMachine {
                indicator_lights: indicators,
                buttons: buttons_ok,
                required_joltage: joltages_ok,
            })
        })
        .collect::<Result<Vec<FactoryMachine>, &str>>()
}

#[expect(clippy::print_stdout, reason = "This is a CLI function.")]
#[expect(clippy::print_stderr, reason = "This is a CLI function.")]
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
    let Ok(input) = parse_input(&contents) else {
        eprintln!("Could not parse input {contents}");
        exit(1);
    };
    info!("Parsed input: {input:?}");
    println!("foo");
}
