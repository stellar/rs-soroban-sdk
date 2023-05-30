#![cfg(any(test, feature = "testutils"))]

use crate::{contractimpl, xdr, Address, RawVal, Vec};

#[doc(hidden)]
pub struct MockAuthContract;

#[contractimpl(crate_path = "crate")]
impl MockAuthContract {
    #[allow(non_snake_case)]
    pub fn __check_auth(_signature_payload: RawVal, _signatures: RawVal, _auth_context: RawVal) {}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MockAuth<'a> {
    pub address: &'a Address,
    pub nonce: u64,
    pub invoke: &'a MockAuthInvoke<'a>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MockAuthInvoke<'a> {
    pub contract: &'a Address,
    pub fn_name: &'a str,
    pub args: Vec<RawVal>,
    pub sub_invokes: &'a [MockAuthInvoke<'a>],
}

impl<'a> From<&MockAuth<'a>> for xdr::ContractAuth {
    fn from(value: &MockAuth) -> Self {
        Self {
            address_with_nonce: Some(xdr::AddressWithNonce {
                address: value.address.try_into().unwrap(),
                nonce: value.nonce,
            }),
            root_invocation: value.invoke.into(),
            signature_args: xdr::ScVec::default(),
        }
    }
}

impl<'a> From<MockAuth<'a>> for xdr::ContractAuth {
    fn from(value: MockAuth<'a>) -> Self {
        (&value).into()
    }
}

impl<'a> From<&MockAuthInvoke<'a>> for xdr::AuthorizedInvocation {
    fn from(value: &MockAuthInvoke<'a>) -> Self {
        Self {
            contract_id: xdr::Hash(value.contract.contract_id().to_array()),
            function_name: value.fn_name.try_into().unwrap(),
            args: value.args.clone().try_into().unwrap(),
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

impl<'a> From<MockAuthInvoke<'a>> for xdr::AuthorizedInvocation {
    fn from(value: MockAuthInvoke<'a>) -> Self {
        (&value).into()
    }
}
