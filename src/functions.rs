pub fn run() {
    greeting("Hello", "Raahim");
    
    // Bind function values to variable
    let get_sum = add(5, 5);
    println!("Sum: {get_sum}");

    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(4, 5));
}

fn greeting(greet: &str, name: &str) {
    println!("{greet} {name}");
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
