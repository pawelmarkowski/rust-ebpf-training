use std::collections::HashMap;

fn main() {
    example_arrays();
    example_slices();
    example_tuples();
    example_vectors();
    example_hash_maps();
}

fn example_arrays() {
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
}

fn example_slices() {
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    // Slices are a view into a contiguous sequence of elements
    for i in slice {
        println!("{}", i);
    }

    // Slicing an array
    let arr = [1, 2, 3, 4, 5];
    let slice_of_arr = &arr[1..4]; // Elements from index 1 to 3 (4 is exclusive)
    for i in slice_of_arr {
        println!("{}", i);
    }
}

fn example_tuples() {
    let tuple = (1, "hello", 3.14);
    // Tuples can hold mixed types and have a fixed size
    println!("Tuple: {:?}", tuple);

    // Destructuring a tuple
    let (x, y, z) = tuple;
    println!("Destructured: x = {}, y = {}, z = {}", x, y, z);

    // Accessing tuple elements by index
    println!("First element: {}", tuple.0);
    println!("Second element: {}", tuple.1);
}

fn example_vectors() {
    // vec! macro creates a new vector
    let mut numbers_mac = vec![1, 2];
    numbers_mac.push(3); // Add an element to the vector
    let mut numbers: Vec<i32> = Vec::new(); // Create an empty vector
    numbers.push(1); // Add elements to the vector
    numbers.push(2);
    numbers.push(3);
    println!("Vector: {:?}", numbers);
    println!("Are vectors equal? {}", numbers == numbers_mac);
    // Iterating over a vector
    for num in &numbers {
        println!("Vector element: {}", num);
    }
    // Accessing elements by index
    if let Some(first) = numbers.get(0) {
        println!("First element: {}", first);
    } else {
        println!("Vector is empty");
    }
    // Modifying elements by index
    // 'Some' is used here to match only when the Option contains a value.
    // This allows us to safely extract 'second' without risking a panic from an absent value.
    if let Some(second) = numbers.get_mut(1) {
        *second += 10; // Dereference to modify the value
    }
    println!("Modified vector: {:?}", numbers);
}

fn example_hash_maps() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    println!("Map: {:?}", map);
    // Iterating over a HashMap
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
    // Accessing values by key
    if let Some(value) = map.get(&2) {
        println!("Value for key 2: {}", value);
    } else {
        println!("Key 2 not found in the map");
    }
    // Modifying values
    if let Some(value) = map.get_mut(&3) {
        *value = "three modified"; // Dereference to modify the value
    }
    println!("Modified map: {:?}", map);
    // Removing a key-value pair
    map.remove(&1);
    println!("Map after removing key 1: {:?}", map);
    // accessing key that does not exist
    if let Some(value) = map.get(&4) {
        println!("Value for key 4: {}", value);
    } else {
        println!("Key 4 not found in the map");
    }
}
