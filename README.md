# rs-stellar-contract-sdk
Rust SDK for writing contracts for [Stellar Jump Cannon].

**This repository contains code that is in early development, incomplete, not tested, and not recommended for use. The API is unstable, experimental, and is receiving breaking changes frequently.**

[Stellar Jump Cannon]: https://stellar.org/blog/smart-contracts-on-stellar-why-now

## Run a contract on the Stellar-Core Jump Cannon prototype

The Stellar-Core Jump Cannon prototype and this SDK are not 100% compatible. Both are still in development and some features may be present in one and not the other. This may cause some example contracts to fail to run on the prototype. However, all examples and tests in this repo should function on the mock host implementation built into the SDK, which is what all tests within the SDK use.

### Prerequisites
1. Install binaryen - https://github.com/WebAssembly/binaryen#building
2. Install wabt - https://github.com/WebAssembly/wabt#cloning
   * This is optional. It contains wasm-objdump, which you can use to dump the contents of a contract. Ex. `wasm-objdump -xh ~/stellar/rs-stellar-contract-sdk/target/wasm32-unknown-unknown/release/example_add.wasm`
3. Make sure binaries from the first two steps are available in your PATH.
4. Install Rust - https://www.rust-lang.org/learn/get-started
5. Checkout and build stellar-core wasm-prototype - https://github.com/graydon/stellar-core/tree/wasm-prototype
6. Build - https://github.com/stellar/rs-stellar-contract-sdk
   * rustup target add wasm32-unknown-unknown
   * make all


### Contract call
Pass one of the built wasm files from rs-stellar-contract-sdk into the stellar-core wasm-prototype
Ex. `stellar-core --conf stellar.conf invoke-contract rs-stellar-contract-sdk/target/wasm32-unknown-unknown/release/example_add.wasm add --arg u63:5 --arg u63:11`
