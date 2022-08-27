#![no_std]
use soroban_sdk::{contractimpl, Bytes, BytesN, CurrentNamespace, Ed25519Namespace, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    // Note that anyone can create a contract here with any salt, so a users call to
    // this could be frontrun and the same salt taken.
    pub fn create(e: Env, wasm: Bytes, salt: BytesN<32>) {
        let _contract_id = e.contract_id(CurrentNamespace, &salt);
        let _contract_id = e.deployer(CurrentNamespace).deploy_wasm(&salt, &wasm);

        let public_key = BytesN::from_array(&e, &[0; 32]);
        let signature = BytesN::from_array(&e, &[0; 64]);
        let namespace = Ed25519Namespace { public_key };
        let _contract_id = e.contract_id(&namespace, &salt);
        let _contract_id = e.deployer(&namespace).deploy_wasm(&salt, &wasm, signature);
    }
}
