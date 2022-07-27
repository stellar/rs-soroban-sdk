use core::fmt::Debug;

use crate::{
    env::internal::{self, RawVal},
    Env, IntoVal, TryFromVal,
};

#[derive(Clone)]
pub struct ContractData(Env);

impl Debug for ContractData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ContractData")
    }
}

impl ContractData {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> ContractData {
        ContractData(env.clone())
    }

    // TODO: Use Borrow<K> for all key use in these functions.

    #[inline(always)]
    pub fn has<K>(&self, key: K) -> bool
    where
        K: IntoVal<Env, RawVal>,
    {
        let env = self.env();
        let rv = internal::Env::has_contract_data(env, key.into_val(env));
        rv.try_into().unwrap()
    }

    #[inline(always)]
    pub fn get<K, V>(&self, key: K) -> V
    where
        V::Error: Debug,
        K: IntoVal<Env, RawVal>,
        V: TryFromVal<Env, RawVal>,
    {
        let env = self.env();
        let rv = internal::Env::get_contract_data(env, key.into_val(env));
        // TODO: Return Result.
        V::try_from_val(env, rv).unwrap()
    }

    #[inline(always)]
    pub fn set<K, V>(&self, key: K, val: V)
    where
        K: IntoVal<Env, RawVal>,
        V: IntoVal<Env, RawVal>,
    {
        let env = self.env();
        internal::Env::put_contract_data(env, key.into_val(env), val.into_val(env));
    }

    #[inline(always)]
    pub fn remove<K>(&self, key: K)
    where
        K: IntoVal<Env, RawVal>,
    {
        let env = self.env();
        internal::Env::del_contract_data(env, key.into_val(env));
    }
}
