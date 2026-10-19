//! ContractClientReturn is an internal trait that is used by code generated
//! for contract clients. The generated code uses the trait to get the type a
//! client returns for a contract function's return type.
//!
//! The trait has a blanket implementation for all types that implement
//! TryFromVal<_, Val>, which a client returns as themselves.
//!
//! A [`crate::crypto::Hash`] is returned by a client as a [`crate::BytesN`],
//! and an `Option` of one as an `Option` of a `BytesN`,
//! because a client can't guarantee the bytes it receives came from a secure
//! cryptographic hash function. This is consistent with the contract spec,
//! which has a `Hash` as a `BytesN`, and so with clients generated from the
//! spec.

use crate::{crypto::Hash, BytesN, Env, TryFromVal, Val};

#[doc(hidden)]
pub trait ContractClientReturn {
    type Output: TryFromVal<Env, Val>;
}

#[doc(hidden)]
#[diagnostic::do_not_recommend]
impl<T> ContractClientReturn for T
where
    T: TryFromVal<Env, Val>,
{
    type Output = T;
}

#[doc(hidden)]
#[diagnostic::do_not_recommend]
impl<const N: usize> ContractClientReturn for Hash<N> {
    type Output = BytesN<N>;
}

#[doc(hidden)]
#[diagnostic::do_not_recommend]
impl<const N: usize> ContractClientReturn for Option<Hash<N>> {
    type Output = Option<BytesN<N>>;
}
