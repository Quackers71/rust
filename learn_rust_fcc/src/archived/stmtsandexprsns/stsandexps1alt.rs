// Statements and Expressions
// Exercise 1

// Make it work with two ways
pub fn express() {
    let v: () = {
        let mut _x = 1;
        _x += 2 // 3
    };
 
    assert_eq!(v, ());
 
    println!("Success!");
 }
 