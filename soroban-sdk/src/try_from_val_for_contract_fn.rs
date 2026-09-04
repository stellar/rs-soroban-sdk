//! TryFromValForContractFn is an internal trait that is used by code generated
//! for the export of contract functions. The generated code calls the trait to
//! convert incoming Val's into their respective SDK types.
//!
//! The trait has a blanket implementation for all types that already implement
//! TryFromVal<_, Val>.
//!
//! The trait exists primarily to allow some special types, e.g.
//! [`crate::crypto::Hash`], to be used as inputs to contract functions without
//! otherwise being creatable from a Val via the public TryFromVal trait, and
//! therefore not storeable.
//!
//! For types that can be used and converted everywhere, implementing TryFromVal
//! is most appropriate. For types that should only be used and converted to as
//! part of contract function invocation, then this trait is appropriate.

use crate::{env::internal::Env, Error, TryFromVal};
use core::fmt::Debug;

#[doc(hidden)]
#[deprecated(
    note = "TryFromValForContractFn is an internal trait and is not safe to use or implement"
)]
pub trait TryFromValForContractFn<E: Env, V: ?Sized>: Sized {
    type Error: Debug + Into<Error>;
    fn try_from_val_for_contract_fn(env: &E, v: &V) -> Result<Self, Self::Error>;
}

#[doc(hidden)]
#[allow(deprecated)]
impl<E: Env, T, U> TryFromValForContractFn<E, T> for U
where
    U: TryFromVal<E, T>,
{
    type Error = U::Error;
    fn try_from_val_for_contract_fn(e: &E, v: &T) -> Result<Self, Self::Error> {
        U::try_from_val(e, v)
    }
}
