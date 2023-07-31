// Statements and Expressions
// Exercise 1

// Make it work with two ways
pub fn express() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }
 