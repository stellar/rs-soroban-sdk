#![no_std]
use soroban_sdk::{
    contractimpl,
    deploy::{CurrentNamespace, Ed25519Namespace},
    Bytes, BytesN, Env,
};

pub struct Contract;

#[contractimpl]
impl Contract {
    // Note that anyone can create a contract here with any salt, so a users call to
    // this could be frontrun and the same salt taken.
    pub fn create(e: Env, wasm: Bytes, salt: Bytes) {
        let deployer = e.deployer(CurrentNamespace { salt: salt.clone() });
        let _contract_id = deployer.id();
        let _contract_id = deployer.deploy(&wasm);

        let public_key = BytesN::from_array(&e, &[0; 32]);
        let signature = BytesN::from_array(&e, &[0; 64]);
        let namespace = e.deployer(Ed25519Namespace { public_key, salt });
        let _contract_id = namespace.id();
        let _contract_id = namespace.deploy(&wasm, signature);
    }
}
