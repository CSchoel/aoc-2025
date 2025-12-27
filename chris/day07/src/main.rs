//! Solves day 7 of Advent of code 2025

use std::{collections::HashSet, env::args, fs, path::Path, process::exit};

use log::info;

/// Represents a tachyon manifold
#[derive(Debug)]
struct TachyonManifold {
    /// Starting column
    start_pos: usize,
    /// Splitter positions per row, starting from the top
    splitter_positions: Vec<HashSet<usize>>,
}

/// Parses input string containing of a single line marking a starting position ('S')
/// and multiple lines that can have splitters ('^'). Empty space is marked with a '.'.
/// This ignores lines that only have empty space.
fn parse_input(content: &str) -> TachyonManifold {
    let mut start_pos: usize = 0;
    let mut splitter_positions: Vec<HashSet<usize>> = Vec::new();
    for line in content.lines() {
        // skip empty lines
        if line.chars().all(|chr| chr == '.') {
            continue;
        }
        // set start pos if it exists
        if let Some(s_idx) = line.find('S') {
            start_pos = s_idx;
            continue;
        }
        // not empty, no start => line with splitters
        let positions = line
            .chars()
            .enumerate()
            .filter_map(|(idx, chr)| (chr == '^').then_some(idx))
            .collect::<HashSet<usize>>();
        splitter_positions.push(positions);
    }
    let manifold = TachyonManifold {
        start_pos,
        splitter_positions,
    };
    manifold
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
    let input = parse_input(&contents);
    info!("Parsed input: {input:?}");
    println!("Result: TBD");
}
