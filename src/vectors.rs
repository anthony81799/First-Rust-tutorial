// Vectors - Fixed list where elements are the same data types

// Use statements declare namespace to shorten lines where multiple namespaces are used
// repeatedly
use std::mem;

pub fn run() {
    println!("vectors.rs starts here");
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Print whole vector
    println!("{:?}", numbers);

    // Indexing
    println!("Single value: {:?}", numbers[0]);

    // Re-assign value
    numbers[2] = 21;
    println!("Vector after change: {:?}", numbers);

    //Add to vector
    numbers.push(5);
    numbers.push(6);

    // Pop last value off vector
    numbers.pop();

    // Get length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers)); 

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}