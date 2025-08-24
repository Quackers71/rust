mod tuples;
mod even;

fn main() {
    /* let a = 10;
    let b = 15;
    println!("Hello, Rust!, a= {} & b= {}", a, b); */

    tuples::tuple();
    
    let num = 10;
    println!("your number {} is {}", num, even::is_even(num));
}
