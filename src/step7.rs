trait Quxx<RHS = Self> {
    type Output;

    fn quxx(self, rhs: RHS) -> Self::Output;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Quxx for Point {
    type Output = Point;

    fn quxx(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

use std::ops::Add;

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Quxx::quxx(self, other)
    }
}

pub fn step7() {
    println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 })
}
