extern crate std;

use core::str::FromStr;

use soroban_sdk::{
    contractimpl, symbol,
    testutils::{Ledger, LedgerInfo},
    BytesN, Env,
};

use crate::{
    testutils::ed25519::{generate, sign, signer},
    verify, Signature,
};

pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn examplefn(env: Env, sig: Signature, arg1: i32, arg2: i32) {
        verify(
            &env,
            &sig,
            symbol!("examplefn"),
            (&sig.identifier(&env), arg1, arg2),
        );
    }
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    env.ledger().set(LedgerInfo {
        base_reserve: 0,
        network_passphrase: "soroban-auth test".as_bytes().to_vec(),
        protocol_version: 0,
        sequence_number: 0,
        timestamp: 0,
    });

    std::println!("network: {:?}", env.ledger().network_passphrase());
    std::println!("contract id: {:?}", contract_id);
    std::println!("name: {:?}", symbol!("examplefn"));

    let (id, signer) = generate(&env);
    std::println!("signer: {:?}", signer);
    std::println!("id: {:?}", id);
    let sig = sign(
        &env,
        &signer,
        &contract_id,
        symbol!("examplefn"),
        (&id, &1, &2),
    );
    std::println!("signature: {:?}", sig);

    client.examplefn(&sig, &1, &2);
}

#[test]
fn test_build_keypair() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    env.ledger().set(LedgerInfo {
        base_reserve: 0,
        network_passphrase: "soroban-auth test".as_bytes().to_vec(),
        protocol_version: 0,
        sequence_number: 0,
        timestamp: 0,
    });

    std::println!("network: {:?}", env.ledger().network_passphrase());
    std::println!("contract id: {:?}", contract_id);
    std::println!("name: {:?}", symbol!("examplefn"));

    let key = stellar_strkey::StrkeyPrivateKeyEd25519::from_str(
        "SC24O4H2LT4PVOYCWMKUSD2DL4UL26IYGPFKANDH7S4MU6JVQEFOS7DC",
    )
    .unwrap();
    let (id, signer) = signer(&env, &key.0);
    std::println!("signer: {:?}", signer);
    std::println!("id: {:?}", id);
    let sig = sign(
        &env,
        &signer,
        &contract_id,
        symbol!("examplefn"),
        (&id, &1, &2),
    );
    std::println!("signature: {:?}", sig);

    client.examplefn(&sig, &1, &2);
}
