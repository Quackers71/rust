# Rust

### Learn Rust Programming - freeCodeCamp.org

Learn Rust Programming - Complete Course 🦀
- https://youtu.be/BpPEoZW5IiY?t=2531 currently at 42:11

### Main Commands and Rust Language</br></br>
```
cargo init
cargo run
cargo build
cargo build --release
```

To run each program make the appropiate changes to main.rs e.g.
```
mod numbers;

fn main() {
    numbers::number();
}
```
and make sure the number function is made public i.e.
```
// Numbers
// Modify `assert_eq!` to make it work
pub fn number() {
    let x: u32 = 5; // added : u32 
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
```
Then run 
```
cargo run
```

### Binary Numbers Stuff

<img src="./images/numbers-integer-types.png" width="500"/><br/><br/>
<img src="./images/numbers-base10.png" width="500"/><br/><br/>
<img src="./images/numbers-base2.png" width="500"/><br/><br/>
<img src="./images/numbers-range-8-bit-integers.png" width="500"/><br/><br/>
<img src="./images/numbers-range-16-bit-integers.png" width="500"/><br/><br/>
<img src="./images/numbers-signed-integers.png" width="500"/><br/><br/>

<img src="./images/numbers-integer-ranges.png" width="500"/><br/><br/>
<img src="./images/numbers-usize-isize.png" width="500"/><br/><br/>
<img src="./images/numbers-what-is-a-word.png" width="500"/><br/><br/>
<img src="./images/numbers-unit-of-data.png" width="500"/><br/><br/>
<img src="./images/numbers-floating-point.png" width="500"/><br/><br/>
