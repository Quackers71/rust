// Statements and Expressions
// Exercise 3

pub fn express() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("{}", s);
    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}


 