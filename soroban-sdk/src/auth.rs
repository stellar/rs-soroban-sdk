use crate::{contracttype, Address, BytesN, Env, RawVal, Status, Symbol, Vec};

/// Auth context that the environment provides to account contracts. A context
/// represents a single instance of authorization in the auth call tree.
#[contracttype(crate_path = "crate", export = false)]
#[derive(Clone)]
pub struct Context {
    pub contract: BytesN<32>,
    pub fn_name: Symbol,
    pub args: Vec<RawVal>,
}

impl Context {
    pub fn contract(&self) -> Address {
        Address::from_contract_id(&self.contract)
    }
}

/// Custom account interface that a contract implements to support being used
/// as a custom account for auth.
///
/// Once a contract implements the interface, the address of the contract can be
/// used for addresses that have [`Address::require_auth`] called.
pub trait CustomAccountInterface {
    type Signature;
    type Error: Into<Status>;

    /// Check that the signatures and auth contexts are valid.
    fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signatures: Vec<Self::Signature>,
        auth_contexts: Vec<Context>,
    ) -> Result<(), Self::Error>;
}
