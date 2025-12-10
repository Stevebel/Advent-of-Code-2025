use std::collections::{HashMap, HashSet};

use advent_of_code::util::{grid::Grid, point::*};

advent_of_code::solution!(7);

pub fn input_to_start_and_splitters(input: &str) -> (Point, Vec<Point>) {
    let grid = Grid::parse(input);
    let start = grid.get_first_point(b'S').unwrap();
    let splitters = grid.get_points(b'^');
    (start, splitters)
}

pub fn get_hit(from: Point, splitters: &Vec<Point>) -> Option<Point> {
    let mut hit = None;
    for splitter in splitters {
        if splitter.x == from.x && splitter.y > from.y && splitter.y < hit.map_or(9999999, |h: Point| h.y) {
            hit = Some(*splitter);
        }
    }
    hit
}

pub fn part_one(input: &str) -> Option<u64> {
    let (start, splitters) = input_to_start_and_splitters(input);
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    let first = get_hit(start, &splitters)?;
    visited.insert(first);
    queue.push(first);
    while let Some(current) = queue.pop() {
        let left = get_hit(current + LEFT, &splitters);
        let right = get_hit(current + RIGHT, &splitters);
        left.map(|l| {
            if visited.insert(l) {
                queue.push(l);
            }
        });
        right.map(|r| {
            if visited.insert(r) {
                queue.push(r);
            }
        });
    }
    Some(visited.len() as u64)
}

pub fn path_count(start: Point, splitters: &Vec<Point>, memo: &mut HashMap<Point, u64>) -> u64 {
    if memo.contains_key(&start) {
        return memo[&start];
    }
    let left = get_hit(start + LEFT, splitters);
    let right = get_hit(start + RIGHT, splitters);
    let mut count = 0;
    count += left.map_or(1, |l| path_count(l, splitters, memo));
    count += right.map_or(1, |r| path_count(r, splitters, memo));
    memo.insert(start, count);
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let (start, splitters) = input_to_start_and_splitters(input);
    Some(path_count(get_hit(start, &splitters).unwrap(), &splitters, &mut HashMap::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
