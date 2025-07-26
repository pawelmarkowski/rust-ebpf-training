// Rust Naming Conventions:
// - Functions and variables: snake_case
// - Constants: SCREAMING_SNAKE_CASE
// - Types (structs, enums): PascalCase
// - Modules: snake_case

#[derive(PartialEq)]
struct CityNames(String, i32);

fn main() {
    println!("Rust Loop Examples with Break");
    
    // Example 1: Simple loop with break
    print_numbers_with_break();
    
    // Example 2: Loop with condition and break
    find_target_number(3);
    
    // Example 3: While loop
    print_with_while_loop();

    let city1 = CityNames("Warsaw".to_string(), 1865000);
    println!("City 1: {} with population {}", city1.0, city1.1);
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