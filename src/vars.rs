// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Anthony";

    // Use mut to make variable mutable
    let mut age = 22;

    println!("my name is {} and I am {}", name, age);

    age += 1;

    println!("my name is {} and I am {}", name, age);

    // Define constant
    // Constants are usually all uppercase and need a type assigned

    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple variables
    let ( my_name, my_age) = ("Anthony", 22);
    println!("{} is {}", my_name, my_age);
}