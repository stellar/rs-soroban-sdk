//! Storage contains types for storing data for the currently executing contract.
use core::fmt::Debug;

use crate::{
    env::internal::{self, RawVal, StorageType},
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Env, IntoVal, TryFromVal,
};

/// Storage stores and retrieves data for the currently executing contract.
///
/// All data stored can only be queried and modified by the contract that stores
/// it. Contracts cannot query or modify data stored by other contracts.
///
/// There are three types of storage - Exclusive, Mergeable, and Temporary.
///
/// Temporary entries are the cheapest storage option and are never in the Expired State Stack (ESS). Whenever
/// a TemporaryEntry expires, the entry is permanently deleted and cannot be recovered.
/// This storage type is best for entries that are only relevant for short periods of
/// time or for entries that can be arbitrarily recreated.
///
/// Recreateable entries are in between temporary and exclusive entries when it comes to fees.
/// Whenever a recreateable entry expires, it is deleted from the ledger, but sent to an
/// ESS. The entry can then be recovered later through an operation in Stellar Core issued for the
/// expired entry.
///
/// Exclusive entries are the most expensive storage type. Like mergeable entries, whenever
/// a exclusive entry expires, it is sent to the ESS and can be recovered via an operation in Stellar Core.
/// Unlike recreateable entries, only a single version of a exclusive entry can exist at a time.
/// This extra security guarantee adds cost to the exclusive entry making it the most expensive.
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
/// storage.mergeable().set(&key, &1, None);
/// assert_eq!(storage.mergeable().has(&key), true);
/// assert_eq!(storage.mergeable().get::<_, i32>(&key), Some(1));
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

    pub fn exclusive(&self) -> Exclusive {
        Exclusive {
            storage: self.clone(),
        }
    }

    pub fn mergeable(&self) -> Mergeable {
        Mergeable {
            storage: self.clone(),
        }
    }

    pub fn temporary(&self) -> Temporary {
        Temporary {
            storage: self.clone(),
        }
    }

    /// Returns if there is a value stored for the given key in the currently
    /// executing contracts storage.
    #[inline(always)]
    pub(crate) fn has<K>(&self, key: &K, storage_type: StorageType) -> bool
    where
        K: IntoVal<Env, RawVal>,
    {
        self.has_internal(key.into_val(&self.env), storage_type)
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
    pub fn get<K, V>(&self, key: &K, storage_type: StorageType) -> Option<V>
    where
        K: IntoVal<Env, RawVal>,
        V: TryFromVal<Env, RawVal>,
    {
        let key = key.into_val(&self.env);
        if self.has_internal(key, storage_type) {
            let rv = self.get_internal(key, storage_type);
            Some(V::try_from_val(&self.env, &rv).unwrap_optimized())
        } else {
            None
        }
    }

    /// Returns the value there is a value stored for the given key in the
    /// currently executing contract's storage.
    ///
    /// The returned value is a result of converting the internal value
    pub(crate) fn set<K, V>(&self, key: &K, val: &V, storage_type: StorageType, flags: Option<u32>)
    where
        K: IntoVal<Env, RawVal>,
        V: IntoVal<Env, RawVal>,
    {
        let f: RawVal = match flags {
            None => ().into(),
            Some(i) => i.into(),
        };
        let env = &self.env;
        internal::Env::put_contract_data(
            env,
            key.into_val(env),
            val.into_val(env),
            storage_type,
            f,
        )
        .unwrap_infallible();
    }

    pub(crate) fn bump<K>(&self, key: &K, storage_type: StorageType, min_ledgers_to_live: u32)
    where
        K: IntoVal<Env, RawVal>,
    {
        let env = &self.env;
        internal::Env::bump_contract_data(
            env,
            key.into_val(env),
            storage_type,
            min_ledgers_to_live.into(),
        )
        .unwrap_infallible();
    }

    /// Removes the key and the corresponding value from the currently executing
    /// contract's storage.
    ///
    /// No-op if the key does not exist.
    #[inline(always)]
    pub(crate) fn remove<K>(&self, key: &K, storage_type: StorageType)
    where
        K: IntoVal<Env, RawVal>,
    {
        let env = &self.env;
        internal::Env::del_contract_data(env, key.into_val(env), storage_type).unwrap_infallible();
    }

    fn has_internal(&self, key: RawVal, storage_type: StorageType) -> bool {
        internal::Env::has_contract_data(&self.env, key, storage_type)
            .unwrap_infallible()
            .into()
    }

    fn get_internal(&self, key: RawVal, storage_type: StorageType) -> RawVal {
        internal::Env::get_contract_data(&self.env, key, storage_type).unwrap_infallible()
    }
}

