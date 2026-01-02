//! Solves day 12 of Advent of Code 2025

use core::num::ParseIntError;
use std::{env::args, fs, path::Path, process::exit};

use log::info;
use regex::Regex;

/// Represents a present shape
#[derive(Debug, Clone)]
struct PresentShape {
    /// The pixels occupied by the present
    /// Outer index is width, inner index is length
    pixels: Vec<Vec<bool>>,
}

/// Represents a region under a tree and the requirements of presents that should be placed there
#[derive(Debug)]
struct TreeRegion {
    /// Length of the region
    length: usize,
    /// The shapes of presents to be placed in the region
    present_shapes: Vec<PresentShape>,
    /// How many of the individual shapes should be placed in the region
    shape_quantities: Vec<usize>,
    /// Width of the region
    width: usize,
}

/// Parses input for day 12
fn parse_input(content: &str) -> Result<Vec<TreeRegion>, String> {
    let error_mapper = |err: regex::Error| format!("Internal error: {err:?}");
    let pat_pixels = Regex::new(r"[#\.]+").map_err(error_mapper)?;
    let pat_region = Regex::new(r"(\d+)x(\d+)\:\s*((?:\d+\s*)+)").map_err(error_mapper)?;
    let mut pixels: Vec<Vec<bool>> = Vec::new();
    let mut present_shapes: Vec<PresentShape> = Vec::new();
    let mut regions = Vec::new();
    for line in content.lines() {
        if let Some(match_region) = pat_region.captures(line) {
            let (_, [length_str, width_str, quantities]) = match_region.extract();
            let length = length_str
                .parse::<usize>()
                .map_err(|err| format!("Could not parse length. Reason:\n{err:?}"))?;
            let width = width_str
                .parse::<usize>()
                .map_err(|err| format!("Could not parse width. Reason:\n{err:?}"))?;
            let shape_quantities = quantities
                .split_ascii_whitespace()
                .map(str::parse::<usize>)
                .collect::<Result<Vec<usize>, ParseIntError>>()
                .map_err(|err| format!("Could not parse shape quantities. Reason:\n{err:?}"))?;
            regions.push(TreeRegion {
                length,
                present_shapes: present_shapes.clone(),
                shape_quantities,
                width,
            });
        } else if pat_pixels.is_match(line) {
            let pixel_line = line.chars().map(|chr| chr == '#').collect::<Vec<bool>>();
            pixels.push(pixel_line);
        } else {
            if !pixels.is_empty() {
                present_shapes.push(PresentShape {
                    pixels: pixels.clone(),
                });
            }
            pixels = Vec::new();
        }
    }
    Ok(regions)
}

#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "This is a CLI function."
)]
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
    let input = match parse_input(&contents) {
        Ok(inp) => inp,
        Err(err) => {
            eprint!("Could not parse input! Reason:\n{err}");
            exit(1);
        }
    };
    info!("Parsed input: {input:?}");
    let result = "TBD";
    println!("Result: {result}");
}
