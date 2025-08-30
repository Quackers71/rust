pub fn muta() {
    let mut num = 5; // everything is immutable (CONSTANT) unless you add 'mut'
    num = 3;
    println!("num is now = {}", num);
}