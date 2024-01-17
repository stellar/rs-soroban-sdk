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
/// instance entry, allowing users to tie that data directly to the TTL
/// of the instance. Instance storage is good for global contract data like
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

    /// Returns the maximum TTL (number of ledgers that an entry can have rent paid
    /// for it in one moment).
    ///
    /// When counting the number of ledgers an entry is active for, the current
    /// ledger is included. If an entry is created in the current ledger, its
    /// maximum live_until ledger will be the TTL (value returned from
    /// the function) plus the current ledger. This means the last ledger
    /// that the entry will be accessible will be the current ledger sequence
    /// plus the max TTL minus one.
    pub fn max_ttl(&self) -> u32 {
        let seq = self.env.ledger().sequence();
        let max = self.env.ledger().max_live_until_ledger();
        max - seq + 1
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
    pub(crate) fn get<K, V>(&self, key: &K, storage_type: StorageType) -> Option<V>
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

    /// Update a value stored against a key.
    ///
    /// Loads the value, calls the function with it, then sets the value to the
    /// returned value of the function.  If no value is stored with the key then
    /// the function is called with None.
    ///
    /// The returned value is the value stored after updating.
    pub(crate) fn update<K, V>(
        &self,
        key: &K,
        storage_type: StorageType,
        f: impl FnOnce(Option<V>) -> V,
    ) -> V
    where
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
        let key = key.into_val(&self.env);
        let val = self.get(&key, storage_type);
        let val = f(val);
        self.set(&key, &val, storage_type);
        val
    }

    /// Update a value stored against a key.
    ///
    /// Loads the value, calls the function with it, then sets the value to the
    /// returned value of the function.  If no value is stored with the key then
    /// the function is called with None.  If the function returns an error it
    /// will be passed through.
    ///
    /// The returned value is the value stored after updating.
    pub(crate) fn try_update<K, V, E>(
        &self,
        key: &K,
        storage_type: StorageType,
        f: impl FnOnce(Option<V>) -> Result<V, E>,
    ) -> Result<V, E>
    where
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
        let key = key.into_val(&self.env);
        let val = self.get(&key, storage_type);
        let val = f(val)?;
        self.set(&key, &val, storage_type);
        Ok(val)
    }

    pub(crate) fn extend_ttl<K>(
        &self,
        key: &K,
        storage_type: StorageType,
        threshold: u32,
        extend_to: u32,
    ) where
        K: IntoVal<Env, Val>,
    {
        let env = &self.env;
        internal::Env::extend_contract_data_ttl(
            env,
            key.into_val(env),
            storage_type,
            threshold.into(),
            extend_to.into(),
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

    /// Update a value stored against a key.
    ///
    /// Loads the value, calls the function with it, then sets the value to the
    /// returned value of the function.  If no value is stored with the key then
    /// the function is called with None.
    ///
    /// The returned value is the value stored after updating.
    pub fn update<K, V>(&self, key: &K, f: impl FnOnce(Option<V>) -> V) -> V
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        self.storage.update(key, StorageType::Persistent, f)
    }

    /// Update a value stored against a key.
    ///
    /// Loads the value, calls the function with it, then sets the value to the
    /// returned value of the function.  If no value is stored with the key then
    /// the function is called with None.  If the function returns an error it
    /// will be passed through.
    ///
    /// The returned value is the value stored after updating.
    pub fn try_update<K, V, E>(
        &self,
        key: &K,
        f: impl FnOnce(Option<V>) -> Result<V, E>,
    ) -> Result<V, E>
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        self.storage.try_update(key, StorageType::Persistent, f)
    }

    pub fn extend_ttl<K>(&self, key: &K, threshold: u32, extend_to: u32)
    where
        K: IntoVal<Env, Val>,
    {
        self.storage
            .extend_ttl(key, StorageType::Persistent, threshold, extend_to)
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

    /// Update a value stored against a key.
    ///
    /// Loads the value, calls the function with it, then sets the value to the
    /// returned value of the function.  If no value is stored with the key then
    /// the function is called with None.
    ///
    /// The returned value is the value stored after updating.
    pub fn update<K, V>(&self, key: &K, f: impl FnOnce(Option<V>) -> V) -> V
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        self.storage.update(key, StorageType::Temporary, f)
    }

    /// Update a value stored against a key.
    ///
    /// Loads the value, calls the function with it, then sets the value to the
    /// returned value of the function.  If no value is stored with the key then
    /// the function is called with None.  If the function returns an error it
    /// will be passed through.
    ///
    /// The returned value is the value stored after updating.
    pub fn try_update<K, V, E>(
        &self,
        key: &K,
        f: impl FnOnce(Option<V>) -> Result<V, E>,
    ) -> Result<V, E>
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        self.storage.try_update(key, StorageType::Temporary, f)
    }

    pub fn extend_ttl<K>(&self, key: &K, threshold: u32, extend_to: u32)
    where
        K: IntoVal<Env, Val>,
    {
        self.storage
            .extend_ttl(key, StorageType::Temporary, threshold, extend_to)
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

    /// Update a value stored against a key.
    ///
    /// Loads the value, calls the function with it, then sets the value to the
    /// returned value of the function.  If no value is stored with the key then
    /// the function is called with None.
    ///
    /// The returned value is the value stored after updating.
    pub fn update<K, V>(&self, key: &K, f: impl FnOnce(Option<V>) -> V) -> V
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        self.storage.update(key, StorageType::Instance, f)
    }

    /// Update a value stored against a key.
    ///
    /// Loads the value, calls the function with it, then sets the value to the
    /// returned value of the function.  If no value is stored with the key then
    /// the function is called with None.  If the function returns an error it
    /// will be passed through.
    ///
    /// The returned value is the value stored after updating.
    pub fn try_update<K, V, E>(
        &self,
        key: &K,
        f: impl FnOnce(Option<V>) -> Result<V, E>,
    ) -> Result<V, E>
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        self.storage.try_update(key, StorageType::Instance, f)
    }

    #[inline(always)]
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, Val>,
    {
        self.storage.remove(key, StorageType::Instance)
    }

    pub fn extend_ttl(&self, threshold: u32, extend_to: u32) {
        internal::Env::extend_current_contract_instance_and_code_ttl(
            &self.storage.env,
            threshold.into(),
            extend_to.into(),
        )
        .unwrap_infallible();
    }
}

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
mod testutils {
    use super::*;
    use crate::{testutils, xdr, Map, TryIntoVal};

