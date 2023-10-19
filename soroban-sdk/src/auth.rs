use crate::{contracttype, Address, BytesN, Env, Error, Symbol, Val, Vec};

/// Context of a single authorized call performed by an address.
///
/// Custom account contracts that implement `__check_auth` special function
/// receive a list of `Context` values corresponding to all the calls that
/// need to be authorized.
#[derive(Clone)]
#[contracttype(crate_path = "crate", export = false)]
pub enum Context {
    Contract(ContractContext),
    CreateContractHostFn(CreateContractHostFnContext),
}

/// Authorization context of a single contract call.
///
/// This struct corresponds to a `require_auth_for_args` call for an address
/// from `contract` function with `fn_name` name and `args` arguments.
#[derive(Clone)]
#[contracttype(crate_path = "crate", export = false)]
pub struct ContractContext {
    pub contract: Address,
    pub fn_name: Symbol,
    pub args: Vec<Val>,
}

/// Authorization context for `create_contract` host function that creates a
/// new contract on behalf of authorizer address.
#[derive(Clone)]
#[contracttype(crate_path = "crate", export = false)]
pub struct CreateContractHostFnContext {
    pub executable: ContractExecutable,
    pub salt: BytesN<32>,
}

/// Contract executable used for creating a new contract and used in
/// `CreateContractHostFnContext`.
#[derive(Clone)]
#[contracttype(crate_path = "crate", export = false)]
pub enum ContractExecutable {
    Wasm(BytesN<32>),
}

/// A node in the tree of authorizations performed on behalf of the current
/// contract as invoker of the contracts deeper in the call stack.
///
/// This is used as an argument of `authorize_as_current_contract` host function.
///
/// This tree corresponds `require_auth[_for_args]` calls on behalf of the
/// current contract.
#[derive(Clone)]
#[contracttype(crate_path = "crate", export = false)]
pub enum InvokerContractAuthEntry {
    Contract(SubContractInvocation),
    CreateContractHostFn(CreateContractHostFnContext),
}

/// Value of contract node in InvokerContractAuthEntry tree.
#[derive(Clone)]
#[contracttype(crate_path = "crate", export = false)]
pub struct SubContractInvocation {
    pub context: ContractContext,
    pub sub_invocations: Vec<InvokerContractAuthEntry>,
}

/// Custom account interface that a contract implements to support being used
/// as a custom account for auth.
///
/// Once a contract implements the interface, call to [`Address::require_auth`]
/// for the contract's address will call its `__check_auth` implementation.
pub trait CustomAccountInterface {
    type Signature;
    type Error: Into<Error>;

    /// Check that the signatures and auth contexts are valid.
    fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signatures: Self::Signature,
        auth_contexts: Vec<Context>,
    ) -> Result<(), Self::Error>;
}
