#![no_main]

use libfuzzer_sys::fuzz_target;

use soroban_sdk::{testutils::arbitrary::{arbitrary,Arbitrary,SorobanArbitrary}, Env, IntoVal, U256};

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

    let _ = client.run(&a, &b);
});