    impl testutils::storage::Instance for Instance {
        fn all(&self) -> Map<Val, Val> {
            let env = &self.storage.env;
            let storage = env.host().with_mut_storage(|s| Ok(s.map.clone())).unwrap();
            let address: xdr::ScAddress = env.current_contract_address().try_into().unwrap();
            for entry in storage {
                let (k, Some((v, _))) = entry else {
                    continue;
                };
                let xdr::LedgerKey::ContractData(xdr::LedgerKeyContractData {
                    ref contract, ..
                }) = *k
                else {
                    continue;
                };
                if contract != &address {
                    continue;
                }
                let xdr::LedgerEntry {
                    data:
                        xdr::LedgerEntryData::ContractData(xdr::ContractDataEntry {
                            key: xdr::ScVal::LedgerKeyContractInstance,
                            val:
                                xdr::ScVal::ContractInstance(xdr::ScContractInstance {
                                    ref storage,
                                    ..
                                }),
                            ..
                        }),
                    ..
                } = *v
                else {
                    continue;
                };
                return match storage {
                    Some(map) => {
                        let map: Val =
                            Val::try_from_val(env, &xdr::ScVal::Map(Some(map.clone()))).unwrap();
                        map.try_into_val(env).unwrap()
                    }
                    None => Map::new(env),
                };
            }
            panic!("contract instance for current contract address not found");
        }
    }

    impl testutils::storage::Persistent for Persistent {
        fn all(&self) -> Map<Val, Val> {
            all(&self.storage.env, xdr::ContractDataDurability::Persistent)
        }
    }

    impl testutils::storage::Temporary for Temporary {
        fn all(&self) -> Map<Val, Val> {
            all(&self.storage.env, xdr::ContractDataDurability::Temporary)
        }
    }

    fn all(env: &Env, d: xdr::ContractDataDurability) -> Map<Val, Val> {
        let storage = env.host().with_mut_storage(|s| Ok(s.map.clone())).unwrap();
        let mut map = Map::<Val, Val>::new(env);
        for entry in storage {
            let (_, Some((v, _))) = entry else {
                continue;
            };
            let xdr::LedgerEntry {
                data:
                    xdr::LedgerEntryData::ContractData(xdr::ContractDataEntry {
                        ref key,
                        ref val,
                        durability,
                        ..
                    }),
                ..
            } = *v
            else {
                continue;
            };
            if d != durability {
                continue;
            }
            let Ok(key) = Val::try_from_val(env, key) else {
                continue;
            };
            let Ok(val) = Val::try_from_val(env, val) else {
                continue;
            };
            map.set(key, val);
        }
        map
    }
}
