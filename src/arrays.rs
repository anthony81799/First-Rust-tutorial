// Arrays - Fixed list where elements are the same data types

// Use statements declare namespace to shorten lines where multiple namespaces are used
// repeatedly
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Print whole array
    println!("{:?}", numbers);

    // Indexing
    println!("Single value: {:?}", numbers[0]);

    // Re-assign value
    numbers[2] = 21;
    println!("Array after change: {:?}", numbers);

    // Get length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers)); 

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}