//! Auth contains types for building custom account contracts.

use crate::{contracttype, crypto::Hash, Address, BytesN, Env, Error, Symbol, Val, Vec};

/// Context of a single authorized call performed by an address.
///
/// Custom account contracts that implement `__check_auth` special function
/// receive a list of `Context` values corresponding to all the calls that
/// need to be authorized.
#[derive(Clone)]
#[contracttype(crate_path = "crate", export = false)]
pub enum Context {
    /// Contract invocation.
    Contract(ContractContext),
    /// Contract that has a constructor with no arguments is created.
    CreateContractHostFn(CreateContractHostFnContext),
    /// Contract that has a constructor with 1 or more arguments is created.
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
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

/// Authorization context for `create_contract` host function that creates a
/// new contract on behalf of authorizer address.
/// This is the same as `CreateContractHostFnContext`, but also has
/// contract constructor arguments.
#[derive(Clone)]
#[contracttype(crate_path = "crate", export = false)]
pub struct CreateContractWithConstructorHostFnContext {
    pub executable: ContractExecutable,
    pub salt: BytesN<32>,
    pub constructor_args: Vec<Val>,
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
    /// Invoke a contract.
    Contract(SubContractInvocation),
    /// Create a contract passing 0 arguments to constructor.
    CreateContractHostFn(CreateContractHostFnContext),
    /// Create a contract passing 0 or more arguments to constructor.
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
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
        signature_payload: Hash<32>,
        signatures: Self::Signature,
        auth_contexts: Vec<Context>,
    ) -> Result<(), Self::Error>;
}
