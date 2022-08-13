// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped la&nguage

pub fn run() {
    let name = "Q";
    let mut age = 51; // age variable is now mutable
    println!("My name is {} and I am {}", name, age);

    age = 52;
    println!("My name is {} and I am {}", name, age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let ( my_name, my_age ) = ("Q", 51);
    println!("{} is {}", my_name, my_age);
}