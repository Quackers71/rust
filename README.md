# Rust

### Rust Courses

Rust Crash Course | Rustlang via Traversy Media
- https://www.youtube.com/watch?v=zF34dRivLOw

Building a Rust App with Yew! via Let's Get Rusty
- https://www.youtube.com/watch?v=KmOeFrwz8BM

#### Commands for Rust App with Yew!
```
rustc --version
rustup target add wasm32-unknown-unknown
cargo install trunk
# Output : Installed package `trunk v0.16.0` (executable `trunk`)
To start the app run :
trunk serve
# Then goto localhost:8080
```
</br>
Rust Fundamentals | PluralSight
- https://www.pluralsight.com/courses/rust-fundamentals

### Rust-lang.org Setup
https://www.rust-lang.org/learn/get-started
```
- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 'option 1'
```
Then restart the terminal</br></br>

### Main Commands and Rust Language</br></br>
```
rustup --version
rustup update
rustc --version
cargo --version

mkdir rust_sandbox
cargo init
cargo run
cargo build
cargo build --release
```

### Variables
Variables hold primitive data or references to data</br>
Variables are immutable by default</br>
Rust is a block-scoped language</br></br>

### Data Types
Primitive Types--</br>
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)</br>
Floats: f32, f64</br>
Boolean (bool)</br>
Characters (char)</br>
Tuples</br>
Arrays</br></br>

Rust is a statically typed language, which means that it must know the types of all</br>
variables at compile time. However, the compiler can usually infer what type we want to</br>
use based on the value and  how we use it.</br></br>

### Strings
Primitive str = Immutable fixed-length string somewhere in memory</br>
String = Growable, heap-allocated data structure - Use when you need to modify or own string data</br></br>

### Tuples
Tuples group together values of different types</br>
Max 12 elements</br></br>

### Arrays
Arrays - Fixed list where elements are the same data types</br></br>

### Vectors
Vectors - Resizable arrays</br></br>

### Conditionals
Conditionals - used to check the condition of something and act on the result</br></br>

### Loops
Loops - used to iterate until a condition is met</br></br>

### Functions
Functions - used to store block of code for re-use</br></br>

### Reference Pointers
Reference Pointers - Point to a resource in memory</br></br>

With non-primitives, if you assign another variable to a piece of data, the first</br>
variable will no longer hold that value.  You'll need to use a reference (&) to</br>
point to the resource...</br></br>

### Structs
Structs - used to create custom data types</br></br>

### Enums
Enums are types which have a few definite values</br></br>

### Command Line Args
See the file "cli.rs"</br></br></br>
