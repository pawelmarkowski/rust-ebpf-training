// Rust Naming Conventions:
// - Functions and variables: snake_case
// - Constants: SCREAMING_SNAKE_CASE
// - Types (structs, enums): PascalCase
// - Modules: snake_case

struct CitiesTupleStruct(String, i32);

struct CitiesStruct {
    name: String,
    population: i32,
}

fn main() {
    println!("Rust Loop Examples with Break");
    
    // Example 1: Simple loop with break
    print_numbers_with_break();
    
    // Example 2: Loop with condition and break
    find_target_number(3);
    
    // Example 3: While loop
    print_with_while_loop();

    // Example 4: Using structs to represent cities
    // String::from and to_string() are used to create String types
    // "Warsaw" and "Krakow" are &'static str literals, but converted to String
    let city1 = CitiesTupleStruct(String::from("Warsaw"), 1865000);
    let city2 = CitiesStruct {
        name: "Krakow".to_string(),
        population: 779115,
    };
    println!("City 1: {} with population {}", city1.0, city1.1);
    println!("City 2: {} with population {}", city2.name, city2.population);

    println!("Population in {} is higher than in {}: {}",
             city1.0, city2.name, city1.1 > city2.population);
}

/// Prints numbers from 1 to 5 using a loop with break
fn print_numbers_with_break() {
    println!("\nPrinting numbers 1-5:");
    
    let mut counter = 1;
    loop {
        println!("{}", counter);
        
        if counter == 5 {
            break; // Exit the loop when we reach 5
        }
        
        counter += 1;
    }
}

/// Example of using break with a condition
fn find_target_number(target: i32) {
    println!("\nFinding target number:");
    
    let mut current = 1;
    
    loop {
        println!("Current number: {}", current);
        
        if current == target {
            println!("Found target number: {}", target);
            break;
        }
        
        if current > 10 {
            println!("Target not found, stopping search");
            break;
        }
        
        current += 1;
    }
}

/// Prints numbers from 1 to 3 using a while loop
fn print_with_while_loop() {
    println!("\nPrinting numbers 1-3 with while loop:");
    
    let mut number = 1;
    while number <= 3 {
        println!("{}", number);
        number += 1;
    }
}