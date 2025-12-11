use std::collections::HashSet;

use advent_of_code::util::point3d::Point3d;
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn input_to_points(input: &str) -> Vec<Point3d> {
    input.lines()
        .map(|line| {
            let items: Vec<isize> = line.split(',').map(|s| s.parse().unwrap_or(0)).collect();
            Point3d::new(items[0], items[1], items[2])
        })
        .collect()
}

pub fn get_circuit_size(input: &str, pairings: u64) -> u64 {
    let points = input_to_points(input);
    let pairs: Vec<_> = points.iter().combinations(2).sorted_by(|a, b| {
        let dist_a = a[0].distance(&a[1]);
        let dist_b = b[0].distance(&b[1]);
        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
    }).take(pairings as usize).collect();
    let mut circuits: Vec<Vec<Point3d>> = Vec::new();
    for i in 0..pairs.len() {
        let pair = &pairs[i];
        // Collect indices first to avoid borrow checker issues
        let left_idx = circuits.iter().position(|circuit| circuit.contains(&pair[0]));
        let right_idx = circuits.iter().position(|circuit| circuit.contains(&pair[1]));

        match (left_idx, right_idx) {
            (Some(left), Some(right)) => {
                // Combine the two circuits, unless they are the same circuit
                if left != right {
                    let right_circuit = circuits.remove(right);
                    circuits[left].extend(&right_circuit);
                }
            }
            (Some(left), None) => {
                circuits[left].push(*pair[1]);
            }
            (None, Some(right)) => {
                circuits[right].push(*pair[0]);
            }
            (None, None) => {
                circuits.push(vec![*pair[0], *pair[1]]);
            }
        }
    }
    circuits.iter().map(|circuit| circuit.len() as u64).sorted().rev().take(3).product()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(get_circuit_size(input, 1000))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = get_circuit_size(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
