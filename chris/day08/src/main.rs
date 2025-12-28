//! Solves day 8 of Advent of Code 2025

use core::fmt::Write as _;
use core::num::ParseIntError;
use std::collections::{self, HashSet};
use std::usize;
use std::{collections::HashMap, env::args, fs, path::Path, process::exit};

use log::debug;
use log::info;

/// Represents a position in 3D carthesian coordinates
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position3D {
    /// Position on x axis
    x: usize,
    /// Position on y axis
    y: usize,
    /// Position on z axis
    z: usize,
}

impl Position3D {
    /// Calculates euclidean distance between two positions
    #[expect(
        clippy::cast_precision_loss,
        reason = "We have to convert to float to take the square root."
    )]
    #[expect(
        clippy::as_conversions,
        reason = "As conversion from usize to u64 and from i64 to f64 is safe."
    )]
    fn dist(&self, other: &Self) -> f64 {
        let square_sum = (self.x.abs_diff(other.x))
            .pow(2)
            .saturating_add((self.y.abs_diff(other.y)).pow(2))
            .saturating_add((self.z.abs_diff(other.z)).pow(2));
        (square_sum as u64 as f64).sqrt()
    }
}

/// Represents a matrix of distances which can be queried by `Position3D` objects
type DistanceMatrix<'link> = HashMap<(&'link Position3D, &'link Position3D), f64>;

/// Parses input for day 8
fn parse_input(content: &str) -> Result<Vec<Position3D>, String> {
    content
        .lines()
        .map(|line| {
            let Ok(numbers) = line
                .split(',')
                .map(str::parse::<usize>)
                .collect::<Result<Vec<usize>, ParseIntError>>()
            else {
                return Err(format!("Could not parse numbers in line {line}"));
            };
            let &[x, y, z] = numbers.as_slice() else {
                return Err(format!("Wrong number of coordinates: {numbers:?}"));
            };
            Ok(Position3D { x, y, z })
        })
        .collect()
}

/// Creates a new distance matrix containing the distances between all given positions
#[expect(
    clippy::indexing_slicing,
    reason = "If we are out of index here, it's a programming error and we want to panic."
)]
fn distance_matrix(positions: &[Position3D]) -> DistanceMatrix<'_> {
    let mut map = HashMap::new();
    for pos1_idx in 0..positions.len() {
        for pos2_idx in pos1_idx..positions.len() {
            let pos1 = &positions[pos1_idx];
            let pos2 = &positions[pos2_idx];
            let dist = pos1.dist(pos2);
            map.insert((pos1, pos2), dist);
            map.insert((pos2, pos1), dist);
            debug!("Distance between {pos1:?} and {pos2:?} is {dist}.");
        }
    }
    map
}

/// Gets the `num` pairs in `matrix` with the shortest distances to each other
fn shortest_distances<'link>(
    matrix: &DistanceMatrix<'link>,
) -> Vec<(&'link Position3D, &'link Position3D)> {
    let mut positions: Vec<&(&Position3D, &Position3D)> =
        matrix.keys().filter(|&&(pos1, pos2)| pos1 < pos2).collect();
    positions.sort_by(|pair1, pair2| {
        let dist1 = matrix.get(pair1).unwrap_or(&f64::INFINITY);
        let dist2 = matrix.get(pair2).unwrap_or(&f64::INFINITY);
        // debug!("Distance between {pair1:?} is {dist1} and distance between {pair2:?} is {dist2}.");
        dist1.total_cmp(dist2)
    });
    positions.iter().map(|x| **x).collect()
}

/// Solves part 1 of day 8
#[expect(
    unused_must_use,
    reason = "The error would only be part of a debug print."
)]
fn count_connected(positions: &[Position3D], num: usize) -> usize {
    let distances = distance_matrix(positions);
    let shortest = shortest_distances(&distances);
    let mut group_ids: HashMap<&Position3D, usize> = HashMap::new();
    let mut groups: HashMap<usize, HashSet<&Position3D>> = HashMap::new();
    let mut connections: usize = 0;
    for (pos1, pos2) in shortest {
        let group_id_1 = *group_ids.get(pos1).unwrap_or(&connections);
        let group_id_2 = *group_ids.get(pos2).unwrap_or(&connections);
        // Skip connections within the same group
        if group_id_1 != connections && group_id_1 == group_id_2 {
            debug!(
                "Skipping connection between {pos1:?} and {pos2:?} because both have group ID {group_id_1}."
            );
            continue;
        }
        // Merge groups: All group IDs of group2 have to be set to the ID of group1
        debug!(
            "Connecting {pos1:?} and {pos2:?}, merging group {group_id_2} into group {group_id_2}."
        );
        let removed = groups
            .remove(&group_id_2)
            .unwrap_or_else(|| HashSet::from_iter(vec![pos2]));
        let mut removed_sorted = removed.iter().collect::<Vec<&&Position3D>>();
        removed_sorted.sort();
        let group1_set = groups
            .entry(group_id_1)
            .or_insert_with(|| HashSet::from_iter(vec![pos1]));
        for pos_to_update in removed_sorted {
            group1_set.insert(pos_to_update);
            group_ids.insert(pos_to_update, group_id_1);
        }
        connections = connections.saturating_add(1);
        let sizes = groups.iter().fold(String::new(), |mut acc, (id, grp)| {
            let grp_len = grp.len();
            write!(acc, "{id}: {grp_len}, ");
            acc
        });
        debug!("Group sizes: {sizes}");
        if connections >= num {
            // Only add `num` connections
            break;
        }
    }
    // Get group sizes and multiply the three largest ones.
    let mut sizes = groups
        .values()
        .map(collections::HashSet::len)
        .collect::<Vec<usize>>();
    // sort in descending order
    sizes.sort_by_key(|val| usize::MAX.saturating_sub(*val));
    sizes.iter().take(3).product()
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
    let result = count_connected(&input, 1000);
    println!("Result: {result}");
}
