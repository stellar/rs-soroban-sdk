#![cfg(any(test, feature = "testutils"))]

use crate::{contractimpl, contracttype, BytesN, RawVal, Symbol, Vec};

#[contracttype(crate_path = "crate")]
#[derive(Clone)]
pub struct AuthorizationContext {
    pub contract: BytesN<32>,
    pub fn_name: Symbol,
    pub args: Vec<RawVal>,
}

pub struct MockAuthContract;

#[contractimpl(crate_path = "crate")]
impl MockAuthContract {
    #[allow(non_snake_case)]
    pub fn __check_auth(
        _signature_payload: RawVal,
        _signatures: RawVal,
        _auth_context: AuthorizationContext,
    ) {
    }
}
