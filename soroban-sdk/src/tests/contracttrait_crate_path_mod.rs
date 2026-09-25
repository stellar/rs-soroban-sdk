use crate::{contract, contractimpl, Env};

mod tt {
    use crate::{self as soroban_sdk, contracttrait, Env};

    #[contracttrait]
    pub trait CratePathModTrait {
        fn default_method(env: &Env) -> u32 {
            let _ = env;
            100
        }

        fn overridden_method(env: &Env) -> u32 {
            let _ = env;
            0
        }
    }
}

#[contract(crate_path = "crate")]
pub struct Contract;

#[contractimpl(crate_path = "crate", contracttrait)]
impl tt::CratePathModTrait for Contract {
    fn overridden_method(env: &Env) -> u32 {
        let _ = env;
        200
    }
}

#[test]
fn test_crate_path() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    assert_eq!(client.default_method(), 100);
    assert_eq!(client.overridden_method(), 200);
}
