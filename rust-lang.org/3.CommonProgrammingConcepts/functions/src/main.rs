// a function with a return value
/* fn five() -> i32 {
    5
} */

fn main() {
    println!("Hello, world!");

    // Parameters (special variables i.e. x: i32) and arguments (concrete values i.e. 5)
    another_function(5);

    // a function signature, declaring each parameter
    print_labeled_measurements(5, 'h');

    // Statements and Expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let z = plus_one(5);
    println!("The value of z is: {z}");

    let new_sum: i32 = add_function(101, 22);
    println!("The total is : {}", new_sum);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn add_function(f: i32, g: i32) -> i32 {
    println!("The value of f is: {}", f);
    println!("The value of g is: {}", g);
    f + g
}