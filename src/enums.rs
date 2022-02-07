// types with definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(movement: Movement) {
    // Perform action depending on info
    match movement {
        Movement::Up => println!("Moving up"),
        Movement::Down => println!("Moving down"),
        Movement::Left => println!("Moving left"),
        Movement::Right => println!("Moving right")
    }
}

pub fn run() {
    let avatar_1 = Movement::Left;
    let avatar_2 = Movement::Up;
    let avatar_3 = Movement::Right;
    let avatar_4 = Movement::Down;

    move_avatar(avatar_1);
    move_avatar(avatar_2);
    move_avatar(avatar_3);
    move_avatar(avatar_4);
}