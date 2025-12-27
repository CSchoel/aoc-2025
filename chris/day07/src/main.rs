//! Solves day 7 of Advent of code 2025

use std::{env::args, fs, path::Path, process::exit};

use log::info;

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
    let Ok(input) = parse_input(&contents) else {
        eprintln!("Could not parse input {contents}");
        exit(1);
    };
    info!("Parsed input: {input:?}");
    println!("Result: TBD");
}
