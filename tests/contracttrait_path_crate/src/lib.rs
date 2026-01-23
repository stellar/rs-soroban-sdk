#![no_std]
use soroban_sdk::{contract, contractimpl};

// =============================================================================
// Test: crate:: path (crate::module::Trait)
// Uses crate:: prefix to reference a trait in another module.
// =============================================================================

pub mod traits {
    use soroban_sdk::{contracttrait, Env};

    #[contracttrait]
    pub trait CratePathTrait {
        fn crate_path_method(env: &Env) -> u32 {
            let _ = env;
            100
        }
    }
}

#[contract]
pub struct ContractCratePath;

#[contractimpl(contracttrait)]
impl crate::traits::CratePathTrait for ContractCratePath {}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_crate_path() {
        let e = Env::default();
        let contract_id = e.register(ContractCratePath, ());
        let client = ContractCratePathClient::new(&e, &contract_id);

        assert_eq!(client.crate_path_method(), 100);
    }
}
