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

    // capacity in bytes
    println!("Capacity: {} bytes", hello.capacity());

    // check if empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains 'Quackers': {}", hello.contains("Quackers"));

    // replace
    println!("Replace: {}", hello.replace("Quackers", "Quacks"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    // println!("{}", hello);
}