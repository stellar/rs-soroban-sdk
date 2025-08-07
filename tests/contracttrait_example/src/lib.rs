#![no_std]

use test_contracttrait_lib::{Administratable, Pausable, PausableBase, Upgradable};

use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, Event};

#[contractevent( topics = ["pause"], data_format = "vec", export = false)]
pub struct Pause {
    #[topic]
    pub by: Address,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Administratable for Contract {}

#[contractimpl]
impl Upgradable for Contract {}

#[contractimpl]
impl Pausable for Contract {
    type Impl = PausableBase;
    fn pause(env: &Env) {
        let by = Self::admin(env);
        env.events().publish_event(&Pause { by });
        Self::Impl::pause(env);
    }
}

#[contractimpl]
impl Contract {
    pub fn __constructor(env: &Env, admin: soroban_sdk::Address) {
        Self::set_admin(env, &admin);
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::testutils::{Address as _, Events, MockAuth, MockAuthInvoke};
    use soroban_sdk::{Address, Env, FromVal, IntoVal, Symbol};

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
        assert_eq!(
            soroban_sdk::symbol_short!("pause"),
            Symbol::from_val(
                e,
                &e.events()
                    .all()
                    .try_first()
                    .unwrap()
                    .unwrap()
                    .1
                    .get(0)
                    .unwrap()
            )
        );
        assert!(client.is_paused());
    }
}
