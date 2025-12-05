advent_of_code::solution!(4);

use advent_of_code::util::grid::Grid;
use advent_of_code::util::point::*;

const EMPTY: u8 = b'.';
const ROLL: u8 = b'@';

pub fn input_to_grid(input: &str) -> Grid<u8> {
    Grid::parse_with_padding(input, EMPTY)
}

pub fn get_neighbors(grid: &Grid<u8>, p: Point) -> Vec<u8> {
    NEIGHBORS.iter().map(|&n| grid[p + n]).collect()
}

pub fn roll_count(cells: &Vec<u8>) -> u64 {
    cells.iter().filter(|&&cell| cell == ROLL).count() as u64
}

pub fn get_accessible_rolls(grid: &Grid<u8>) -> Vec<Point> {
    (0..grid.height)
        .flat_map(|row| (0..grid.width)
        .map(move |col| Point::new(col as isize, row as isize)))
        .filter(|&p| grid[p] == ROLL && roll_count(&get_neighbors(&grid, p)) < 4)
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input_to_grid(input);
    Some(
        get_accessible_rolls(&grid).len() as u64
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = input_to_grid(input);
    let mut removed_count = 0;
    loop {
        let accessible_rolls = get_accessible_rolls(&grid);
        if accessible_rolls.len() == 0 {
            break;
        }
        removed_count += accessible_rolls.len();
        for &p in &accessible_rolls {
            grid[p] = EMPTY;
        }
    }
    Some(removed_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
