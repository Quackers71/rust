# Rust

### Learn Rust Programming - freeCodeCamp.org

Learn Rust Programming - Complete Course 🦀
- https://youtu.be/BpPEoZW5IiY?t=5129 currently at 1:25:29

### Documentation Links
- https://github.com/3rfaan/courses/tree/main/Rust/rust-by-practice/src

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
and make sure the function (fn) is made public i.e.
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
cd src/
cargo run
```

### Binary Numbers Stuff

<img src="./images/numbers-integer-types.png" width="500"/><br/>
<img src="./images/numbers-base10.png" width="500"/><br/>
<img src="./images/numbers-base2.png" width="500"/><br/>
<img src="./images/numbers-range-8-bit-integers.png" width="500"/><br/>
<img src="./images/numbers-range-16-bit-integers.png" width="500"/><br/>
<img src="./images/numbers-signed-integers.png" width="500"/><br/>
<img src="./images/numbers-integer-ranges.png" width="500"/><br/>
<img src="./images/numbers-usize-isize.png" width="500"/><br/>
<img src="./images/numbers-what-is-a-word.png" width="500"/><br/>
<img src="./images/numbers-unit-of-data.png" width="500"/><br/>
<img src="./images/numbers-floating-point.png" width="500"/><br/>
https://commons.wikimedia.org/wiki/File:ASCII-Table-wide.svg<br/>
<img src="./images/numbers-ascii-table.png" width="500"/><br/>

<img src="./images/numbers-boolean-logic.png" width="500"/><br/>
<img src="./images/numbers-bitwise-operations.png" width="500"/><br/>
<img src="./images/numbers-bitwise-AND-(&).png" width="500"/><br/>
<img src="./images/numbers-bitwise-OR-(|).png" width="500"/><br/>
<img src="./images/numbers-bitwise-XOR-(^).png" width="500"/><br/>
<img src="./images/numbers-logic-gates.png" width="500"/><br/>
<img src="./images/numbers-bitwise-left-shift.png" width="500"/><br/>
<img src="./images/numbers-bitwise-right-shift.png" width="500"/><br/>

### Char, Bool and Unit Types
<img src="./images/basic-types-recap.png" width="500"/><br/>

### Statements and Expressions
<img src="./images/statements-and-expressions.png" width="500"/><br/>

### Functions
<img src="./images/functions.png" width="500"/><br/>