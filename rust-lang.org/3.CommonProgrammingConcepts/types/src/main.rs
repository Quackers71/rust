use std::io;

/* fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
} */

fn main() {

 /*    /* Scalar Types */

    // Integer Types
    let _guess: u32 = "42".parse().expect("Not a number!");
    println!("The no. is {_guess}");

    // Floating-Point Types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    print!("x = {x} and is a type : ");
    print_type_of(&x);
    print!("y = {y} and is a type : ");
    print_type_of(&y);

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("sum        = {sum}");
    println!("difference = {difference}");
    println!("product    = {product}");
    println!("quotient   = {quotient}");
    println!("truncated  = {truncated}");
    println!("remainder  = {remainder}");

    // Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {t}");
    println!("f = {f}");

    // The Character Type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c = {c}");
    println!("z = {z}");
    println!("heart_eyed_cat = {heart_eyed_cat}");

    /* Compound Types */

    // Tuple Type
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("five_hundred = {five_hundred}");
    println!("six_point_four = {six_point_four}");
    println!("one = {one}"); */

    // Array Types

    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    /* let first = a[0]; // 1
    let second = a[1]; // 2

    let b = [3; 5]; // [3,3,3,3,3]

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"]; */

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    eprint!("Please enter an month (1-12) : ");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");
    
    let index: usize = index.trim().parse().expect("Index entered was not a number!");
    let element = months[index - 1];

    println!("The value of the element at index {index} is : {element}");
}
