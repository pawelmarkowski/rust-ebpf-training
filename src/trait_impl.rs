use std::convert::Into;
use std::fmt::{Display, Formatter, Result};
use std::ops::Add;
use std::string::FromUtf8Error;

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

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

// Original function with explicit match
fn bytestring_to_string_with_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = match String::from_utf8(str) {
        Ok(str) => str.to_uppercase(),
        Err(err) => return Err(err),
    };
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

// Composition with map - more functional style
fn bytestring_to_string_with_map(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    String::from_utf8(str)
        .map(|s| s.to_uppercase()) // Apply uppercase only if Ok
        .map(|s| {
            // Apply side effect (println) only if Ok
            println!("Conversion succeeded: {}", s);
            s
        })
}

// Even more composition examples
fn composition_examples() {
    let bytes1 = vec![72, 101, 108, 108, 111]; // "Hello" in bytes
    let bytes2 = vec![255, 254, 253]; // Invalid UTF-8

    println!("=== Match version ===");
    match bytestring_to_string_with_match(bytes1.clone()) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Map version ===");
    match bytestring_to_string_with_map(bytes1) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Error case ===");
    match bytestring_to_string_with_map(bytes2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // Point examples
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = p1 + p2;
    println!("Summed: {:?}", p3);

    // Convert to tuple (this consumes p3)
    let tuple: (i32, i32) = p3.into();
    println!("Converted to tuple: {:?}", tuple);

    println!("\n=== Composition Examples ===");
    composition_examples();
}
