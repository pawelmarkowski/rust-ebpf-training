// example docs for function marked with `///` and module docs with `//!`
//! This module demonstrates various data structures in Rust.
//!
//! # Examples
//! ```
//! use lib_with_tests::example_arrays;
//! assert_eq!(example_arrays()[0], 1);
//! ```
/// Example function demonstrating arrays
/// # Examples
/// ```
/// use lib_with_tests::example_arrays;
/// assert_eq!(example_arrays(), [1, 2, 3, 4, 5]);
/// ```
pub fn example_arrays() -> [i8; 5] {
    let arr = [1, 2, 3, 4, 5];
    // Arrays in Rust are fixed-size, homogeneous collections
    let arr_four: [i32; 4] = [1, 2, 3, 4]; // Array of 4 integers
    println!("Array of 4 integers: {:?}", arr_four);
    // Both loops do the same thing - iterate over references to array elements
    // arr.iter() explicitly creates an iterator of references
    for i in arr.iter() {
        println!("arr element: {}", i);
    }

    // &arr is syntactic sugar - Rust automatically calls .iter() for you
    // Both i variables are &i32 (references to integers)
    for i in &arr {
        println!("arr element: {}", i);
    }
    return arr;
}

pub fn fast_example_arrays() -> [i8; 5] {
    [1, 2, 3, 4, 5]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = super::example_arrays();
        assert_eq!(result, [1, 2, 3, 4, 5]);
    }
}
