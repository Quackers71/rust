fn main() {
    /* The String Type */

    let mut s = String::from("Hello");

    s.push_str(" World!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
