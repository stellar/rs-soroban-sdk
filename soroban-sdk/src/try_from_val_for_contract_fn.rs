//! TryFromValForContractFn is an internal trait that is used by code generated
//! for the export of contract functions. The generated code calls the trait to
//! convert incoming Val's into their respective SDK types.
//!
//! The trait has a blanket implementation for all types that already implement
//! TryFromVal<_, Val>.
//!
//! A companion trait, TryFromValForCheckAuthPayload, converts the first
//! argument of a `__check_auth` function, and allows a special type,
//! [`crate::crypto::Hash`], to be used there without otherwise being creatable
//! from a Val via the public TryFromVal trait, and therefore not storeable, nor
//! usable as any other contract function argument.

use crate::{env::internal::Env, Error, TryFromVal};
use core::fmt::Debug;

#[deprecated(
    note = "TryFromValForContractFn is an internal trait and is not safe to use or implement"
)]
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used as a contract function argument",
    label = "not a contract function argument type",
    note = "contract function arguments must be convertible from a `Val`, such as types marked `#[contracttype]`",
    note = "`Hash<N>` can only be used in contexts where there is a guarantee that the hash has been sourced from a secure cryptographic hash function, such as the signature payload of `__check_auth`"
)]
pub trait TryFromValForContractFn<E: Env, V: ?Sized>: Sized {
    type Error: Debug + Into<Error>;
    fn try_from_val_for_contract_fn(env: &E, v: &V) -> Result<Self, Self::Error>;
}

#[doc(hidden)]
#[allow(deprecated)]
#[diagnostic::do_not_recommend]
impl<E: Env, T, U> TryFromValForContractFn<E, T> for U
where
    U: TryFromVal<E, T>,
{
    type Error = U::Error;
    fn try_from_val_for_contract_fn(e: &E, v: &T) -> Result<Self, Self::Error> {
        U::try_from_val(e, v)
    }
}

/// TryFromValForCheckAuthPayload is an internal trait that is used by code
/// generated for the export of a `__check_auth` function, to convert its first
/// argument, the signature payload.
///
/// The signature payload is the one argument that may be a
/// [`crate::crypto::Hash`], because the host guarantees it is the hash of the
/// payload being authorized. Every type usable as any other contract function
/// argument is usable as the signature payload too.
#[doc(hidden)]
#[deprecated(
    note = "TryFromValForCheckAuthPayload is an internal trait and is not safe to use or implement"
)]
pub trait TryFromValForCheckAuthPayload<E: Env, V: ?Sized>: Sized {
    type Error: Debug + Into<Error>;
    fn try_from_val_for_check_auth_payload(env: &E, v: &V) -> Result<Self, Self::Error>;
}

#[doc(hidden)]
#[allow(deprecated)]
impl<E: Env, T, U> TryFromValForCheckAuthPayload<E, T> for U
where
    U: TryFromVal<E, T>,
{
    type Error = U::Error;
    fn try_from_val_for_check_auth_payload(e: &E, v: &T) -> Result<Self, Self::Error> {
        U::try_from_val(e, v)
    }
}
