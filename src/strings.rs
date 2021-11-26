// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own
// string data

pub fn run() {
    // Growable
    let mut hello = String::from("Hello ");
    
    // Get length for either type
    println!("{}", hello.len());
    
    // Push char or str to mutable String
    hello.push('W');
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Empty check
    println!("Is Empty:{}", hello.is_empty());

    // Contains
    println!("Contains World: {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Iterate through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assert testing
    assert_eq!(2, s.len());
    assert_ne!(12, s.capacity());

    println!("{}", hello);
}