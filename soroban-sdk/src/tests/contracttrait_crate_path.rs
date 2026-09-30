// Each mod has its own alias for the crate so a crate_path used outside its mod fails to resolve.

mod one {
    use crate::{self as crate1, contracttrait, Env};

    #[contracttrait(crate_path = "crate1")]
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
}

mod two {
    use super::one;
    use crate::{self as crate2, contract, contractimpl, Env};

    #[contract(crate_path = "crate2")]
    pub struct Contract;

    #[contractimpl(crate_path = "crate2", contracttrait)]
    impl one::CratePathTrait for Contract {
        fn overridden_method(env: &Env) -> u32 {
            let _ = env;
            200
        }
    }
}

#[test]
fn test_crate_path() {
    let e = crate::Env::default();
    let contract_id = e.register(two::Contract, ());
    let client = two::ContractClient::new(&e, &contract_id);

    assert_eq!(client.default_method(), 100);
    assert_eq!(client.overridden_method(), 200);
}
