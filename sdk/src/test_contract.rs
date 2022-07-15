#![cfg(feature = "testutils")]

use crate::env::{
    internal::{ContractFunctionSet, EnvImpl},
    Env, RawVal, Symbol,
};
use std::collections::HashMap;

pub struct TestContract(HashMap<Symbol, &'static dyn Fn(Env, &[RawVal]) -> RawVal>);

impl TestContract {
    pub fn new() -> TestContract {
        TestContract(HashMap::new())
    }

    pub fn add_function(&mut self, name: &str, f: &'static dyn Fn(Env, &[RawVal]) -> RawVal) {
        self.0.insert(Symbol::from_str(name), f);
    }
}

impl ContractFunctionSet for TestContract {
    fn call(&self, func: &Symbol, env_impl: &EnvImpl, args: &[RawVal]) -> Option<RawVal> {
        let f = self.0.get(func)?;
        let env = Env::with_impl(env_impl.clone());
        Some(f(env, args))
    }
}
