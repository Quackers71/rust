# Cargo Stuff

### Information
By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it [in the standard library documentation](https://doc.rust-lang.org/std/prelude/index.html).

### Install Rust and Cargo
- https://doc.rust-lang.org/cargo/getting-started/installation.html

### Main Commands</br>
- Install Rust and Cargo
  - https://doc.rust-lang.org/cargo/getting-started/installation.html<br/><br/>

- Creating a New Package
```
cargo new hello_world --bin
```
<span style="color:darkgreen;">
We’re passing --bin because we’re making a binary program: if we were making a library, we’d pass --lib. This also initializes a new git repository by default. If you don’t want it to do that, pass --vcs none. i.e.
```
</span></br></br>
cargo new hello_world --lib --vcs
```

- Build and Compile using :
```
cd hello_world
cargo build
```

- Then run the program using :
```
./target/debug/hello_world
```
- or
```
cargo run
```
- Once your ready to release use :
```
cargo build --release
   Compiling hello_world v0.1.0 (/Users/robq/workspace/rust/cargo/hello_world)
    Finished release [optimized] target(s) in 0.76s
```
- then :
```
./target/release/hello_world
# output
# Hello, world!
```
<span style="color:darkgreen;">
Compiling in debug mode is the default for development. Compilation time is shorter since the compiler doesn’t do optimizations, but the code will run slower. Release mode takes longer to compile, but the code will run faster.
</span></br></br>

You can build a project without producing a binary to check for errors using :
```
cargo check
```

### Notes

From Programming a Guessing Game (Specically regarding Line 19): <br/>
```
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
  We create a variable named guess. But wait, doesn’t the program already have a variable named<br/> 
  guess? It does, but helpfully Rust allows us to shadow the previous value of guess with a new one.<br/> 
  Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables,<br/> 
  such as guess_str and guess, for example. We’ll cover this in more detail in Chapter 3, but for now,<br/> 
  know that this feature is often used when you want to convert a value from one type to another type.<br/>
  