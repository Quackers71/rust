# Paritytech / ink! Stuff

### Parity's ink! for writing smart contracts

- https://github.com/paritytech/ink


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
cargo build contract
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
cargo build contract
```