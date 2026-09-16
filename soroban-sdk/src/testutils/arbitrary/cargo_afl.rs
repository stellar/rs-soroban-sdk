//! Fuzzing with [`cargo-afl`], which drives [AFL++] and runs on stable Rust.
//!
//! The prototype pattern these examples generate their input with is covered by
//! the [`arbitrary`] module.
//!
//! [`cargo-afl`]: https://github.com/rust-fuzz/afl.rs/
//! [AFL++]: https://aflplus.plus
//! [`arbitrary`]: super
//!
//! Installing it builds AFL++ from source, so a C compiler and LLVM need to be
//! available. `cargo afl system-config` then tunes the machine for fuzzing:
//!
//! ```text
//! cargo install cargo-afl --locked
//! cargo afl system-config
//! ```
//!
//! An AFL++ fuzz target is a crate with a `main` that calls the [`afl::fuzz!`]
//! macro, and it depends on the `arbitrary` crate directly, because that is
//! where the `Arbitrary` derive comes from and where `fuzz!` looks up the trait:
//!
//! ```toml
//! [dependencies]
//! afl = "0.18"
//! arbitrary = { version = "~1.3.0", features = ["derive"] }
//! soroban-sdk = { version = "*", features = ["testutils"] }
//! my-contract = { path = ".." }
//! ```
//!
//! [`afl::fuzz!`]: https://docs.rs/afl/latest/afl/macro.fuzz.html
//!
//! ```
//! # macro_rules! fuzz {
//! #     (|$data:ident: $dty: ty| $body:block) => { };
//! # }
//! use arbitrary::Arbitrary;
//! use soroban_sdk::testutils::arbitrary::SorobanArbitrary;
//! use soroban_sdk::{Address, Env, IntoVal};
//!
//! #[derive(Arbitrary, Debug)]
//! struct TestInput {
//!     deposit_amount: i128,
//!     claim_address: <Address as SorobanArbitrary>::Prototype,
//! }
//!
//! fn main() {
//!     fuzz!(|input: TestInput| {
//!         // Create the `Env` inside the closure, not outside: AFL++ reuses the
//!         // process for many inputs, and state created outside the closure
//!         // would leak from one input into the next.
//!         let env = Env::default();
//!         let claim_address: Address = input.claim_address.into_val(&env);
//!         // fuzz the contract based on the input
//!     });
//! }
//! ```
//!
//! Build with `cargo afl build`, which is `cargo build` with the AFL++
//! instrumentation added, and fuzz the resulting binary with an input directory
//! containing at least one seed input:
//!
//! ```text
//! cargo afl build
//! cargo afl fuzz -i in -o out target/debug/fuzz_target_1
//! ```
//!
//! Fuzz debug builds, at least at first: they keep integer overflow checks and
//! `debug_assert!`s enabled, and those catch bugs a release build allows.
//!
//! Crashing inputs are written to `out/default/crashes/`. The target reads an
//! input on stdin when it is not being driven by AFL++, so a crash is replayed
//! by feeding the file back in:
//!
//! ```text
//! RUST_BACKTRACE=1 ./target/debug/fuzz_target_1 < out/default/crashes/id:000000*
//! ```
