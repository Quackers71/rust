fn main() {
    /* Variable Scope */
                       // s is not valid here, it’s not yet declared
    let s = "Hello World";   // s is valid from this point forward

    // do stuff with s
                       // this scope is now over, and s is no longer valid

    println!("{s}");
}
