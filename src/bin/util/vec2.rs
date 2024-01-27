use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

pub type Direction = Point2;

pub const LEFT: Direction = Point2 { x: -1, y: 0 };
pub const RIGHT: Direction = Point2 { x: 1, y: 0 };
pub const UP: Direction = Point2 { x: 0, y: -1 };
pub const DOWN: Direction = Point2 { x: 0, y: 1 };

impl Mul<i32> for Direction {
    type Output = Direction;

    fn mul(self, rhs: i32) -> Self::Output {
        return Direction {
            x: rhs * self.x,
            y: rhs * self.y,
        };
    }
}

impl Add<Direction> for Point2 {
    type Output = Point2;

    fn add(self, rhs: Direction) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Direction> for Point2 {
    fn add_assign(&mut self, rhs: Direction) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl PartialOrd for Point2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x != other.x {
            return Some(self.x.cmp(&other.x));
        }
        return Some(self.y.cmp(&other.y));
    }
}

impl Ord for Point2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x != other.x {
            return self.x.cmp(&other.x);
        }
        return self.y.cmp(&other.y);
    }
}

pub fn origin() -> Point2 {
    return Point2 { x: 0, y: 0 };
}

pub fn rotate_left(d: Direction) -> Direction {
    Direction { x: -d.y, y: d.x }
}

pub fn rotate_right(d: Direction) -> Direction {
    Direction { x: d.y, y: -d.x }
}
