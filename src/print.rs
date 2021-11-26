pub fn run() {
    // Print to console
    println!("Hello from the print.rs file.");

    // Use {} to replace in string literal
    // Basic formatting
    println!("{} is from {}", "Anthony", "NJ");

    // Positional parameters
    println!("{0} is from {1} and {0} likes to {2}", "Anthony", "NJ", "code");

    // Named Arguments
    println!("{name} likes to play {activity}.", name = "Anthony", activity = "Basketball");

    // Placeholder  traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 3, 6, 9);

    // Debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math

    println!("10 + 10 = {}", 20 + 10);
}