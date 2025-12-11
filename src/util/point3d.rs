use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point3d {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

pub const ORIGIN: Point3d = Point3d { x: 0, y: 0, z: 0 };


impl Point3d {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn manhattan(&self, o: &Point3d) -> isize {
        (self.x - o.x).abs() + (self.y - o.y).abs() + (self.z - o.z).abs()
    }

    pub fn distance_sq(&self, o: &Point3d) -> isize {
        (self.x - o.x).pow(2) + (self.y - o.y).pow(2) + (self.z - o.z).pow(2)
    }

    pub fn distance(&self, o: &Point3d) -> f64 {
        (self.distance_sq(o) as f64).sqrt()
    }
}

impl Hash for Point3d {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_isize(self.x);
        state.write_isize(self.y);
        state.write_isize(self.z);
    }
}

impl Add for Point3d {
    type Output = Self;

    #[inline]
    fn add(self, o: Point3d) -> Point3d {
        Point3d { x: self.x + o.x, y: self.y + o.y, z: self.z + o.z }
    }
}

impl AddAssign for Point3d {
    #[inline]
    fn add_assign(&mut self, o: Point3d) {
        self.x += o.x;
        self.y += o.y;
        self.z += o.z;
    }
}

impl Sub for Point3d {
    type Output = Self;

    #[inline]
    fn sub(self, o: Point3d) -> Point3d {
        Point3d { x: self.x - o.x, y: self.y - o.y, z: self.z - o.z }
    }
}

impl SubAssign for Point3d {
    #[inline]
    fn sub_assign(&mut self, o: Point3d) {
        self.x -= o.x;
        self.y -= o.y;
        self.z -= o.z;
    }
}

impl Mul<isize> for Point3d {
    type Output = Self;

    #[inline]
    fn mul(self, o: isize) -> Point3d {
        Point3d { x: self.x * o, y: self.y * o, z: self.z * o }
    }
}
