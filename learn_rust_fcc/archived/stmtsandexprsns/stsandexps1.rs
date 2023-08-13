// Statements and Expressions
// Exercise 1

// Make it work with two ways
pub fn express() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2; // 3
        x
    };
 
    assert_eq!(v, 3);
    println!("{}", v);
 
    println!("Success!");
 }
 