use crate::{self as soroban_sdk, BytesN, IntoVal};
use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl,
    crypto::Hash,
    Env, Vec,
};

#[contract]
pub struct Contract;

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    Fail = 1,
}

#[contractimpl]
impl CustomAccountInterface for Contract {
    type Signature = ();
    type Error = Error;

    /// Check that the signatures and auth contexts are valid.
    fn __check_auth(
        _env: Env,
        _signature_payload: Hash<32>,
        _signatures: Self::Signature,
        _auth_contexts: Vec<Context>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn test_functional() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());
    let payload = BytesN::from_array(&e, &[0; 32]);
    let signature = ();
    let auth_context = Vec::new(&e);
    let result = e.try_invoke_contract_check_auth::<Error>(
        &contract_id,
        &payload,
        signature.into_val(&e),
        &auth_context,
    );
    assert_eq!(result, Ok(()));
}
