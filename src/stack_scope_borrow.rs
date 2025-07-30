fn func_b(y: i32) -> i32 {
    let x = 10 + y;
    return x;
}

fn func_a(x: i32) -> i32 {
    let z = func_b(15);
    return z + x;
}

fn main() {
    println!("stack - func_a(25) is {}", func_a(25));
    let level_0 = String::from("foo");
    {
        let level_1 = 9;
        {
            let level_2 = vec![1, 2, 3];
        }
    } // level_1 goes out of scope here
    // println!("Level 2: {}", level_2); // will not compile

    let num_a = 1;
    let num_b = num_a;
    let str_a = String::from("asdf");
    let str_b = str_a.clone(); // Clone to copy the string data 
    println!("Number num1 is {}", num_a);
    println!("Number num2 is {}", num_b);
    println!("String str_a is {}", str_a);
    println!("String str_b is {}", str_b);

    let x1 = &num_a; // Borrowing num_a
    let x2 = &num_a; // Borrowing num_a again
    println!("--- Values ---");
    println!("x1 points to value: {}", *x1);
    println!("x2 points to value: {}", *x2);

    println!("--- Addr vars ---");
    println!("num_a lives at address: {:p}", &num_a);

    println!("--- Addr refs ---");
    println!("x1 points to address: {:p}", x1);
    println!("x2 points to address: {:p}", x2);

    println!("--- Addr of refs ---");
    println!("x1 itself lives at: {:p}", &x1);
    println!("x2 itself lives at: {:p}", &x2);

    let mut num_c = 2;
    let mut x3 = &mut num_c; // Mutable borrow
    *x3 += 1; // Modify through mutable reference
    let x4 = &num_c; // Another mutable borrow
    // println!("{} {}", x3, x4); // Both used = conflict!
}
