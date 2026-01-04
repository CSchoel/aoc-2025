//! Solves day 12 of Advent of Code 2025

use core::clone::Clone;
use core::fmt::Debug;
use core::iter::repeat;
use core::num::ParseIntError;
use std::{collections::HashSet, env::args, fs, path::Path, process::exit};

use log::{Level, debug, error, info};
use regex::Regex;

/// State of a region
type Pixels = Vec<Vec<bool>>;

/// Represents a present shape
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PresentShape {
    /// The pixels occupied by the present
    /// Outer index is length, inner index is width
    pixels: Pixels,
}

impl PresentShape {
    /// Flips the shape along the length axis
    fn flip_lengthwise(&self) -> Self {
        Self {
            pixels: flip_lengthwise(&self.pixels),
        }
    }
    /// Flips the shape along the width axis
    fn flip_widthwise(&self) -> Self {
        Self {
            pixels: flip_widthwise(&self.pixels),
        }
    }
    /// Returns all different shapes that can be obtained by rotating and flipping this shape
    fn orientations(&self) -> Vec<Self> {
        let mut oris: HashSet<Self> = HashSet::new();
        for flip in [
            self.clone(),
            self.flip_lengthwise(),
            self.flip_widthwise(),
            self.flip_lengthwise().flip_widthwise(),
        ] {
            for rot in [
                flip.rotate_clockwise(),
                flip.rotate_clockwise().rotate_clockwise(),
                flip.rotate_clockwise()
                    .rotate_clockwise()
                    .rotate_clockwise(),
            ] {
                oris.insert(rot);
            }
        }
        oris.into_iter().collect::<Vec<Self>>()
    }

    /// Rotates the shape clockwise (interpreting length als y-axis and width as x-axis)
    fn rotate_clockwise(&self) -> Self {
        Self {
            pixels: rotate_clockwise(&self.pixels),
        }
    }
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
        let region: Pixels = vec![vec![false; self.width]; self.length];
        self.fits_all_in_region(&region, self.shape_quantities.clone(), &mut HashSet::new())
    }
    /// Version of `can_fit()` that also accepts a current region for recursive calls.
    fn fits_all_in_region(
        &self,
        current_region: &[Vec<bool>],
        mut remaining_quantities: Vec<usize>,
        known_impossible: &mut HashSet<Pixels>,
    ) -> bool {
        if known_impossible.contains(current_region) {
            // We already know that there is no solution for this state
            return false;
        }
        let Some((idx, _)) = remaining_quantities
            .iter()
            .enumerate()
            .find(|&(_, quant)| *quant > 0)
        else {
            // We've placed all presents => We found a solution!
            let current_region_str = print_region(current_region, None);
            info!("Found a solution:\n{current_region_str}");
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
        let orientations = shape.orientations();
        // Iterate over all free positions and try placing the present there
        let indices = (0..self.length).flat_map(|idxl| repeat(idxl).zip(0..self.width));
        for (idx_length, idx_width) in indices {
            for orientation in &orientations {
                if !present_fits_in_region_at_pos(
                    orientation,
                    current_region,
                    idx_length,
                    idx_width,
                ) {
                    debug!(
                        "Present of type {idx} does not fit into region at ({idx_length}, {idx_width}) with {total_remaining} remaining presents."
                    );
                    continue;
                }
                debug!(
                    "Placing present of type {idx} at pos ({idx_length}, {idx_width}) with {total_remaining} remaining presents."
                );
                // If the present fits, copy the region, and place it there
                let new_region = match place_present_in_region_at_pos(
                    orientation,
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
                if self.fits_all_in_region(
                    &new_region,
                    remaining_quantities.clone(),
                    known_impossible,
                ) {
                    // If yes, we just return.
                    return true;
                }
                // If no, we continue to evaluate different positions.
                // But we first save our knowledge that this state did not lead to a solution
                known_impossible.insert(new_region.clone());
                // Also add variants
                known_impossible.insert(flip_lengthwise(&new_region));
                known_impossible.insert(flip_widthwise(&new_region));
                known_impossible.insert(flip_lengthwise(&flip_widthwise(&new_region)));
            }
        }
        // We evaluated all positions but did not find a candidate => Unable to place.
        debug!(
            "Evaluated all positions for present of type {idx}, but found no free position with {total_remaining} remaining presents."
        );
        // Record impossibility of current state
        let current_region_copy = current_region.to_vec();
        known_impossible.insert(current_region_copy.clone());
        known_impossible.insert(flip_lengthwise(&current_region_copy));
        known_impossible.insert(flip_widthwise(&current_region_copy));
        known_impossible.insert(flip_lengthwise(&flip_widthwise(&current_region_copy)));
        false
    }
}

/// Flips the shape along the length axis
fn flip_lengthwise(pixels: &Pixels) -> Pixels {
    pixels.iter().rev().map(Clone::clone).collect()
}

/// Flips the shape along the width axis
fn flip_widthwise(pixels: &Pixels) -> Pixels {
    pixels
        .iter()
        .map(|len_slice| len_slice.iter().rev().copied().collect::<Vec<bool>>())
        .collect()
}

/// Rotates the shape clockwise (interpreting length als y-axis and width as x-axis)
fn rotate_clockwise(pixels: &Pixels) -> Pixels {
    let mut new_pixels: Pixels = Vec::new();
    for len_slice in pixels.iter().rev() {
        while new_pixels.len() < len_slice.len() {
            new_pixels.push(Vec::new());
        }
        for (val, vect) in len_slice.iter().zip(new_pixels.iter_mut()) {
            vect.push(*val);
        }
    }
    new_pixels
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
        debug!("Could not get region slice at length {idx_len}!");
        return false;
    };
    for (present_row, region_row) in present.pixels.iter().zip(region_slice.iter()) {
        let columns = idx_width..idx_width.saturating_add(present_row.len());
        let Some(region_row_slice) = region_row.get(columns.clone()) else {
            debug!("Could not get region row slice at width {idx_width}!");
            return false;
        };
        for (present_pixel, region_pixel) in present_row.iter().zip(region_row_slice) {
            if *present_pixel && *region_pixel {
                debug!(
                    "Pixel mismatch: present_pixel = {present_pixel}, region_pixel = {region_pixel}"
                );
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
                    } else if *pix {
                        '#'
                    } else {
                        '.'
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
) -> Result<Pixels, String> {
    let mut region_copy = region.iter().map(Clone::clone).collect::<Pixels>();
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
        for (present_pixel, region_pixel) in present_row.iter().zip(region_row_slice) {
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
    let mut pixels: Pixels = Vec::new();
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
    let tmp_region = vec![
        vec![false, false, false, false],
        vec![true, false, true, false],
        vec![true, false, true, false],
        vec![true, true, true, false],
    ];
    let tmp_shape = PresentShape {
        pixels: vec![
            vec![true, true, true],
            vec![true, false, true],
            vec![true, false, true],
        ],
    };
    let fits = present_fits_in_region_at_pos(&tmp_shape, &tmp_region, 0, 1);
    assert!(fits, "Shape should fit here!");
    let result = input.iter().map(TreeRegion::fits_all).collect::<Vec<_>>();
    println!("Result: {result:?}");
}
