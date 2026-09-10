use afl::fuzz;
use arbitrary::Arbitrary;

use soroban_sdk::{testutils::arbitrary::SorobanArbitrary, Env, IntoVal, U256};

use test_fuzz_afl::{Contract, ContractClient};

#[derive(Arbitrary, Debug)]
struct Input {
    a: <U256 as SorobanArbitrary>::Prototype,
    b: <U256 as SorobanArbitrary>::Prototype,
}

fn main() {
    // Every iteration gets a fresh `Env` so that no ledger state leaks from one
    // input to the next. AFL++ runs the body of `fuzz!` in a persistent loop,
    // so anything created outside of it would be shared between inputs.
    fuzz!(|input: Input| {
        let env = Env::default();

        let a: U256 = input.a.into_val(&env);
        let b: U256 = input.b.into_val(&env);

        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let _ = client.run(&a, &b);
    });
}
