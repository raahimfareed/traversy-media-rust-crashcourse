// grouped value, can be mixed types
// max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Raahim", "Rawalpindi", 20);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}