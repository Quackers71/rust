use std::io;

fn main() {

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    let x = x + 1;

    let spaces = "   ";

    let float_x = 2.0; // f64
    let float_y: f32 = 3.0; // f32

    println!("The value of x is {x}");

    {
        // Shadowing
        let x = x *2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");

    println!("3 hours in seconds is {}", THREE_HOURS_IN_SECONDS);

    println!("spaces = !{}!", spaces);

    let spaces = spaces.len();
    println!("spaces character length = {}", spaces);

    let _guess: u32 = "42".parse().expect("Not a number!");

    println!("x is {} // default f64", float_x);
    println!("y is {} // set to f32", float_y);

    // Basic Mathematical Operations

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
    let remainder = 43 / 5;

    println!("sum = {}",sum);
    println!("difference = {}", difference);
    println!("product = {}", product);
    println!("quotient = {:.5}",quotient); // 5 decimal places
    println!("truncated = {}",truncated);
    println!("remainder = {}",remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("t = {}", t);
    println!("f = {}", f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c = {}", c);
    println!("z = {}", z);
    println!("heart eyed cat = {}", heart_eyed_cat);

    let mychar = String::from("\u{D7FF}");
    println!("mychar = {}", mychar);

    // Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x1, x2, x3) = tup; // destructuring

    println!("tup = {}, {}, {}", x1, x2, x3);

    let tp: (i32, f64, u8) = (93800, 33.981, 21);
    let ninety_three_gees = tp.0;
    let thirty_three_n_stuff = tp.1;
    let twenny_one = tp.2;

    println!("tp.0: i32 = {}", ninety_three_gees);
    println!("tp.1: f64 = {}", thirty_three_n_stuff);
    println!("tp.2: u8 = {}", twenny_one);

    /*
    This program creates the tuple x and then accesses each element of the tuple using their respective indices. As with most programming languages, the first index in a tuple is 0.

    The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.
    */

    // Arrays

    // let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("a = {:?}", months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);

    let a = [3; 5];
    println!("a = {:?}", a);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("a[0] = {}", first);
    println!("a[1] = {}", second);

    // access element array

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}