// mod tuples;
// mod even;
// mod muts;
// mod arrays;

fn main() {
    /* let a = 10;
    let b = 15;
    println!("Hello, Rust!, a= {} & b= {}", a, b); */

    /* tuples::tuple();
    
    let num = 10;
    println!("your number {} is {}", num, even::is_even(num)); */

   //  muts::muta();

   // arrays::borrowing_slice();

/*    let arr = [0, 1, 2, 3]; // length
   let slice = &arr[1 .. 3]; // [1, 2] don't know the length

   arrays::borrowing_slice(arr, slice); */

    let str: &str = "Hello Rust!";
    let mut string: String = String::from("Hello Rust");

    let slice = &string[.. 6];
    slice.len();

    string.push_str(" ");
    string.push('1');
    string.push_str("! Bob");
    string = string.replace("Hello", "Bye");

    println!("{}", string);
}
