#![cfg(feature = "testutils")]

use crate::env::{
    internal::{ContractFunctionSet, EnvImpl},
    Env, Object, RawVal, Symbol,
};
use std::collections::HashMap;
use std::rc::Rc;

pub struct TestContract(HashMap<Symbol, &'static dyn Fn(Env, &[RawVal]) -> RawVal>);

impl TestContract {
    pub fn new() -> TestContract {
        TestContract(HashMap::new())
    }

    pub fn add_function(&mut self, name: &str, f: &'static dyn Fn(Env, &[RawVal]) -> RawVal) {
        self.0.insert(Symbol::from_str(name), f);
    }

    pub fn register(self, e: &Env, contract_id: RawVal) {
        let id_obj: Object = RawVal::from(contract_id).try_into().unwrap();
        e.env_impl
            .register_test_contract(id_obj, Rc::new(self))
            .unwrap();
    }
}

impl ContractFunctionSet for TestContract {
    fn call(&self, func: &Symbol, env_impl: &EnvImpl, args: &[RawVal]) -> Option<RawVal> {
        let f = self.0.get(func)?;
        Some(f(
            Env {
                env_impl: env_impl.clone(),
            },
            args,
        ))
    }
}
