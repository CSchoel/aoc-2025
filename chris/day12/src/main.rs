//! Solves day 12 of Advent of Code 2025

use core::clone::Clone;
use core::fmt::Debug;
use core::num::ParseIntError;
use std::{env::args, fs, path::Path, process::exit};

use log::{Level, debug, error, info};
use regex::Regex;

/// Represents a present shape
#[derive(Debug, Clone)]
struct PresentShape {
    /// The pixels occupied by the present
    /// Outer index is length, inner index is width
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

impl TreeRegion {
    /// Determines whether all presents of the desired shapes can fit into this region
    fn fits_all(&self) -> bool {
        let region: Vec<Vec<bool>> = vec![vec![false; self.width]; self.length];
        self.fits_all_in_region(&region, self.shape_quantities.clone())
    }
    /// Version of `can_fit()` that also accepts a current region for recursive calls.
    fn fits_all_in_region(
        &self,
        current_region: &[Vec<bool>],
        mut remaining_quantities: Vec<usize>,
    ) -> bool {
        let Some((idx, _)) = remaining_quantities
            .iter()
            .enumerate()
            .find(|&(_, quant)| *quant > 0)
        else {
            // We've placed all presents => We found a solution!
            return true;
        };
        let Some(shape) = self.present_shapes.get(idx) else {
            error!("Could not find shape with index {idx} in {self:?}. This should never happen!");
            return false;
        };
        let total_remaining: usize = remaining_quantities.iter().sum();
        if let Some(el) = remaining_quantities.get_mut(idx) {
            *el = el.saturating_sub(1);
        }
        // Iterate over all free positions and try placing the present there
        for idx_length in 0..current_region.len() {
            for idx_width in 0..current_region.first().map_or(0, Vec::len) {
                if !present_fits_in_region_at_pos(shape, current_region, idx_length, idx_width) {
                    debug!(
                        "Present of type {idx} does not fit into region at ({idx_length}, {idx_width}) with {total_remaining} remaining presents."
                    );
                    continue;
                }
                info!(
                    "Placing present of type {idx} at pos ({idx_length}, {idx_width}) with {total_remaining} remaining presents."
                );
                // If the present fits, copy the region, and place it there
                let new_region = match place_present_in_region_at_pos(
                    shape,
                    current_region,
                    idx_length,
                    idx_width,
                ) {
                    Ok(reg) => reg,
                    Err(err) => {
                        error!("Could not place present in region. Reason:\n{err}");
                        return false;
                    }
                };
                // Now check recursively if we reach a solution by placing the present there
                if self.fits_all_in_region(&new_region, remaining_quantities.clone()) {
                    // If yes, we just return.
                    return true;
                }
                // If no, we continue to evaluate different positions.
            }
        }
        // We evaluated all positions but did not find a candidate => Unable to place.
        info!(
            "Evaluated all positions for present of type {idx}, but found no free position with {total_remaining} remaining presents."
        );
        false
    }
}

/// Checks whether the present `present` fits into `region` at index (`idx_len`, `idx_width`)
fn present_fits_in_region_at_pos(
    present: &PresentShape,
    region: &[Vec<bool>],
    idx_len: usize,
    idx_width: usize,
) -> bool {
    if log::log_enabled!(Level::Debug) {
        let present_str = print_region(&present.pixels, None);
        let region_str = print_region(region, Some((idx_len, idx_width)));
        debug!(
            "Checking whether package fits in region.\n\nPackage:\n{present_str}\n\nRegion:\n{region_str}"
        );
    }
    let Some(region_slice) = region.get(idx_len..idx_len.saturating_add(present.pixels.len()))
    else {
        return false;
    };
    for (present_row, region_row) in present.pixels.iter().zip(region_slice.iter()) {
        let columns = idx_width..idx_width.saturating_add(present_row.len());
        let Some(region_row_slice) = region_row.get(columns.clone()) else {
            return false;
        };
        let Some(present_row_slice) = present_row.get(columns) else {
            return false;
        };
        for (present_pixel, region_pixel) in present_row_slice.iter().zip(region_row_slice) {
            if *present_pixel && *region_pixel {
                return false;
            }
        }
    }
    true
}

/// Prints a region in the same format as used by the exercise description
fn print_region(pixels: &[Vec<bool>], mark_position: Option<(usize, usize)>) -> String {
    let (mark_len, mark_wid) = mark_position.unwrap_or((usize::MAX, usize::MAX));
    pixels
        .iter()
        .enumerate()
        .map(|(idx_len, len_slice)| {
            let mut len_slice_str = len_slice
                .iter()
                .enumerate()
                .map(|(idx_wid, pix)| {
                    if idx_len == mark_len && idx_wid == mark_wid {
                        if *pix { 'X' } else { 'x' }
                    } else {
                        if *pix { '#' } else { '.' }
                    }
                })
                .collect::<String>();
            len_slice_str.push('\n');
            len_slice_str
        })
        .collect::<String>()
}

/// Places present `present` into position (`idx_len`, `idx_width`) in `region`.
fn place_present_in_region_at_pos(
    present: &PresentShape,
    region: &[Vec<bool>],
    idx_len: usize,
    idx_width: usize,
) -> Result<Vec<Vec<bool>>, String> {
    let mut region_copy = region.iter().map(Clone::clone).collect::<Vec<Vec<bool>>>();
    let Some(region_slice) =
        region_copy.get_mut(idx_len..idx_len.saturating_add(present.pixels.len()))
    else {
        return Err(format!(
            "Could not access length slice at {idx_len} of region!"
        ));
    };
    for (present_row, region_row) in present.pixels.iter().zip(region_slice.iter_mut()) {
        let columns = idx_width..idx_width.saturating_add(present_row.len());
        let Some(region_row_slice) = region_row.get_mut(columns.clone()) else {
            return Err(format!(
                "Could not access width slice at {idx_width} of region!"
            ));
        };
        let Some(present_row_slice) = present_row.get(columns) else {
            return Err(format!(
                "Could not access width slice at {idx_width} of present!"
            ));
        };
        for (present_pixel, region_pixel) in present_row_slice.iter().zip(region_row_slice) {
            if *present_pixel {
                *region_pixel = true;
            }
        }
    }
    Ok(region_copy)
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
            let (_, [width_str, length_str, quantities]) = match_region.extract();
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
    let result = input.first().map(TreeRegion::fits_all);
    println!("Result: {result:?}");
}
