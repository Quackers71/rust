fn main() {

    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}", s);

    let x: i32 = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2.clone());
    println!("s1 = {}, s3 = {}", s1, s3);
    println!("s1 address : {:p}", &s1);
    println!("s2 address : {:p}", &s2);
    println!("s3 address : {:p}", &s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello Again!");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

    /* Previous Stuff - The String Type */
    /* let mut s = String::from("Hello");
    s.push_str(" World!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!` */