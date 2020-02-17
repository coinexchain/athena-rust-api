# Athena Rust API
Rust API for CoinEx Smart Chain.



## Build

1. Install [Rust](https://www.rust-lang.org/) toolchain with [rustup](https://rustup.rs/)

2. Install Wasm backend:

   ```bash
   $ rustup target add wasm32-unknown-unknown
   ```

3. Build API and examples:

   ```bash
$ git clone https://github.com/coinexchain/athena-rust-api.git
   $ cd athena-rust-api
   $ cargo build --examples --target wasm32-unknown-unknown
   ```


