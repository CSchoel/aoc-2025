//! Solve day 4 of Advent of Code 2025
use std::{
    fs,
    num::Saturating,
    ops::{Div, Rem},
    path::Path,
    process::exit,
};

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

    /// Get element at position or None, allow negative inputs
    fn at_signed(&self, row: isize, col: isize) -> Option<char> {
        if row < 0 || col < 0 {
            return None;
        }
        self.at(row as usize, col as usize)
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

/// Solves part 1 of the puzzle
fn count_movable(mat: &CharMatrix) -> usize {
    (1..mat.matrix.len())
        .map(|idx| (idx.div_euclid(mat.columns), idx.rem_euclid(mat.columns)))
        .filter(|&(row, col)| {
            let neighbors = (-1_isize..2_isize)
                .flat_map(|dr| {
                    (-1_isize..2_isize).map(move |dc| {
                        if dr == 0 && dc == 0 {
                            return '.';
                        }
                        let neighbor_row = match isize::try_from(row) {
                            Ok(idx) => idx.saturating_add(dr),
                            Err(_) => return '.',
                        };
                        let neighbor_col = match isize::try_from(col) {
                            Ok(idx) => idx.saturating_add(dc),
                            Err(_) => return '.',
                        };
                        mat.at_signed(neighbor_row, neighbor_col).unwrap_or('.')
                    })
                })
                .filter(|chr| *chr == '@')
                .count();
            neighbors <= 4
        })
        .count()
}

#[expect(clippy::print_stdout, reason = "This is a CLI function")]
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
    let input = parse_input(&contents).unwrap_or_else(|| {
        error!("Could not parse input {contents}");
        exit(1);
    });
    info!("Parsed input: {input:?}");
    let result = count_movable(&input);
    println!("Result: {result}");
}
