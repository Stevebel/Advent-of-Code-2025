use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub const ORIGIN: Point = Point { x: 0, y: 0 };

pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };

pub const ORTHAGONALS: [Point; 4] = [UP, RIGHT, DOWN, LEFT];
pub const NEIGHBORS: [Point; 8] = [
    Point { x: -1, y: -1 },
    UP,
    Point { x: 1, y: -1 },
    RIGHT,
    Point { x: 1, y: 1 },
    DOWN,
    Point { x: -1, y: 1 },
    LEFT
];

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn manhattan(&self, o: Point) -> isize {
        (self.x - o.x).abs() + (self.y - o.y).abs()
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_isize(self.x);
        state.write_isize(self.y);
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, o: Point) -> Point {
        Point { x: self.x + o.x, y: self.y + o.y }
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, o: Point) {
        self.x += o.x;
        self.y += o.y;
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, o: Point) -> Point {
        Point { x: self.x - o.x, y: self.y - o.y }
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, o: Point) {
        self.x -= o.x;
        self.y -= o.y;
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    #[inline]
    fn mul(self, o: isize) -> Point {
        Point { x: self.x * o, y: self.y * o }
    }
}
