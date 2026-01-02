//! Solves day 12 of Advent of Code 2025

use std::{env::args, fs, path::Path, process::exit};

use log::info;

/// Represents a present shape
#[derive(Debug, Clone)]
struct PresentShape {
    /// The pixels occupied by the present
    /// Outer index is width, inner index is length
    pixels: Vec<Vec<bool>>,
}

/// Represents a region under a tree and the requirements of presents that should be placed there
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
    Err(format!("Not yet implemented!"))
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
    // let input = match parse_input(&contents) {
    //     Ok(inp) => inp,
    //     Err(err) => {
    //         eprint!("Could not parse input! Reason:\n{err}");
    //         exit(1);
    //     }
    // };
    // info!("Parsed input: {input:?}");
    let result = "TBD";
    println!("Result: {result}");
}
