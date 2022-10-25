extern crate std;

use soroban_sdk::{
    contractimpl, symbol,
    testutils::{ed25519::Sign, Ledger, LedgerInfo},
    BytesN, Env,
};

use crate::{
    testutils::ed25519::{generate, sign, Identifier},
    verify, Signature, SignaturePayload,
};

pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    pub fn examplefn(env: Env, sig1: Signature, sig2: Signature, arg1: i32, arg2: i32) {
        verify(
            &env,
            &sig1,
            symbol!("examplefn"),
            (&sig1.identifier(&env), &sig2.identifier(&env), arg1, arg2),
        );
        verify(
            &env,
            &sig1,
            symbol!("examplefn"),
            (&sig1.identifier(&env), &sig2.identifier(&env), arg1, arg2),
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

    let (id1, signer1) = generate(&env);
    std::println!("id1: {:?}", id1);
    std::println!("signer1: {:?}", signer1);

    let (id2, signer2) = generate(&env);
    std::println!("id2: {:?}", id2);
    std::println!("signer2: {:?}", signer2);

    let (sig1, sig2) = both_sign(&env, &contract_id, signer1, signer2, 1, 2);
    std::println!("signature1: {:?}", sig1);
    std::println!("signature2: {:?}", sig2);

    client.examplefn(&sig1, &sig2, &1, &2);
}

fn both_sign<S>(
    env: &Env,
    contract_id: &BytesN<32>,
    signer1: S,
    signer2: S,
    arg1: i32,
    arg2: i32,
) -> (Signature, Signature)
where
    S: Identifier + Sign<SignaturePayload, Signature = [u8; 64]>,
{
    (
        sign(
            &env,
            &signer1,
            &contract_id,
            symbol!("examplefn"),
            (
                &signer1.identifier(env),
                &signer2.identifier(env),
                &arg1,
                &arg2,
            ),
        ),
        sign(
            &env,
            &signer2,
            &contract_id,
            symbol!("examplefn"),
            (
                &signer1.identifier(env),
                &signer2.identifier(env),
                &arg1,
                &arg2,
            ),
        ),
    )
}
