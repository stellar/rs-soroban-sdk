#![no_std]
use soroban_sdk::{contract, contractimpl, contracttrait, Env};

// =============================================================================
// Test: self:: path (self::Trait)
// Uses self:: prefix to reference a trait in the current module.
// =============================================================================

#[contracttrait]
pub trait SelfPathTrait {
    fn self_path_method(env: &Env) -> u32 {
        let _ = env;
        200
    }
}

#[contract]
pub struct ContractSelfPath;

#[contractimpl(contracttrait)]
impl self::SelfPathTrait for ContractSelfPath {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_self_path() {
        let e = Env::default();
        let contract_id = e.register(ContractSelfPath, ());
        let client = ContractSelfPathClient::new(&e, &contract_id);

        assert_eq!(client.self_path_method(), 200);
    }
}
