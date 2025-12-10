use std::ops::{Index, IndexMut};
use crate::util::point::Point;

pub struct Grid<T: Copy> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = lines[0].len();
        let height = lines.len();
        let data = lines.concat();
        Self { data, width, height }
    }

    pub fn parse_with_padding(input: &str, pad: u8) -> Self {
        let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = lines[0].len() + 2;
        let height = lines.len() + 2;
        let mut data = Vec::with_capacity(width * height);
        for x in 0..width {
            for y in 0..height {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    data.push(pad);
                } else {
                    data.push(lines[y - 1][x - 1]);
                }
            }
        }
        Self { data, width, height }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y) as char);
            }
            println!();
        }
    }
}

impl<T: Copy> Grid<T> {
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[y * self.width + x]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y * self.width + x] = value;
    }
}

impl<T: PartialEq + Copy> Grid<T> {
    #[inline]
    pub fn get_first_point(&self, value: T) -> Option<Point> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == value {
                    return Some(Point::new(x as isize, y as isize));
                }
            }
        }
        None
    }

    #[inline]
    pub fn get_points(&self, value: T) -> Vec<Point> {
        let mut points = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == value {
                    points.push(Point::new(x as isize, y as isize));
                }
            }
        }
        points
    }
}

impl<T: Copy> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &T {
        &self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl<T: Copy> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut T {
        &mut self.data[index.y as usize * self.width + index.x as usize]
    }
}
