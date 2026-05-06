//! IntoValForContractFn is an internal trait that is used by code generated
//! for the export of contract functions. The generated code calls the trait to
//! convert return values from their respective SDK types into Val.
//!
//! The trait has a blanket implementation for all types that already implement
//! IntoVal<Env, Val>.
//!
//! Spec shaking roots function return types from the `contractspecv0` function
//! entries, so this conversion path does not need to emit marker code.

use crate::{Env, IntoVal, Val};

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
    T: IntoVal<Env, Val>,
{
    fn into_val_for_contract_fn(self, env: &Env) -> Val {
        self.into_val(env)
    }
}
