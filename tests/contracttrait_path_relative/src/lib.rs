#![no_std]
use soroban_sdk::{contract, contractimpl};

// =============================================================================
// Test: Relative path (module::Trait) - without crate:: or self::
// Uses a relative path to reference a trait in a sibling module.
// =============================================================================

pub mod traits {
    use soroban_sdk::{contracttrait, Env};

    #[contracttrait]
    pub trait RelativePathTrait {
        fn relative_path_method(env: &Env) -> u32 {
            let _ = env;
            400
        }
    }
}

#[contract]
pub struct ContractRelativePath;

#[contractimpl(contracttrait)]
impl traits::RelativePathTrait for ContractRelativePath {}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_relative_path() {
        let e = Env::default();
        let contract_id = e.register(ContractRelativePath, ());
        let client = ContractRelativePathClient::new(&e, &contract_id);

        assert_eq!(client.relative_path_method(), 400);
    }
}
