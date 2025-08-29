use std::collections::HashMap;

// Hash Map - objects to store data in a key value pairing 

fn main() {
    let mut map = HashMap::new();

    map.insert(0, "Hi");
    map.insert(1, "Hi2");
    println!("{:?}", map);

    match map.get(&0) {
        Some(str1) => println!("{}", str1),
        None => println!("Doesn't exist in map"),
    }

    match map.get(&2) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map"),
    }

    map.remove(&0);
    map.insert(2, "Hi3");
    map.insert(5, "Hello Rust!");
    println!("{:?}", map);
}
