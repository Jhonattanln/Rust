const X: i32 = 1;

fn main() {
    println!("Hello, world!");
    println!("The first number is {}", X);

    // Tuplas
    const TUPLA_E: (char, i32, f64, bool) = ('E', 5, 5.5, true);
    println!("Tuplas: {}, {}, {}, {}", TUPLA_E.0, TUPLA_E.1, TUPLA_E.2, TUPLA_E.3);

    print_function();
}

// structs
struct Clubs {
    club_name: &'static str,
    birth: i32,
    bianconeri: f64
}

static JUVENTUS: Clubs = Clubs {
    club_name: "Juventus",
    birth: 1897,
    bianconeri: 0.5
};

fn print_function() {
    println!("Structs: {}, {}, {}", JUVENTUS.club_name, JUVENTUS.birth, JUVENTUS.bianconeri);
}