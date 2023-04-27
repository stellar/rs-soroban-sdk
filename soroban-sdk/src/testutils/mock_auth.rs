#![cfg(any(test, feature = "testutils"))]

use crate::{contractimpl, contracttype, Address, BytesN, Env, RawVal, Symbol, Vec};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MockAuth<'a> {
    pub address: &'a Address,
    pub nonce: u64,
    pub fn_name: &'a str,
    pub args: Vec<RawVal>,
    pub sub_invokes: &'a [MockAuthSubInvoke<'a>],
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MockAuthSubInvoke<'a> {
    pub contract: &'a BytesN<32>,
    pub fn_name: &'a str,
    pub args: Vec<RawVal>,
    pub sub_invokes: &'a [MockAuthSubInvoke<'a>],
}

pub struct MockAuthContract;

#[contracttype(crate_path = "crate")]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthContext {
    pub contract: BytesN<32>,
    pub fn_name: Symbol,
    pub args: Vec<RawVal>,
}

impl MockAuthContract {
    const AUTH_STORAGE_KEY: Symbol = Symbol::short("AUTHS");

    pub fn set_auths(e: Env, auth_contexts: Vec<AuthContext>) {
        e.storage().set(&Self::AUTH_STORAGE_KEY, &auth_contexts)
    }

    pub fn auths(e: Env) -> Vec<AuthContext> {
        e.storage()
            .get(&Self::AUTH_STORAGE_KEY)
            .unwrap_or_else(|| Ok(Vec::new(&e)))
            .unwrap()
    }
}

#[contractimpl(crate_path = "crate")]
impl MockAuthContract {
    #[allow(non_snake_case)]
    pub fn __check_auth(
        e: Env,
        _signature_payload: RawVal,
        _signatures: RawVal,
        auth_context: AuthContext,
    ) {
        if !Self::auths(e.clone())
            .iter()
            .any(|ac| &ac.unwrap() == &auth_context)
        {
            panic!("Not Authorized")
        }
    }
}
