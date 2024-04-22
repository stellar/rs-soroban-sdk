#![no_std]
use soroban_sdk::{
    auth::Context, auth::CustomAccountInterface, contract, contracterror, contractimpl,
    crypto::Hash, Env, Vec,
};

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    Fail = 1,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl CustomAccountInterface for Contract {
    type Error = Error;
    type Signature = ();
    #[allow(non_snake_case)]
    fn __check_auth(
        _env: Env,
        _signature_payload: Hash<32>,
        _signatures: (),
        _auth_contexts: Vec<Context>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{
        contract,
        testutils::{MockAuth, MockAuthInvoke},
        Env, IntoVal,
    };

    use crate::Contract;

    #[contract]
    struct TestContract;

    #[test]
    fn test() {
        let e = Env::default();
        let test_contract_id = e.register_contract(None, TestContract);
        let contract_id = e.register_contract(None, Contract);

        e.set_auths(&[MockAuth {
            address: &contract_id,
            invoke: &MockAuthInvoke {
                contract: &test_contract_id,
                fn_name: "",
                args: ().into_val(&e),
                sub_invokes: &[],
            },
        }
        .into()]);

        e.as_contract(&test_contract_id, || contract_id.require_auth());
    }
}
