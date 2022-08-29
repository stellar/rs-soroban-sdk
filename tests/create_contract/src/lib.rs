#![no_std]
use soroban_sdk::{contractimpl, Bytes, BytesN, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    // Note that anyone can create a contract here with any salt, so a users call to
    // this could be frontrun and the same salt taken.
    pub fn create(e: Env, wasm: Bytes, salt: Bytes) {
        let deployer = e.deployer().from_current_contract(&salt);
        let _contract_id = deployer.id();
        let _contract_id = deployer.deploy(&wasm);

        let public_key = BytesN::from_array(&e, &[0; 32]);
        let signature = BytesN::from_array(&e, &[0; 64]);
        let deployer = e.deployer().from_ed25519(&public_key, &salt);
        let _contract_id = deployer.id();
        let _contract_id = deployer.deploy(&wasm, signature);
    }
}
