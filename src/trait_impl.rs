use std::convert::Into;
use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Into<(T, T)> for Point<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = p1 + p2;
    println!("Summed point: {:?}", p3);

    // Convert to tuple (this consumes p3)
    let tuple: (i32, i32) = p3.into();
    println!("Converted to tuple: {:?}", tuple);
}
