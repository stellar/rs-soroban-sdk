#![cfg(any(test, feature = "testutils"))]

use crate::{contract, contractimpl, xdr, Address, Env, Symbol, TryFromVal, Val, Vec};

use super::Ledger;

#[doc(hidden)]
#[contract(crate_path = "crate")]
pub struct MockAuthContract;

#[contractimpl(crate_path = "crate")]
impl MockAuthContract {
    #[allow(non_snake_case)]
    pub fn __check_auth(_signature_payload: Val, _signatures: Val, _auth_context: Val) {}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MockAuth<'a> {
    pub address: &'a Address,
    pub invoke: &'a MockAuthInvoke<'a>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MockAuthInvoke<'a> {
    pub contract: &'a Address,
    pub fn_name: &'a str,
    pub args: Vec<Val>,
    pub sub_invokes: &'a [MockAuthInvoke<'a>],
}

impl<'a> From<&MockAuth<'a>> for xdr::SorobanAuthorizationEntry {
    fn from(value: &MockAuth) -> Self {
        let env = value.address.env();
        let curr_ledger = env.ledger().sequence();
        let max_entry_ttl = env.ledger().get().max_entry_ttl;
        Self {
            root_invocation: value.invoke.into(),
            credentials: xdr::SorobanCredentials::Address(xdr::SorobanAddressCredentials {
                address: value.address.try_into().unwrap(),
                nonce: env.with_generator(|mut g| g.nonce()),
                signature_expiration_ledger: curr_ledger + max_entry_ttl - 1,
                signature: xdr::ScVal::Void,
            }),
        }
    }
}

impl<'a> From<MockAuth<'a>> for xdr::SorobanAuthorizationEntry {
    fn from(value: MockAuth<'a>) -> Self {
        (&value).into()
    }
}

impl<'a> From<&MockAuthInvoke<'a>> for xdr::SorobanAuthorizedInvocation {
    fn from(value: &MockAuthInvoke<'a>) -> Self {
        Self {
            function: xdr::SorobanAuthorizedFunction::ContractFn(xdr::InvokeContractArgs {
                contract_address: xdr::ScAddress::Contract(value.contract.contract_id()),
                function_name: value.fn_name.try_into().unwrap(),
                args: value.args.clone().try_into().unwrap(),
            }),
            sub_invocations: value
                .sub_invokes
                .iter()
                .map(Into::<_>::into)
                .collect::<std::vec::Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<'a> From<MockAuthInvoke<'a>> for xdr::SorobanAuthorizedInvocation {
    fn from(value: MockAuthInvoke<'a>) -> Self {
        (&value).into()
    }
}

/// Describes an authorized invocation tree from the perspective of a single
/// address.
///
/// The authorized invocation tree for a given address is different from a
/// regular call tree: it only has nodes for the contract calls that called
/// `require_auth[_for_args]` for that address.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AuthorizedInvocation {
    /// Function that has called `require_auth`.
    pub function: AuthorizedFunction,
    /// Authorized invocations originating from `function`.
    pub sub_invocations: std::vec::Vec<AuthorizedInvocation>,
}

/// A single node in `AuthorizedInvocation` tree.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AuthorizedFunction {
    /// Contract function defined by the contract address, function name and
    /// `require_auth[_for_args]` arguments (these don't necessarily need to
    /// match the actual invocation arguments).
    Contract((Address, Symbol, Vec<Val>)),
    /// Create contract host function with arguments specified as the respective
    /// XDR.
    CreateContractHostFn(xdr::CreateContractArgs),
}

impl AuthorizedFunction {
    pub fn from_xdr(env: &Env, v: &xdr::SorobanAuthorizedFunction) -> Self {
        match v {
            xdr::SorobanAuthorizedFunction::ContractFn(contract_fn) => {
                let mut args = Vec::new(env);
                for v in contract_fn.args.iter() {
                    args.push_back(Val::try_from_val(env, v).unwrap());
                }
                Self::Contract((
                    Address::try_from_val(
                        env,
                        &xdr::ScVal::Address(contract_fn.contract_address.clone()),
                    )
                    .unwrap(),
                    Symbol::try_from_val(env, &contract_fn.function_name).unwrap(),
                    args,
                ))
            }
            xdr::SorobanAuthorizedFunction::CreateContractHostFn(create_contract) => {
                Self::CreateContractHostFn(create_contract.clone())
            }
        }
    }
}

impl AuthorizedInvocation {
    pub fn from_xdr(env: &Env, v: &xdr::SorobanAuthorizedInvocation) -> Self {
        Self {
            function: AuthorizedFunction::from_xdr(env, &v.function),
            sub_invocations: v
                .sub_invocations
                .iter()
                .map(|si| AuthorizedInvocation::from_xdr(env, si))
                .collect(),
        }
    }
}
