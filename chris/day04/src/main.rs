//! Solve day 4 of Advent of Code 2025
use std::{fs, num::Saturating, path::Path, process::exit};

use log::{debug, error, info};

/// Represents a 2D map built from characters
#[derive(Debug)]
struct CharMatrix {
    /// Number of columns
    columns: usize,
    /// The actual matrix data in order from left to right and top to bottom
    matrix: Vec<char>,
}

impl CharMatrix {
    /// Get element at position or None
    fn at(&self, row: usize, col: usize) -> Option<char> {
        let s_row = Saturating(row);
        let s_col = Saturating(col);
        if col >= self.columns || row >= self.rows() {
            return None;
        }
        let idx = s_row * Saturating(self.columns) + s_col;

        self.matrix.get(idx.0).copied()
    }

    /// Get number of rows
    const fn rows(&self) -> usize {
        self.matrix.len().saturating_div(self.columns)
    }
}

/// Parses puzzle input for day 4
fn parse_input(text: &str) -> Option<CharMatrix> {
    let columns = text.lines().next()?.len();
    let matrix = text
        .lines()
        .flat_map(|row| row.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();
    Some(CharMatrix { columns, matrix })
}

fn main() {
    env_logger::init();
    let input_path: &Path = Path::new("input.txt");
    let contents: String = match fs::read_to_string(input_path) {
        Ok(str) => str,
        Err(err) => {
            let input_disp = input_path.display();
            error!("Could not read {input_disp}!\nReason: Err({err})");
            exit(1)
        }
    };
    let input = parse_input(&contents);
    info!("Parsed input: {input:?}");
}
