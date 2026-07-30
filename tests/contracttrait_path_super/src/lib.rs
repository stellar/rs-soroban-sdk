#![no_std]
use soroban_sdk::{contracttrait, Env};

// =============================================================================
// Test: super:: path (super::Trait)
// Uses super:: prefix to reference a trait from a parent module.
// =============================================================================

#[contracttrait]
pub trait SuperPathTrait {
    fn super_path_method(env: &Env) -> u32 {
        let _ = env;
        300
    }
}

pub mod submodule {
    use soroban_sdk::{contract, contractimpl};

    #[contract]
    pub struct ContractSuperPath;

    #[contractimpl(contracttrait)]
    impl super::SuperPathTrait for ContractSuperPath {}
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_super_path() {
        let e = Env::default();
        let contract_id = e.register(submodule::ContractSuperPath, ());
        let client = submodule::ContractSuperPathClient::new(&e, &contract_id);

        assert_eq!(client.super_path_method(), 300);
    }
}
