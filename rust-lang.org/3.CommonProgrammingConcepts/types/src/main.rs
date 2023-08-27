fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    /* Scalar Types */

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

    
}
