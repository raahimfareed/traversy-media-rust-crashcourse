// Checks condition of something and acts on result

pub fn run() {
    let age: u8 = 20;
    let check_id: bool = true;
    let knows_person: bool = true;

    // If else
    if age >= 21 && check_id || knows_person {
        println!("Eligible");
    } else if age < 21 && check_id {
        println!("Too young!");
    } else {
        println!("ID not checked");
    }

    // Shorthand if
    let of_age: bool = if age >= 21 { true } else { false };
    println!("Is of age: {of_age}");
}