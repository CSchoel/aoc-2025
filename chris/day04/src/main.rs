//! Solve day 4 of Advent of Code 2025
use core::num::Saturating;
use std::{fs, path::Path, process::exit};

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
        let Ok(urow) = usize::try_from(row) else {
            return None;
        };
        let Ok(ucol) = usize::try_from(col) else {
            return None;
        };
        self.at(urow, ucol)
    }

    /// Deletes movable stacks
    /// Returns updated matrix and number of stacks removed
    fn delete_movable(&self) -> (Self, usize) {
        let mut counter: usize = 0;
        let new_mat = self
            .matrix
            .iter()
            .enumerate()
            .map(|(idx, chr)| {
                let (row, col) = (idx.div_euclid(self.columns), idx.rem_euclid(self.columns));
                let neighbors = self.neighbors_at(row, col);
                if *chr == '@' && neighbors < 4 {
                    counter += 1;
                    '.'
                } else {
                    *chr
                }
            })
            .collect();
        (
            Self {
                columns: self.columns,
                matrix: new_mat,
            },
            counter,
        )
    }

    /// Count the neighbors at a position
    fn neighbors_at(&self, row: usize, col: usize) -> usize {
        (-1_isize..2_isize)
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
                    self.at_signed(neighbor_row, neighbor_col).unwrap_or('.')
                })
            })
            .filter(|chr| *chr == '@')
            .count()
    }

    /// Get number of rows
    fn rows(&self) -> usize {
        self.matrix.len().checked_div(self.columns).unwrap_or(0)
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
/// This findes movable stacks ('@'), that is stacks that have less than 4 neighbors.
#[expect(
    dead_code,
    reason = "This is the solution for part 1, we only call part 2 but want to keep this."
)]
fn count_movable(mat: &CharMatrix) -> usize {
    (0..mat.matrix.len())
        .map(|idx| (idx.div_euclid(mat.columns), idx.rem_euclid(mat.columns)))
        .filter(|&(row, col)| mat.at(row, col) == Some('@'))
        .filter(|&(row, col)| {
            let neighbors = mat.neighbors_at(row, col);
            if neighbors < 4 {
                debug!(
                    "Found movable position at row {row}, col {col} with {neighbors} neighbors."
                );
            }
            neighbors < 4
        })
        .count()
}

/// Solves part 2
/// Count movable stacks and remove them, repeat until no more can be removed.
fn count_and_delete_movable(mat: &CharMatrix) -> usize {
    // note: Duplication of delete_movable call is needed because the matrix we begin with
    // is borrowed but the matrices we then generate are owned by us
    let (mut cur_mat, mut deleted) = mat.delete_movable();
    debug!("Deleted {deleted} stacks.");
    let mut movable = Saturating(deleted);
    while deleted > 0 {
        (cur_mat, deleted) = cur_mat.delete_movable();
        debug!("Deleted {deleted} stacks.");
        movable += deleted;
    }
    movable.0
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
    let result = count_and_delete_movable(&input);
    println!("Result: {result}");
}
