# rs-stellar-contract-sdk
Rust SDK for writing contracts for [Stellar Jump Cannon].

This SDK is in early development. The API is unstable, experimental, and breaking changes are being committed frequently.

[Stellar Jump Cannon]: https://stellar.org/blog/smart-contracts-on-stellar-why-now

# Run a contract

## Prerequisites
1. Install wasm  - If building from source, make sure binaries are available in your PATH
   * Install binaryen - https://github.com/WebAssembly/binaryen#building
   * Install wabt - https://github.com/WebAssembly/wabt#cloning
     * This is optional. It contains wasm-objdump, which you can use to dump the contents of a contract. Ex. `wasm-objdump -xh ~/stellar/rs-stellar-contract-sdk/target/wasm32-unknown-unknown/release/test_add.wasm`
2. Install Rust - https://www.rust-lang.org/learn/get-started
3. Checkout and build stellar-core wasm-prototype - https://github.com/graydon/stellar-core/tree/wasm-prototype
4. Build - https://github.com/stellar/rs-stellar-contract-sdk
   * rustup target add wasm32-unknown-unknown
   * make all


## Contract call
Pass one of the built wasm files from rs-stellar-contract-sdk into the stellar-core wasm-prototype
Ex. `stellar-core --conf stellar.conf invoke-contract rs-stellar-contract-sdk/target/wasm32-unknown-unknown/release/test_add.wasm add --arg u63:5 --arg u63:11`
