//! IntoValForContractFn is an internal trait that is used by code generated
//! for the export of contract functions. The generated code calls the trait to
//! convert return values from their respective SDK types into Val.
//!
//! The trait has a blanket implementation for all types that already implement
//! IntoVal<Env, Val>.
//!
//! When the `experimental_spec_shaking_v2` feature is enabled, this trait also
//! calls `SpecShakingMarker::spec_shaking_marker()` to ensure that type specs
//! are included in the WASM when types are used at external boundaries
//! (function return values).

use crate::{Env, IntoVal, Val};

#[doc(hidden)]
#[deprecated(
    note = "IntoValForContractFn is an internal trait and is not safe to use or implement"
)]
pub trait IntoValForContractFn {
    fn into_val_for_contract_fn(self, env: &Env) -> Val;
}

#[cfg(feature = "experimental_spec_shaking_v2")]
#[doc(hidden)]
#[allow(deprecated)]
impl<T> IntoValForContractFn for T
where
    T: IntoVal<Env, Val> + crate::SpecShakingMarker,
{
    fn into_val_for_contract_fn(self, env: &Env) -> Val {
        T::spec_shaking_marker();
        self.into_val(env)
    }
}

#[cfg(not(feature = "experimental_spec_shaking_v2"))]
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
