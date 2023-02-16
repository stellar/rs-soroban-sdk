//! Storage mapper contains types for storing specific predetermined types of
//! data for the currently executing contract.
use core::{fmt::Debug, marker::PhantomData};

use crate::{
    env::internal::{self, RawVal},
    unwrap::UnwrapInfallible,
    Env, IntoVal, TryFromVal,
};

/// Storage map stores and retrieves specific key and value types for the
/// currently executing contract.
///
/// Storage map is a layer ontop of [`Storage`][crate::storage::Storage],
/// and the same behaviors and access rules apply.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, Symbol};
///
/// # use soroban_sdk::{contractimpl, symbol, BytesN, StorageMap};
/// #
/// # pub struct Contract;
/// #
/// # #[contractimpl]
/// # impl Contract {
/// #     pub fn f(env: Env) {
/// let mapping: StorageMap<Symbol, Symbol, u32> = env.storage_map(symbol!("key_to_num"));
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
pub struct StorageMap<D, K, V>
where
    D: IntoVal<Env, RawVal>,
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    V: TryFromVal<Env, RawVal>,
{
    env: Env,
    discriminant: RawVal,
    _p: (PhantomData<D>, PhantomData<K>, PhantomData<V>),
}

impl<D, K, V> Debug for StorageMap<D, K, V>
where
    D: IntoVal<Env, RawVal>,
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    V: TryFromVal<Env, RawVal>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StorageMap")
    }
}

impl<D, K, V> StorageMap<D, K, V>
where
    D: IntoVal<Env, RawVal>,
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    V: TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.env
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env, discriminant: D) -> Self {
        Self {
            env: env.clone(),
            discriminant: discriminant.into_val(env),
            _p: (PhantomData, PhantomData, PhantomData),
        }
    }

    /// Returns if there is a value stored for the given key in the currently
    /// executing contracts data.
    #[inline(always)]
    pub fn has(&self, key: K) -> bool {
        let env = self.env();
        let key = (self.discriminant, key.into_val(env)).into_val(env);
        let rv = internal::Env::has_contract_data(env, key).unwrap_infallible();
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
        let key = (self.discriminant, key.into_val(env)).into_val(env);
        let has = internal::Env::has_contract_data(env, key).unwrap_infallible();
        if has.is_true() {
            let rv = internal::Env::get_contract_data(env, key).unwrap_infallible();
            Some(V::try_from_val(env, &rv))
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
        let key = (self.discriminant, key.into_val(env)).into_val(env);
        let rv = internal::Env::get_contract_data(env, key).unwrap_infallible();
        V::try_from_val(env, &rv)
    }

    /// Sets the value for the given key in the currently executing contract's
    /// data.
    ///
    /// If the key already has a value associated with it, the old value is
    /// replaced by the new value.
    #[inline(always)]
    pub fn set(&self, key: K, val: V) {
        let env = self.env();
        let key = (self.discriminant, key.into_val(env)).into_val(env);
        let val = val.into_val(env);
        internal::Env::put_contract_data(env, key, val).unwrap_infallible();
    }

    #[inline(always)]
    pub fn remove(&self, key: K) {
        let env = self.env();
        let key = (self.discriminant, key.into_val(env)).into_val(env);
        internal::Env::del_contract_data(env, key).unwrap_infallible();
    }
}
