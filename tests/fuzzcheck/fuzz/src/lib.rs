//! Fuzz tests for a Soroban contract, using fuzzcheck.
//!
//! The fuzz tests are compiled only by `cargo fuzzcheck`, which sets the
//! "fuzzing" cfg and builds with the coverage instrumentation fuzzcheck
//! requires. Run them with a nightly compiler:
//!
//! ```sh
//! cargo +nightly fuzzcheck tests::fuzz_run
//! ```
//!
//! The tests that are not fuzz tests check that the mutators of every builtin
//! and contract type generate values that convert to contract values, and are
//! run by `cargo +nightly test`.
#![feature(coverage_attribute)]

use soroban_sdk::testutils::arbitrary::SorobanArbitrary;
use soroban_sdk::testutils::fuzzcheck::{Mutator, SorobanFuzzcheck};
use soroban_sdk::{Env, IntoVal, Vec};
use test_fuzzcheck::{Contract, ContractClient, Instruction};

#[cfg_attr(not(fuzzing), allow(dead_code))]
type RunInput = <Vec<Instruction> as SorobanArbitrary>::Prototype;

/// Invoke the contract's `run` function with the instructions the fuzzer
/// generated.
#[cfg_attr(not(fuzzing), allow(dead_code))]
fn run(input: &RunInput) {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let instructions: Vec<Instruction> = input.into_val(&env);
    let _ = client.try_run(&instructions);
}

/// Generate a value with the mutator of `T`, and convert it to `T`.
///
/// Panics if the mutator generates a prototype that does not convert.
#[cfg(test)]
fn generate<T>(env: &Env) -> T
where
    T: SorobanFuzzcheck,
    T::Prototype: Clone + 'static,
{
    let mutator = T::soroban_mutator();
    let (prototype, _cplx) = mutator.random_arbitrary(100.0);
    prototype.into_val(env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        crypto::bls12_381::{
            Bls12381Fp, Bls12381Fp2, Bls12381Fr, Bls12381G1Affine, Bls12381G2Affine,
        },
        crypto::bn254::{Bn254Fp, Bn254Fr, Bn254G1Affine, Bn254G2Affine},
        Address, Bytes, BytesN, Duration, Error, Map, MuxedAddress, String, Symbol, Timepoint, Val,
        I256, U256,
    };
    use test_fuzzcheck::{Deposit, Priority, Tagged};

    /// Generate a value of every builtin type with its mutator, so that any
    /// mutator generating a prototype that does not convert to a contract value
    /// is caught.
    #[test]
    fn generate_builtin_types() {
        let env = Env::default();
        for _ in 0..100 {
            generate::<()>(&env);
            generate::<bool>(&env);
            generate::<u32>(&env);
            generate::<i32>(&env);
            generate::<u64>(&env);
            generate::<i64>(&env);
            generate::<u128>(&env);
            generate::<i128>(&env);
            generate::<U256>(&env);
            generate::<I256>(&env);
            generate::<Error>(&env);
            generate::<Bytes>(&env);
            generate::<BytesN<32>>(&env);
            generate::<String>(&env);
            generate::<Symbol>(&env);
            generate::<Address>(&env);
            generate::<MuxedAddress>(&env);
            generate::<Timepoint>(&env);
            generate::<Duration>(&env);
            generate::<Val>(&env);
            generate::<Option<u64>>(&env);
            generate::<Vec<Address>>(&env);
            generate::<Map<String, u32>>(&env);
            generate::<(u32, u64)>(&env);
            generate::<(u32, Bytes, Vec<u32>, Option<i128>)>(&env);
            generate::<Bls12381Fp>(&env);
            generate::<Bls12381Fp2>(&env);
            generate::<Bls12381Fr>(&env);
            generate::<Bls12381G1Affine>(&env);
            generate::<Bls12381G2Affine>(&env);
            generate::<Bn254Fp>(&env);
            generate::<Bn254Fr>(&env);
            generate::<Bn254G1Affine>(&env);
            generate::<Bn254G2Affine>(&env);
        }
    }

    /// Generate a value of the contract types, whose mutators are derived by the
    /// `contracttype` attribute.
    #[test]
    fn generate_contract_types() {
        let env = Env::default();
        for _ in 0..100 {
            generate::<Deposit>(&env);
            generate::<Instruction>(&env);
            generate::<Vec<Instruction>>(&env);
            generate::<Tagged>(&env);
            generate::<Priority>(&env);
        }
    }

    /// Invoke the contract with generated values, checking that the values the
    /// mutators generate are usable as contract function arguments.
    #[test]
    fn invoke_with_generated_values() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        for _ in 0..100 {
            let _ = client.try_builtins(
                &generate(&env),
                &generate(&env),
                &generate(&env),
                &generate(&env),
                &generate(&env),
                &generate(&env),
                &generate(&env),
                &generate(&env),
                &generate(&env),
                &generate(&env),
                &generate(&env),
            );
        }
    }

    /// Fuzz the contract's `run` function, which panics when it is given
    /// instructions that deposit and withdraw the same amount.
    #[cfg(fuzzing)]
    #[test]
    fn fuzz_run() {
        use soroban_sdk::testutils::fuzzcheck::{fuzzcheck, DebugSerializer};

        let result = fuzzcheck::fuzz_test(super::run)
            .mutator(<Vec<Instruction> as SorobanFuzzcheck>::soroban_mutator())
            .serializer(DebugSerializer::default())
            .default_sensor_and_pool()
            .arguments_from_cargo_fuzzcheck()
            .stop_after_first_test_failure(true)
            .launch();
        assert!(!result.found_test_failure);
    }
}
