struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorT(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("{} {} {}", c.red, c.green, c.blue);

    c.green = 128;

    println!("{} {} {}", c.red, c.green, c.blue);

    let c_t = ColorT(3, 120, 9);

    println!("{} {} {}", c_t.0, c_t.1, c_t.2);

    let mut p = Person::new("John", "Doe");

    println!("Person: {} {}", p.first_name, p.last_name);

    p.set_last_name("Dane");

    println!("Person: {}", p.full_name());
    println!("{:?}", p.to_tuple());
}