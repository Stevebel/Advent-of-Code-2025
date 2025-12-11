use advent_of_code::util::{geom::Rectangle, point::Point};
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn input_to_points(input: &str) -> Vec<Point> {
    input.lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    input_to_points(input).into_iter()
        .combinations(2)
        .map(|pair| {
            let length = (pair[0].x - pair[1].x).abs() + 1;
            let width = (pair[0].y - pair[1].y).abs() + 1;
            (length * width) as u64
        })
        .max()
}

pub fn is_clockwise(dir_a: Point, dir_b: Point) -> bool {
    let cross_product = dir_a.x * dir_b.y - dir_a.y * dir_b.x;
    cross_product > 0
}

pub fn is_reflex(a: Point, b: Point, c: Point) -> bool {
    //cross = (B.x - A.x)*(C.y - B.y) - (B.y - A.y)*(C.x - B.x)
    let cross = (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x);
    cross > 0
}

pub fn has_clockwise_path(perimeter: &Vec<Point>, a: usize, b: usize, rect: &Rectangle) -> bool {
    let end = if b < a { b + perimeter.len() } else { b };
    let mut direction = perimeter[(a + 1) % perimeter.len()] - perimeter[a];
    let mut last = perimeter[(a + 1) % perimeter.len()];
    for k in a + 2..end {
        let test = perimeter[k % perimeter.len()];
        let next_direction = test - last;
        if !is_clockwise(direction, next_direction) && rect.contains(&test) {
            println!("not clockwise: {:?}, {:?}, {:?}", direction, next_direction, test);
            return false;
        }
        direction = next_direction;
        last = test;
    }
    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let perimeter = input_to_points(input);
    (0..perimeter.len()).combinations(2).map(|points| {
        let a = points[0];
        let b = points[1];
        // If a point between a and b is inside the rectangle formed by a and b, then the perimeter doesn't contain the rectangle.
        let rect = Rectangle::from_points(&perimeter[a], &perimeter[b]);
        let fwd_cw = has_clockwise_path(&perimeter, a, b, &rect);
        let bwd_cw = has_clockwise_path(&perimeter, b, a, &rect);
        if !bwd_cw || !fwd_cw {
            return 0;
        }
        println!("a: {:?}, b: {:?}, area: {}, fwd_cw: {}, bwd_cw: {}, dir: {:?}, rect_dir: {:?}", perimeter[a], perimeter[b], rect.area(), fwd_cw, bwd_cw, perimeter[b] - perimeter[a], rect.bottom_right - rect.top_left);

        rect.area() as u64
    }).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
