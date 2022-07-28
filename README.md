# rs-soroban-sdk
Rust SDK for writing contracts for [Soroban].

**This repository contains code that is in early development, incomplete, not tested, and not recommended for use. The API is unstable, experimental, and is receiving breaking changes frequently.**

[Soroban]: https://soroban.stellar.org

## Run a contract on the Stellar-Core Soroban prototype

The Stellar-Core Soroban prototype and this SDK are not 100% compatible. Both are still in development and some features may be present in one and not the other. This may cause some example contracts to fail to run on the prototype. However, all examples and tests in this repo should function on the mock host implementation built into the SDK, which is what all tests within the SDK use.

### Prerequisites
1. Install binaryen - https://github.com/WebAssembly/binaryen#building
2. Install wabt - https://github.com/WebAssembly/wabt#cloning
   * This is optional. It contains wasm-objdump, which you can use to dump the contents of a contract. Ex. `wasm-objdump -xh ~/stellar/rs-soroban-sdk/target/wasm32-unknown-unknown/release/example_add.wasm`
3. Make sure binaries from the first two steps are available in your PATH.
4. Install Rust - https://www.rust-lang.org/learn/get-started
5. Build - https://github.com/stellar/rs-soroban-sdk
   * rustup target add wasm32-unknown-unknown
   * make all
