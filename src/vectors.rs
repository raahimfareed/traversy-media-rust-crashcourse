// resizable arrays, same datatypes

// Like c++
// Std lib
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = -2;

    numbers.push(5);
    numbers.push(11);

    // Print all element of array line by line
    println!("{:?}", numbers);
    numbers.pop();
    println!("{:?}", numbers, );

    // Loop through vectors
    for x in numbers.iter() {
        print!("{} ", x);
    }
    println!();

    // loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers, );


    // Print len of vectors
    println!("Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[2..5];
    println!("Slice: {:?}", slice);
}