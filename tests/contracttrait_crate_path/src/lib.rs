#![no_std]
use sdk::{contract, contractimpl, contracttrait, Env};

// =============================================================================
// Test: crate_path with a renamed SDK dependency
// The SDK is depended on as `sdk`, so there is no `soroban_sdk` in scope and
// all generated code, including for non-overridden default functions, must use
// the configured crate_path.
// =============================================================================

#[contracttrait(crate_path = "sdk")]
pub trait CratePathTrait {
    fn default_method(env: &Env) -> u32 {
        let _ = env;
        100
    }

    fn overridden_method(env: &Env) -> u32 {
        let _ = env;
        0
    }
}

#[contract(crate_path = "sdk")]
pub struct Contract;

#[contractimpl(crate_path = "sdk", contracttrait)]
impl CratePathTrait for Contract {
    fn overridden_method(env: &Env) -> u32 {
        let _ = env;
        200
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sdk::Env;

    #[test]
    fn test_crate_path() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        assert_eq!(client.default_method(), 100);
        assert_eq!(client.overridden_method(), 200);
    }
}
