//! Storage contains types for storing data for the currently executing contract.
use core::fmt::Debug;

use crate::{
    env::internal::{self, RawVal},
    Env, IntoVal, TryFromVal,
};

/// Storage stores and retrieves data for the currently executing contract.
///
/// All data stored can only be queried and modified by the contract that stores
/// it. Other contracts cannot query or modify data stored by other contracts.
/// Data is stored in the ledger and viewable outside of contracts where-ever
/// the ledger is accessible.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, Symbol};
///
/// # use soroban_sdk::{contractimpl, symbol, BytesN};
/// #
/// # pub struct Contract;
/// #
/// # #[contractimpl]
/// # impl Contract {
/// #     pub fn f(env: Env) {
/// let storage = env.storage();
/// let key = symbol!("key");
/// env.storage().set(key, 1);
/// assert_eq!(storage.has(key), true);
/// assert_eq!(storage.get::<_, i32>(key), Some(Ok(1)));
/// #     }
/// # }
/// #
/// # #[cfg(feature = "testutils")]
/// # fn main() {
/// #     let env = Env::default();
/// #     let contract_id = BytesN::from_array(&env, &[0; 32]);
/// #     env.register_contract(&contract_id, Contract);
/// #     ContractClient::new(&env, &contract_id).f();
/// # }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
#[derive(Clone)]
pub struct Storage(Env);

impl Debug for Storage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Storage")
    }
}

impl Storage {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Storage {
        Storage(env.clone())
    }

    // TODO: Use Borrow<K> for all key use in these functions.

    /// Returns if there is a value stored for the given key in the currently
    /// executing contracts data.
    #[inline(always)]
    pub fn has<K>(&self, key: K) -> bool
    where
        K: IntoVal<Env, RawVal>,
    {
        let env = self.env();
        let rv = internal::Env::has_contract_data(env, key.into_val(env));
        rv.is_true()
    }

    /// Returns the value there is a value stored for the given key in the
    /// currently executing contract's data.
    ///
    /// ### Panics
    ///
    /// When the key does not have a value stored.
    ///
    /// When the value stored cannot be converted into the type expected.
    ///
    /// ### TODO
    ///
    /// Add safe checked versions of these functions.
    #[inline(always)]
    pub fn get<K, V>(&self, key: K) -> Option<Result<V, V::Error>>
    where
        V::Error: Debug,
        K: IntoVal<Env, RawVal>,
        V: TryFromVal<Env, RawVal>,
    {
        let env = self.env();
        let key = key.into_val(env);
        let has = internal::Env::has_contract_data(env, key);
        if has.is_true() {
            let rv = internal::Env::get_contract_data(env, key);
            Some(V::try_from_val(env, rv))
        } else {
            None
        }
    }

    /// Returns the value there is a value stored for the given key in the
    /// currently executing contracts data.
    ///
    /// ### Panics
    ///
    /// When the key does not have a value stored.
    #[inline(always)]
    pub fn get_unchecked<K, V>(&self, key: K) -> Result<V, V::Error>
    where
        V::Error: Debug,
        K: IntoVal<Env, RawVal>,
        V: TryFromVal<Env, RawVal>,
    {
        let env = self.env();
        let rv = internal::Env::get_contract_data(env, key.into_val(env));
        V::try_from_val(env, rv)
    }

    /// Sets the value for the given key in the currently executing contract's
    /// data.
    ///
    /// If the key already has a value associated with it, the old value is
    /// replaced by the new value.
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
