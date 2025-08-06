#![no_std]

use test_contracttrait_lib::{Admin, Administratable, Pausable, Upgradable};

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Administratable for Contract {
    type Impl = Admin;
}

#[contractimpl]
impl Upgradable for Contract {}

#[contractimpl]
impl Pausable for Contract {}

#[contractimpl]
impl Contract {
    pub fn __constructor(env: &Env, admin: soroban_sdk::Address) {
        Self::set_admin(env, &admin);
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
    use soroban_sdk::{Address, Env, IntoVal};

    use crate::{Contract, ContractClient};

    #[test]
    fn pause() {
        let e = &Env::default();
        let admin = Address::generate(e);
        let contract_id = e.register(Contract, (admin.clone(),));
        let client = ContractClient::new(e, &contract_id);
        assert!(!client.is_paused());
        e.mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &client.address,
                fn_name: "pause",
                args: ().into_val(e),
                sub_invokes: &[],
            },
        }]);
        client.pause();
        assert!(client.is_paused());
    }
}
