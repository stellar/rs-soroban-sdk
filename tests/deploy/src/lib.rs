#![no_std]
use soroban_sdk::{contract, contractimpl, BytesN, Env};

mod deployable {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/test_constructor.wasm"
    );
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn deploy(env: &Env) {
        let hash = env.deployer().upload_contract_wasm(deployable::WASM);
        env.deployer()
            .with_current_contract(BytesN::from_array(env, &[0; 32]))
            .deploy_v2(hash, deployable::Args::__constructor(&1, &2));
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_deploy() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        client.deploy();
    }
}
