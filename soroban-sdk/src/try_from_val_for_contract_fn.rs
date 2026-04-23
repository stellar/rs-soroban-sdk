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
//!
//! When the `experimental_spec_shaking_v2` feature is enabled, this trait also
//! roots the `SpecShakingMarker::MARKER_NODE` static graph via a volatile read
//! so that the conversion type's spec (and any transitively-referenced types'
//! specs) are kept in the WASM through linker dead-code elimination.

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

#[cfg(spec_shaking_v2)]
#[doc(hidden)]
#[allow(deprecated)]
impl<E: Env, T, U> TryFromValForContractFn<E, T> for U
where
    U: TryFromVal<E, T> + crate::SpecShakingMarker,
{
    type Error = U::Error;
    fn try_from_val_for_contract_fn(e: &E, v: &T) -> Result<Self, Self::Error> {
        #[cfg(target_family = "wasm")]
        {
            // Volatile-read the root MARKER_NODE pointer to root the spec-
            // shaking reachability graph. See soroban-sdk/src/spec_shaking.rs.
            let _ = unsafe { core::ptr::read_volatile(&U::MARKER_NODE) };
        }
        U::try_from_val(e, v)
    }
}

#[cfg(not(spec_shaking_v2))]
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
