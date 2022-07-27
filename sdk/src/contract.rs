use core::fmt::Debug;

use crate::{
    env::internal::{self},
    Binary, ContractData, Env, FixedBinary, TryFromVal,
};

#[derive(Clone)]
pub struct Contract(Env);

impl Debug for Contract {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Contract")
        // TODO: Include contract ID.
    }
}

impl Contract {
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Contract {
        Contract(env.clone())
    }

    #[inline(always)]
    pub fn id(&self) -> FixedBinary<32> {
        let env = self.env();
        internal::Env::get_current_contract(env)
            .in_env(env)
            .try_into()
            .unwrap()
    }

    #[inline(always)]
    pub fn invoking_contract_id(&self) -> FixedBinary<32> {
        let env = self.env();
    }

    #[inline(always)]
    pub fn a(&self) -> ContractData {
        ContractData::new(self.env())
    }
}
