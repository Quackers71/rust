# Paritytech / ink! Stuff

### Parity's ink! for writing smart contracts
- https://github.com/paritytech/ink
- https://use.ink/getting-started/setup/

Other documentation links :
- https://docs.substrate.io/quick-start/
- https://use.ink
- https://paritytech.github.io/ink/ink/

### Install Rust and Cargo
- https://doc.rust-lang.org/cargo/getting-started/installation.html

### Main Commands</br>
- Install cargo-contract CLI tool 
```
cargo install cargo-contract --force
```
- Initialise a new ink! project
```
cargo contract new flipper
```
- Build the contract
```
cd flipper
cargo contract build
```

### Troubleshooting<br/>
If you receive the below error :<br/>
<span style="color:red;">
ERROR: Loading of original wasm failed</br>
Caused by:</br>
    0: Loading of wasm module at '/Users/robq/workspace/rust/flipper/target/ink/wasm32-unknown-unknown/release/flipper.wasm' failed</br>
    1: Unknown opcode 192
</span>

- Use the below commands to rectify the above error
```
rustup update stable
rustup install 1.69
rustup default 1.69
rustup component add rust-src --toolchain 1.69
rustup target add wasm32-unknown-unknown --toolchain 1.69
```
taken from https://substrate.stackexchange.com/questions/7881/error-loading-of-original-wasm-failed<br/>
- Retry - Build the contract
```
cargo contract build
```