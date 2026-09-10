//! Fuzzing with [`cargo-fuzz`], which drives libFuzzer and requires a nightly
//! compiler.
//!
//! The prototype pattern these examples generate their input with is covered by
//! the [`arbitrary`] module.
//!
//! [`cargo-fuzz`]: https://github.com/rust-fuzz/cargo-fuzz/
//! [`arbitrary`]: super
//!
//! ```text
//! cargo install cargo-fuzz --locked
//! ```
//!
//! A libFuzzer fuzz target is its own crate, which `cargo fuzz init` creates in
//! a `fuzz` directory, containing targets that call the [`fuzz_target!`] macro:
//!
//! ```toml
//! [dependencies]
//! libfuzzer-sys = "0.4"
//! soroban-sdk = { version = "*", features = ["testutils"] }
//! my-contract = { path = ".." }
//! ```
//!
//! [`fuzz_target!`]: https://docs.rs/libfuzzer-sys/latest/libfuzzer_sys/macro.fuzz_target.html
//!
//! ```
//! # macro_rules! fuzz_target {
//! #     (|$data:ident: $dty: ty| $body:block) => { };
//! # }
//! use soroban_sdk::testutils::arbitrary::{Arbitrary, SorobanArbitrary};
//! use soroban_sdk::{Address, Env, IntoVal};
//!
//! #[derive(Arbitrary, Debug)]
//! struct TestInput {
//!     deposit_amount: i128,
//!     claim_address: <Address as SorobanArbitrary>::Prototype,
//! }
//!
//! fuzz_target!(|input: TestInput| {
//!     let env = Env::default();
//!     let claim_address: Address = input.claim_address.into_val(&env);
//!     // fuzz the contract based on the input
//! });
//! ```
//!
//! Build and fuzz the target, from the `fuzz` directory:
//!
//! ```text
//! cargo +nightly fuzz run fuzz_target_1
//! ```
//!
//! Crashing inputs are written to `artifacts/fuzz_target_1/`, and a crash is
//! replayed by passing the file back to the same command:
//!
//! ```text
//! RUST_BACKTRACE=1 cargo +nightly fuzz run fuzz_target_1 artifacts/fuzz_target_1/crash-*
//! ```
