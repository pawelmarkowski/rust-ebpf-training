/// This module demonstrates Rust's naming conventions, struct and enum usage, and loop control flow.
/// 
/// # Naming Conventions
/// - Functions and variables use `snake_case`.
/// - Constants use `SCREAMING_SNAKE_CASE`.
/// - Types (structs, enums) use `PascalCase`.
/// - Modules use `snake_case`.
///
/// # Structs
/// - `CitiesTupleStruct`: Tuple struct representing a city with a name and population.
/// - `CitiesStruct`: Classic struct with named fields for city name and population.
///
/// # Enums
/// - `Season`: Represents the four seasons.
/// - `Trigger`: Represents different trigger types, including variants with associated data.
///
/// # Loop Examples
/// - `print_numbers_with_break`: Prints numbers 1-5 using a loop and `break`.
/// - `find_target_number`: Searches for a target number using a loop and conditional `break`.
/// - `print_with_while_loop`: Prints numbers 1-3 using a `while` loop.
///
/// # Enum Pattern Matching
/// - The `enum_example` function demonstrates matching on the `Trigger` enum.
/// - The `ref` keyword is used in match arms (e.g., `ref season`) to borrow the value rather than move it.
///   This is useful when you want to use the value without taking ownership, allowing further use after the match.
/// - For example, `Trigger::SeasonDays { ref season, repeat_every_days }` borrows `season` so it can be printed with `{:?}` without moving it.
///
/// # Methods
/// - `Trigger::is_weekend`: Returns `true` if the trigger is a weekend type, otherwise `false`.
///
/// # Usage
/// The `main` function demonstrates struct instantiation, loop usage, and enum pattern matching.
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

#[derive(Debug)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

enum Trigger {
    Everyday,
    Weekend,
    SeasonDays {
        season: Season,
        repeat_every_days: u8,
    },
    SeasonWeekends(Season),
}

/// Determines if the current `Trigger` variant represents a weekend.
/// The `..` in `Trigger::SeasonDays { .. }` is used to match any fields in the struct variant, ignoring their values.
/// The `_` in `Trigger::SeasonWeekends(_)` matches any value inside the tuple variant, also ignoring its value.
/// This allows the match to focus only on the variant type, not the specific data it holds.
impl Trigger {
    fn is_weekend(&self) -> bool {
        match self {
            Trigger::Everyday => false,
            Trigger::Weekend => true,
            Trigger::SeasonDays { .. } => false,
            Trigger::SeasonWeekends(_) => true,
        }
    }
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
    enum_example();
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

fn enum_example() {
    let trigger = Trigger::SeasonDays {
        season: Season::Summer,
        repeat_every_days: 7,
    };
    
    match trigger {
        Trigger::Everyday => println!("Everyday trigger"),
        Trigger::Weekend => println!("Weekend trigger"),
        Trigger::SeasonDays { ref season, repeat_every_days } => {
            println!("Season days trigger: {:?}, repeat every {} days", season, repeat_every_days);
        }
        Trigger::SeasonWeekends(ref season) => {
            println!("Season weekends trigger: {:?}", season);
        }
    }
    println!("Is weekend? {}", trigger.is_weekend());
}