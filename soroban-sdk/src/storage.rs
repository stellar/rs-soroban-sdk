//! Storage contains types for storing data for the currently executing contract.
use core::fmt::Debug;

use crate::{
    env::internal::{self, StorageType, Val},
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Env, IntoVal, TryFromVal,
};

/// Storage stores and retrieves data for the currently executing contract.
///
/// All data stored can only be queried and modified by the contract that stores
/// it. Contracts cannot query or modify data stored by other contracts.
///
/// There are three types of storage - Temporary, Persistent, and Instance.
///
/// Temporary entries are the cheaper storage option and are never in the Expired State Stack (ESS). Whenever
/// a TemporaryEntry expires, the entry is permanently deleted and cannot be recovered.
/// This storage type is best for entries that are only relevant for short periods of
/// time or for entries that can be arbitrarily recreated.
///
/// Persistent entries are the more expensive storage type. Whenever
/// a persistent entry expires, it is deleted from the ledger, sent to the ESS
/// and can be recovered via an operation in Stellar Core. Only a single version of a
/// persistent entry can exist at a time.
///
/// Instance storage is used to store entries within the Persistent contract
/// instance entry, allowing users to tie that data directly to the expiration
/// on the instance. Instance storage is good for global contract data like
/// metadata, admin accounts, or pool reserves.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, Symbol};
///
/// # use soroban_sdk::{contract, contractimpl, symbol_short, BytesN};
/// #
/// # #[contract]
/// # pub struct Contract;
/// #
/// # #[contractimpl]
/// # impl Contract {
/// #     pub fn f(env: Env) {
/// let storage = env.storage();
/// let key = symbol_short!("key");
/// storage.persistent().set(&key, &1);
/// assert_eq!(storage.persistent().has(&key), true);
/// assert_eq!(storage.persistent().get::<_, i32>(&key), Some(1));
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

    /// Storage for data that can stay in the ledger forever until deleted.
    ///
    /// Persistent entries might expire and be removed from the ledger if they run out
    /// of the rent balance. However, expired entries can be restored and
    /// they cannot be recreated. This means these entries
    /// behave 'as if' they were stored in the ledger forever.
    ///
    /// This should be used for data that requires persistency, such as token
    /// balances, user properties etc.
    pub fn persistent(&self) -> Persistent {
        Persistent {
            storage: self.clone(),
        }
    }

    /// Storage for data that may stay in ledger only for a limited amount of
    /// time.
    ///
    /// Temporary storage is cheaper than Persistent storage.
    ///
    /// Temporary entries will be removed from the ledger after their lifetime
    /// ends. Removed entries can be created again, potentially with different
    /// values.
    ///
    /// This should be used for data that needs to only exist for a limited
    /// period of time, such as oracle data, claimable balances, offer, etc.
    pub fn temporary(&self) -> Temporary {
        Temporary {
            storage: self.clone(),
        }
    }

    /// Storage for a **small amount** of persistent data associated with
    /// the current contract's instance.
    ///
    /// Storing a small amount of frequently used data in instance storage is
    /// likely cheaper than storing it separately in Persistent storage.
    ///
    /// Instance storage is tightly coupled with the contract instance: it will
    /// be loaded from the ledger every time the contract instance itself is
    /// loaded. It also won't appear in the ledger footprint. *All*
    /// the data stored in the instance storage is read from ledger every time
    /// the contract is used and it doesn't matter whether contract uses the
    /// storage or not.
    ///
    /// This has the same lifetime properties as Persistent storage, i.e.
    /// the data semantically stays in the ledger forever and can be
    /// expired/restored.
    ///
    /// The amount of data that can be stored in the instance storage is limited
    /// by the ledger entry size (a network-defined parameter). It is
    /// in the order of 100 KB serialized.
    ///
    /// This should be used for small data directly associated with the current
    /// contract, such as its admin, configuration settings, tokens the contract
    /// operates on etc. Do not use this with any data that can scale in
    /// unbounded fashion (such as user balances).
    pub fn instance(&self) -> Instance {
        Instance {
            storage: self.clone(),
        }
    }

    /// Returns if there is a value stored for the given key in the currently
    /// executing contracts storage.
    #[inline(always)]
    pub(crate) fn has<K>(&self, key: &K, storage_type: StorageType) -> bool
    where
        K: IntoVal<Env, Val>,
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
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        let key = key.into_val(&self.env);
        if self.has_internal(key, storage_type.clone()) {
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
    pub(crate) fn set<K, V>(&self, key: &K, val: &V, storage_type: StorageType)
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
        let env = &self.env;
        internal::Env::put_contract_data(env, key.into_val(env), val.into_val(env), storage_type)
            .unwrap_infallible();
    }

    pub(crate) fn bump<K>(&self, key: &K, storage_type: StorageType, min_ledgers_to_live: u32)
    where
        K: IntoVal<Env, Val>,
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
        K: IntoVal<Env, Val>,
    {
        let env = &self.env;
        internal::Env::del_contract_data(env, key.into_val(env), storage_type).unwrap_infallible();
    }

    fn has_internal(&self, key: Val, storage_type: StorageType) -> bool {
        internal::Env::has_contract_data(&self.env, key, storage_type)
            .unwrap_infallible()
            .into()
    }

    fn get_internal(&self, key: Val, storage_type: StorageType) -> Val {
        internal::Env::get_contract_data(&self.env, key, storage_type).unwrap_infallible()
    }
}

