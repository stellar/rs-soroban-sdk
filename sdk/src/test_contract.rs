#![cfg(feature = "testutils")]

use crate::env::{internal, Env, RawVal, Symbol};

#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
pub trait ContractFunctionSet {
    fn call(&self, func: &Symbol, env: Env, args: &[RawVal]) -> Option<RawVal>;
}

pub(crate) struct InternalContractFunctionSet<T: ContractFunctionSet>(pub(crate) T);

impl<T: ContractFunctionSet> internal::ContractFunctionSet for InternalContractFunctionSet<T> {
    fn call(&self, func: &Symbol, env_impl: &internal::EnvImpl, args: &[RawVal]) -> Option<RawVal> {
        self.0.call(func, Env::with_impl(env_impl.clone()), args)
    }
}
