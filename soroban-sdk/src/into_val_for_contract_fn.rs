//! IntoValForContractFn is an internal trait that is used by code generated
//! for the export of contract functions. The generated code calls the trait to
//! convert return values from their respective SDK types into Val.
//!
//! The trait has a blanket implementation for all types that already implement
//! IntoVal<Env, Val>.
//!
//! This trait exists to call `IncludeSpec::__include_spec_marker()` to ensure that type
//! specs are included in the WASM when types are used at external boundaries
//! (function return values).

use crate::{Env, IncludeSpec, IntoVal, Val};

#[doc(hidden)]
#[deprecated(
    note = "IntoValForContractFn is an internal trait and is not safe to use or implement"
)]
pub trait IntoValForContractFn {
    fn into_val_for_contract_fn(self, env: &Env) -> Val;
}

#[doc(hidden)]
#[allow(deprecated)]
impl<T> IntoValForContractFn for T
where
    T: IntoVal<Env, Val> + IncludeSpec,
{
    fn into_val_for_contract_fn(self, env: &Env) -> Val {
        T::__include_spec_marker();
        self.into_val(env)
    }
}
