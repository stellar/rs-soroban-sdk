use crate::admin::Administratable;
use soroban_sdk::contracttrait;

#[contracttrait]
pub trait Upgradable: Administratable {
    fn upgrade(env: &soroban_sdk::Env, wasm_hash: soroban_sdk::BytesN<32>) {
        Self::require_admin(env);
        env.deployer().update_current_contract_wasm(wasm_hash);
    }
}
