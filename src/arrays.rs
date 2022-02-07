// fixed length, same datatypes

// Like c++
// Std lib
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[2] = -2;

    // Print all element of array line by line
    for number in numbers {
        print!("{} ", number);
    }

    println!();

    // Print len of arrays
    println!("Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[2..5];
    println!("Slice: {:?}", slice);
}