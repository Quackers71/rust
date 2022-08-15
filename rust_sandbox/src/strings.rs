// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {

    let mut hello = String::from("Hello ");

    // get the length
    println!("Length: {}", hello.len());

    // push a char
    hello.push('Q');

    // push a string
    hello.push_str("uackers!");

    println!("{}", hello);
}