pub struct Exclusive {
    storage: Storage,
}

impl Exclusive {
    pub fn has<K>(&self, key: &K) -> bool
    where
        K: IntoVal<Env, RawVal>,
    {
        self.storage.has(key, StorageType::EXCLUSIVE)
    }

    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        V::Error: Debug,
        K: IntoVal<Env, RawVal>,
        V: TryFromVal<Env, RawVal>,
    {
        self.storage.get(key, StorageType::EXCLUSIVE)
    }

    pub fn set<K, V>(&self, key: &K, val: &V, flags: Option<u32>)
    where
        K: IntoVal<Env, RawVal>,
        V: IntoVal<Env, RawVal>,
    {
        self.storage.set(key, val, StorageType::EXCLUSIVE, flags)
    }

    pub fn bump<K>(&self, key: &K, min_ledgers_to_live: u32)
    where
        K: IntoVal<Env, RawVal>,
    {
        self.storage
            .bump(key, StorageType::EXCLUSIVE, min_ledgers_to_live)
    }

    #[inline(always)]
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, RawVal>,
    {
        self.storage.remove(key, StorageType::EXCLUSIVE)
    }
}

pub struct Mergeable {
    storage: Storage,
}

impl Mergeable {
    pub fn has<K>(&self, key: &K) -> bool
    where
        K: IntoVal<Env, RawVal>,
    {
        self.storage.has(key, StorageType::MERGEABLE)
    }

    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        V::Error: Debug,
        K: IntoVal<Env, RawVal>,
        V: TryFromVal<Env, RawVal>,
    {
        self.storage.get(key, StorageType::MERGEABLE)
    }

    pub fn set<K, V>(&self, key: &K, val: &V, flags: Option<u32>)
    where
        K: IntoVal<Env, RawVal>,
        V: IntoVal<Env, RawVal>,
    {
        self.storage.set(key, val, StorageType::MERGEABLE, flags)
    }

    pub fn bump<K>(&self, key: &K, min_ledgers_to_live: u32)
    where
        K: IntoVal<Env, RawVal>,
    {
        self.storage
            .bump(key, StorageType::MERGEABLE, min_ledgers_to_live)
    }

    #[inline(always)]
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, RawVal>,
    {
        self.storage.remove(key, StorageType::MERGEABLE)
    }
}

pub struct Temporary {
    storage: Storage,
}

impl Temporary {
    pub fn has<K>(&self, key: &K) -> bool
    where
        K: IntoVal<Env, RawVal>,
    {
        self.storage.has(key, StorageType::TEMPORARY)
    }

    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        V::Error: Debug,
        K: IntoVal<Env, RawVal>,
        V: TryFromVal<Env, RawVal>,
    {
        self.storage.get(key, StorageType::TEMPORARY)
    }

    pub fn set<K, V>(&self, key: &K, val: &V, flags: Option<u32>)
    where
        K: IntoVal<Env, RawVal>,
        V: IntoVal<Env, RawVal>,
    {
        self.storage.set(key, val, StorageType::TEMPORARY, flags)
    }

    pub fn bump<K>(&self, key: &K, min_ledgers_to_live: u32)
    where
        K: IntoVal<Env, RawVal>,
    {
        self.storage
            .bump(key, StorageType::TEMPORARY, min_ledgers_to_live)
    }

    #[inline(always)]
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, RawVal>,
    {
        self.storage.remove(key, StorageType::TEMPORARY)
    }
}
