#![no_main]

// TODO: HELP!
// This contract sees the following build errors and I'm not sure what the right
// way to fix them is:
//
// ❯ cargo +nightly fuzz build
// Compiling test_fuzz v0.8.4 (/Users/leighmcculloch/Code/rs-soroban-sdk/tests/fuzz)
// Compiling test_fuzz-fuzz v0.0.0 (/Users/leighmcculloch/Code/rs-soroban-sdk/tests/fuzz/fuzz)
// error[E0433]: failed to resolve: could not find `size_hint` in `arbitrary`
// --> fuzz_targets/fuzz_target_1.rs:18:10
// |
// 18 | #[derive(Arbitrary, Debug)]
// |          ^^^^^^^^^ could not find `size_hint` in `arbitrary`
// |
// = note: this error originates in the derive macro `Arbitrary` (in Nightly builds, run with -Z macro-backtrace for more info)
//
// error[E0412]: cannot find type `Unstructured` in module `arbitrary`
// --> fuzz_targets/fuzz_target_1.rs:18:10
// |
// 18 | #[derive(Arbitrary, Debug)]
// |          ^^^^^^^^^ not found in `arbitrary`
// |
// = note: this error originates in the derive macro `Arbitrary` (in Nightly builds, run with -Z macro-backtrace for more info)
//
// error[E0412]: cannot find type `Result` in module `arbitrary`
// --> fuzz_targets/fuzz_target_1.rs:18:10
// |
// 18 | #[derive(Arbitrary, Debug)]
// |          ^^^^^^^^^ not found in `arbitrary`
// |
// = note: this error originates in the derive macro `Arbitrary` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider importing one of these items
// |
// 3  + use core::fmt::Result;
// |
// 3  + use core::result::Result;
// |
// 3  + use std::fmt::Result;
// |
// 3  + use std::io::Result;
// |
// and 2 other candidates
// help: if you import `Result`, refer to it directly
// |
// 18 | #[derive(Arbitrary, Debug)]
// |
//
// error[E0433]: failed to resolve: could not find `Error` in `arbitrary`
// --> fuzz_targets/fuzz_target_1.rs:18:10
// |
// 18 | #[derive(Arbitrary, Debug)]
// |          ^^^^^^^^^ could not find `Error` in `arbitrary`
// |
// = note: this error originates in the derive macro `Arbitrary` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider importing one of these items
// |
// 3  + use core::error::Error;
// |
// 3  + use core::fmt::Error;
// |
// 3  + use soroban_sdk::Error;
// |
// 3  + use soroban_sdk::testutils::ed25519::Error;
// |
// and 4 other candidates
// help: if you import `Error`, refer to it directly
// |
// 18 | #[derive(Arbitrary, Debug)]
// |

use libfuzzer_sys::fuzz_target;

use soroban_sdk::{
    // We need to import `arbitrary` because the `derive(Arbitrary)` code seems
    // to need it. TODO: Is there a way to remove it from being imported?
    arbitrary,
    arbitrary::Arbitrary,
    arbitrary::SorobanArbitrary,
    Env,
    IntoVal,
    U256,
};

use test_fuzz::{Contract, ContractClient};

#[derive(Arbitrary, Debug)]
struct Input {
    a: <U256 as SorobanArbitrary>::Prototype,
    b: <U256 as SorobanArbitrary>::Prototype,
}

fuzz_target!(|input: Input| {
    let env = Env::default();

    let a: U256 = input.a.into_val(&env);
    let b: U256 = input.b.into_val(&env);

    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let _ = client.add(&a, &b);
});
