//! Storage contains types for storing data for the currently executing contract.
use core::fmt::Debug;

use crate::{
    env::internal::{self, RawVal},
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Env, IntoVal, TryFromVal,
};

/// Storage stores and retrieves data for the currently executing contract.
///
/// All data stored can only be queried and modified by the contract that stores
/// it. Contracts cannot query or modify data stored by other contracts.
///
/// Storage has persistent and temporary modes.
///
/// For persistent mode data is stored in the ledger and is viewable outside of
/// contracts wherever the ledger is accessible. This is the most universally
/// useful storage mode.
///
/// For temporary mode data exists only during the execution time of the
/// top-level contract. It is thus only useful to store data between several
/// cross-contract calls (e.g. to increase and then spend the token allowance
/// from another contract).
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, Symbol};
///
/// # use soroban_sdk::{contractimpl, BytesN};
/// #
/// # pub struct Contract;
/// #
/// # #[contractimpl]
/// # impl Contract {
/// #     pub fn f(env: Env) {
/// let storage = env.storage();
/// let key = Symbol::short("key");
/// env.storage().set(&key, &1);
/// assert_eq!(storage.has(&key), true);
/// assert_eq!(storage.get::<_, i32>(&key), Some(1));
/// #     }
/// # }
/// #
/// # #[cfg(feature = "testutils")]
/// # fn main() {
/// #     let env = Env::default();
/// #     let contract_id = env.register_contract(None, Contract);
/// #     ContractClient::new(&env, &contract_id).f();
/// # }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
#[derive(Clone)]
pub struct Storage {
    env: Env,
}

impl Debug for Storage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Storage")
    }
}

impl Storage {
    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Storage {
        Storage { env: env.clone() }
    }

    /// Returns if there is a value stored for the given key in the currently
    /// executing contracts storage.
    #[inline(always)]
    pub fn has<K>(&self, key: &K) -> bool
    where
        K: IntoVal<Env, RawVal>,
    {
        self.has_internal(key.into_val(&self.env))
    }

    /// Returns the value stored for the given key in the currently executing
    /// contract's storage, when present.
    ///
    /// Returns `None` when the value is missing.
    ///
    /// If the value is present, then the returned value will be a result of
    /// converting the internal value representation to `V`, or will panic if
    /// the conversion to `V` fails.
    #[inline(always)]
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: IntoVal<Env, RawVal>,
        V: TryFromVal<Env, RawVal>,
    {
        let key = key.into_val(&self.env);
        if self.has_internal(key) {
            let rv = self.get_internal(key);
            Some(V::try_from_val(&self.env, &rv).unwrap_optimized())
        } else {
            None
        }
    }

    /// Sets the value for the given key in the currently executing contract's
    /// storage.
    ///
    /// If the key already has a value associated with it, the old value is
    /// replaced by the new value.
    #[inline(always)]
    pub fn set<K, V>(&self, key: &K, val: &V)
    where
        K: IntoVal<Env, RawVal>,
        V: IntoVal<Env, RawVal>,
    {
        let env = &self.env;
        internal::Env::put_contract_data(env, key.into_val(env), val.into_val(env))
            .unwrap_infallible();
    }

    /// Removes the key and the corresponding value from the currently executing
    /// contract's storage.
    ///
    /// No-op if the key does not exist.
    #[inline(always)]
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, RawVal>,
    {
        let env = &self.env;
        internal::Env::del_contract_data(env, key.into_val(env)).unwrap_infallible();
    }

    fn has_internal(&self, key: RawVal) -> bool {
        internal::Env::has_contract_data(&self.env, key)
            .unwrap_infallible()
            .into()
    }

    fn get_internal(&self, key: RawVal) -> RawVal {
        internal::Env::get_contract_data(&self.env, key).unwrap_infallible()
    }
}
