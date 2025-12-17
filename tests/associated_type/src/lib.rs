#![no_std]
use soroban_sdk::{contract, contractimpl, contracttrait, Env, String};

// The associated type to hold a default impl for a trait pattern is a pattern that's seen in the
// OpenZeppelin contract library.

#[contract]
pub struct Contract;

pub struct DefaultImpl;

// The contracttrait is used on this trait and the default fns are linked into the contract
// automatically.

#[contracttrait]
pub trait Trait {
    type Impl: Trait;

    fn exec(env: &Env) -> String {
        Self::Impl::exec(env)
    }
}

impl Trait for DefaultImpl {
    type Impl = Self;
    fn exec(env: &Env) -> String {
        String::from_str(env, "default")
    }
}

#[contractimpl(contracttrait)]
impl Trait for Contract {
    type Impl = DefaultImpl;

    // The default fn does not have to be implemented in the contract because the trait used
    // contracttait and the contractimpl was configured to call it.
}

// The contracttrait is not used on this trait and the default fns are not linked into the contract
// automatically, requiring the user to need to specify the function manually and call the
// implementation.

pub trait TraitWithoutContractTrait {
    type Impl: Trait;

    fn exec2(env: &Env) -> String {
        Self::Impl::exec(env)
    }
}

impl TraitWithoutContractTrait for DefaultImpl {
    type Impl = Self;
    fn exec2(env: &Env) -> String {
        String::from_str(env, "default2")
    }
}

#[contractimpl]
impl TraitWithoutContractTrait for Contract {
    type Impl = DefaultImpl;

    // The default fn has to be implemented in the contract for it to be exported because the trait
    // did not use contracttrait.
    fn exec2(env: &Env) -> String {
        Self::Impl::exec2(env)
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{Env, String};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_exec() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        assert_eq!(client.exec(), String::from_str(&e, "default"));
        assert_eq!(client.exec2(), String::from_str(&e, "default2"));
    }
}

#[cfg(test)]
mod test_with_wasm {
    use soroban_sdk::{Env, String};

    mod contract {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/test_associated_types.wasm"
        );
    }

    #[test]
    fn test_exec() {
        let e = Env::default();
        let contract_id = e.register(contract::WASM, ());
        let client = contract::Client::new(&e, &contract_id);

        assert_eq!(client.exec(), String::from_str(&e, "default"));
        assert_eq!(client.exec2(), String::from_str(&e, "default2"));
    }
}
