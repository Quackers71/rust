// Functions and Diverging Functions
// Function 1

pub fn func() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("{}", s);
    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
