fn main() {
    println!("Hello, world!");
    let inferred_integer_x = 5; // by default variables are considered immutable (cannot be changed - causes compilation error if changed)
    let mut integer_x: i32 = 10; // add mut to make it mutable
    println!("Inferred integer x: {}", inferred_integer_x);
    println!("Explicitly typed integer x: {}", integer_x);
    println!("The sum of inferred and explicitly typed integers is: {}", inferred_integer_x + integer_x);
    integer_x = 15; // This line will not cause a compilation error because `integer_x` is mutable
    println!("Updated integer x: {}", integer_x);

    let result = if 1 == 2 {  // if is expression, not a statement
        "First check failed"
    } else if 1 == 6 {
        "Second check failed"
    } else {
        "Nothing makes sense"
    };
    println!("Result of computation: {:?}", result);


}