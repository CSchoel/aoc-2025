//! Solves day 7 of Advent of code 2025

use std::{collections::HashSet, env::args, fs, path::Path, process::exit};

use log::info;

/// Represents a tachyon manifold
#[derive(Debug)]
struct TachyonManifold {
    /// Splitter positions per row, starting from the top
    splitter_positions: Vec<HashSet<usize>>,
    /// Starting column
    start_pos: usize,
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

    TachyonManifold {
        splitter_positions,
        start_pos,
    }
}

/// Counts the number of splits that the beam will encounter
fn count_splits(input: &TachyonManifold) -> usize {
    let mut beam_columns: HashSet<usize> = HashSet::new();
    let mut split: usize = 0;
    beam_columns.insert(input.start_pos);
    for splitters in &input.splitter_positions {
        // compare beam positions with splitters
        let mut new_beam_columns = HashSet::new();
        for beam_index in &beam_columns {
            if splitters.contains(beam_index) {
                split = split.saturating_add(1);
                new_beam_columns.insert(beam_index.saturating_sub(1));
                new_beam_columns.insert(beam_index.saturating_add(1));
            } else {
                new_beam_columns.insert(*beam_index);
            }
        }
        beam_columns = new_beam_columns;
    }
    split
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
    let result = count_splits(&input);
    println!("Result: {result}");
}
