/**
 * Primitive Types
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
 * Floats: f32, f64
 * Boolean: bool
 * Characters: char
 * Tuples
 * Arrays
 */

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 1236711531;

    // Find max size of a variable
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);

    // Boolean
    // let is_eligible: bool = true;
    let is_eligible = true;
    let is_equal: bool = 100 == 150;

    let ch = 'r';

    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_eligible, is_equal, ch, face));
}