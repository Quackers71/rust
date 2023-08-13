# Cargo Stuff

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
