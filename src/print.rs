pub fn run() {
    // Print to console
    println!("Hello from print.rs file");


    // Positional Arguments
    println!("{1} {0}, {{}}", 1, (2.0 + 2.0) / 2.5);


    // Named Arguments
    println!("{name} likes to play {activity}\n", name = "John", activity = "Basketball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 5, 10, 10);

    // Debug trait
    println!("{:?}", (5, "hello", true));

    // Math operations
    println!("10 + 10 = {}", 10 + 10);
}