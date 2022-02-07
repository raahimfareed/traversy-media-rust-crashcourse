pub fn run() {
    let name = "Doe";

    // Short hand for println!("My name is {name}", name=name);
    println!("My name is {name}");

    let mut age = 21;
    println!("My name is {name} and I am {age} years old");

    age = 38;
    println!("My name is {name} and I am {age} years old");

    // Define constant
    const ID: i32 = 001;

    println!("ID: {ID}");

    // Multiple variables at once
    let ( my_name, my_age ) = ( "Raahim", 20 );
    println!("{my_name} is {my_age}");
}