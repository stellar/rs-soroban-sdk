//! IntoValForContractFn is an internal trait that is used by code generated
//! for the export of contract functions. The generated code calls the trait to
//! convert return values from their respective SDK types into Val.
//!
//! The trait has a blanket implementation for all types that already implement
//! IntoVal<Env, Val>.
//!
//! When the `experimental_spec_shaking_v2` feature is enabled, this trait also
//! roots the `SpecShakingMarker::MARKER_NODE` static graph via a volatile read
//! so that the return type's spec (and any transitively-referenced types'
//! specs) are kept in the WASM through linker dead-code elimination.

use crate::{Env, IntoVal, Val};

#[doc(hidden)]
#[deprecated(
    note = "IntoValForContractFn is an internal trait and is not safe to use or implement"
)]
pub trait IntoValForContractFn {
    fn into_val_for_contract_fn(self, env: &Env) -> Val;
}

#[cfg(spec_shaking_v2)]
#[doc(hidden)]
#[allow(deprecated)]
impl<T> IntoValForContractFn for T
where
    T: IntoVal<Env, Val> + crate::SpecShakingMarker,
{
    fn into_val_for_contract_fn(self, env: &Env) -> Val {
        #[cfg(target_family = "wasm")]
        {
            // Volatile-read the root MARKER_NODE pointer to root the spec-
            // shaking reachability graph. See soroban-sdk/src/spec_shaking.rs.
            let _ = unsafe { core::ptr::read_volatile(&T::MARKER_NODE) };
        }
        self.into_val(env)
    }
}

#[cfg(not(spec_shaking_v2))]
#[doc(hidden)]
#[allow(deprecated)]
impl<T> IntoValForContractFn for T
where
    T: IntoVal<Env, Val>,
{
    fn into_val_for_contract_fn(self, env: &Env) -> Val {
        self.into_val(env)
    }
}
