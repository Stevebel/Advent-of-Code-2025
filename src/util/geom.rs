use crate::util::point::Point;

#[derive(Debug)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn from_points(a: &Point, b: &Point) -> Self {
        let top_left = Point::new(a.x.min(b.x), a.y.min(b.y));
        let bottom_right = Point::new(a.x.max(b.x), a.y.max(b.y));
        Self { top_left, bottom_right }
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.top_left.x && point.x <= self.bottom_right.x && point.y >= self.top_left.y && point.y <= self.bottom_right.y
    }

    pub fn area(&self) -> isize {
        ((self.bottom_right.x - self.top_left.x).abs() + 1) * ((self.bottom_right.y - self.top_left.y).abs() + 1)
    }
}
