// Char, Bool and Unit
// No.1 - Char

// Make it work
use std::mem::size_of_val;
pub fn char() {
    let c1: char = 'a'; // 4 bytes
    // assert_eq!(size_of_val(&c1),1);
    println!("{}", size_of_val(&c1)); // output amount in bytes

    // let c2: char = '中';
    // assert_eq!(size_of_val(&c2),3); 

    println!("Success!");
} 
