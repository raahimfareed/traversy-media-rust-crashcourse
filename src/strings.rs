// Primitive str: Immutable, fixed length
// String: Growable, Heap allocated data structure

pub fn run() {
    let hello = "Hello";
    let mut hello_2 = String::from("Hello");

    println!("Str: {hello}\nString: {hello_2}");
    
    hello_2.push('w');
    // hello_2.push(" no");

    hello_2.push_str(" dooods");
    
    // Get length
    println!("Str: {hello}\nString: {hello_2}");
    println!("{}", hello_2.len());

    // Capacity in byte
    println!("Cap: {}", hello_2.capacity());

    // Check if string is empty
    println!("String is empty: {}", hello_2.is_empty());

    // contains hello
    println!("String contains \"Hello\": {}", hello_2.contains("Hello"));

    // replace substr
    println!("Replace \"Hello\": {}", hello_2.replace("Hello", "Bye"));
    println!("{}", hello_2);

    for token in hello_2.split_whitespace() {
        println!("{token}");
    }

    // println!("{:?}", hello_2.split_whitespace());

    // Create string with certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}