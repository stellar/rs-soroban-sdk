//! Storage mapper contains types for storing specific predetermined types of
//! data for the currently executing contract.
use core::{fmt::Debug, marker::PhantomData};

use crate::{
    env::internal::{self, RawVal},
    Env, IntoVal, TryFromVal,
};

/// Storage mapper stores and retrieves specific key and value types for the
/// currently executing contract.
///
/// The storage mapper is a layer ontop of [`Storage`][crate::storage::Storage],
/// and the same behaviors and access rules apply.
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
/// let mapping: StorageMap<Symbol, u32> = env.storage_map();
/// let key = symbol!("key");
/// mapping.set(key, 1);
/// assert_eq!(mapping.has(key), true);
/// assert_eq!(mapping.get(key), Some(Ok(1)));
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
pub struct StorageMap<K, V>(Env, PhantomData<K>, PhantomData<V>)
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    V: TryFromVal<Env, RawVal>;

impl<K, V> Debug for StorageMap<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    V: TryFromVal<Env, RawVal>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StorageMap")
    }
}

impl<K, V> StorageMap<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    V: TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Self {
        Self(env.clone(), PhantomData, PhantomData)
    }

    /// Returns if there is a value stored for the given key in the currently
    /// executing contracts data.
    #[inline(always)]
    pub fn has(&self, key: K) -> bool {
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
    pub fn get(&self, key: K) -> Option<Result<V, V::Error>>
    where
        V::Error: Debug,
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
    pub fn get_unchecked(&self, key: K) -> Result<V, V::Error>
    where
        V::Error: Debug,
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
    pub fn set(&self, key: K, val: V) {
        let env = self.env();
        internal::Env::put_contract_data(env, key.into_val(env), val.into_val(env));
    }

    #[inline(always)]
    pub fn remove(&self, key: K) {
        let env = self.env();
        internal::Env::del_contract_data(env, key.into_val(env));
    }
}