pub struct Persistent {
    storage: Storage,
}

impl Persistent {
    pub fn has<K>(&self, key: &K) -> bool
    where
        K: IntoVal<Env, Val>,
    {
        self.storage.has(key, StorageType::Persistent)
    }

    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        V::Error: Debug,
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        self.storage.get(key, StorageType::Persistent)
    }

    pub fn set<K, V>(&self, key: &K, val: &V)
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
        self.storage.set(key, val, StorageType::Persistent)
    }

    pub fn bump<K>(&self, key: &K, min_ledgers_to_live: u32)
    where
        K: IntoVal<Env, Val>,
    {
        self.storage
            .bump(key, StorageType::Persistent, min_ledgers_to_live)
    }

    #[inline(always)]
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, Val>,
    {
        self.storage.remove(key, StorageType::Persistent)
    }
}

pub struct Temporary {
    storage: Storage,
}

impl Temporary {
    pub fn has<K>(&self, key: &K) -> bool
    where
        K: IntoVal<Env, Val>,
    {
        self.storage.has(key, StorageType::Temporary)
    }

    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        V::Error: Debug,
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        self.storage.get(key, StorageType::Temporary)
    }

    pub fn set<K, V>(&self, key: &K, val: &V)
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
        self.storage.set(key, val, StorageType::Temporary)
    }

    pub fn bump<K>(&self, key: &K, min_ledgers_to_live: u32)
    where
        K: IntoVal<Env, Val>,
    {
        self.storage
            .bump(key, StorageType::Temporary, min_ledgers_to_live)
    }

    #[inline(always)]
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, Val>,
    {
        self.storage.remove(key, StorageType::Temporary)
    }
}

pub struct Instance {
    storage: Storage,
}

impl Instance {
    pub fn has<K>(&self, key: &K) -> bool
    where
        K: IntoVal<Env, Val>,
    {
        self.storage.has(key, StorageType::Instance)
    }

    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        V::Error: Debug,
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        self.storage.get(key, StorageType::Instance)
    }

    pub fn set<K, V>(&self, key: &K, val: &V)
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
        self.storage.set(key, val, StorageType::Instance)
    }

    #[inline(always)]
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, Val>,
    {
        self.storage.remove(key, StorageType::Instance)
    }

    pub fn bump(&self, min_ledgers_to_live: u32) {
        internal::Env::bump_current_contract_instance_and_code(
            &self.storage.env,
            min_ledgers_to_live.into(),
        )
        .unwrap_infallible();
    }
}
