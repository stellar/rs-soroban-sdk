//! Typed contract-data storage support.
use core::{cmp::Ordering, fmt::Debug};

use crate::{
    env::internal::{Compare, StorageType, Val},
    Env, IntoVal, Symbol, TryFromVal,
};

/// An explicit policy for extending an entry's TTL.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TtlPolicy {
    None,
    BumpTo { extend_to: u32 },
    BumpWhenBelow { threshold: u32, extend_to: u32 },
}

/// Errors returned by [`ContractDataKey::update`] and [`FixedContractData::update`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpdateError<E> {
    /// No value exists at the requested storage key.
    Missing,
    /// The update closure returned an error.
    User(E),
    /// A dynamic update attempted to change one or more `#[key]` fields.
    KeyChanged,
}

impl TtlPolicy {
    pub(crate) fn threshold_and_extend_to(self) -> Option<(u32, u32)> {
        match self {
            Self::None => None,
            Self::BumpTo { extend_to } => Some((extend_to, extend_to)),
            Self::BumpWhenBelow {
                threshold,
                extend_to,
            } => Some((threshold, extend_to)),
        }
    }
}

/// Metadata for a value stored by a contract.
pub trait ContractData: Sized {
    /// The Soroban storage class used for this value.
    const STORAGE: StorageType;
    /// The policy applied when [`ContractDataKey::bump`] or
    /// [`FixedContractData::bump`] is called.
    const TTL: TtlPolicy = TtlPolicy::None;
}

/// A contract-data value whose key is derived from call-time arguments.
pub trait ContractDataKey: ContractData {
    /// Arguments that identify an entry.
    type Args;
    /// The storage key, which may borrow from [`Self::Args`] or `self`.
    type Key<'a>: IntoVal<Env, Val>
    where
        Self: 'a;
    /// The serialized body stored under the derived key.
    type Stored: IntoVal<Env, Val> + TryFromVal<Env, Val>;

    /// Builds a storage key from entry-identifying arguments.
    fn key<'a>(env: &Env, args: &'a Self::Args) -> Self::Key<'a>;
    /// Builds a storage key from this value's key fields.
    fn key_from_value(&self, env: &Env) -> Self::Key<'_>;
    /// Extracts the non-key fields that are written to storage.
    fn into_stored(&self) -> Self::Stored;
    /// Reconstructs this value from its identifying arguments and stored body.
    fn from_stored(args: &Self::Args, stored: Self::Stored) -> Self;

    /// Loads the entry identified by `args`, if it exists.
    fn get(env: &Env, args: &Self::Args) -> Option<Self>
    where
        <Self::Stored as TryFromVal<Env, Val>>::Error: Debug,
    {
        let key = Self::key(env, args);
        env.storage()
            .get(&key, Self::STORAGE)
            .map(|stored| Self::from_stored(args, stored))
    }

    /// Mutates an existing entry and saves it when `f` succeeds.
    ///
    /// Returns [`UpdateError::Missing`] when no entry exists, propagates errors
    /// from `f` as [`UpdateError::User`], and rejects changes to `#[key]` fields.
    fn update<E>(
        env: &Env,
        args: &Self::Args,
        f: impl FnOnce(&mut Self) -> Result<(), E>,
    ) -> Result<(), UpdateError<E>>
    where
        <Self::Stored as TryFromVal<Env, Val>>::Error: Debug,
    {
        let original_key = Self::key(env, args).into_val(env);
        let mut value = Self::get(env, args).ok_or(UpdateError::Missing)?;
        f(&mut value).map_err(UpdateError::User)?;
        if env
            .compare(&value.key_from_value(env).into_val(env), &original_key)
            .expect("contract-data keys must be comparable")
            != Ordering::Equal
        {
            return Err(UpdateError::KeyChanged);
        }
        value.save(env);
        Ok(())
    }

    /// Stores this value and returns it for fluent calls such as `.save(env).bump(env)`.
    fn save(&self, env: &Env) -> &Self {
        env.storage().set(
            &self.key_from_value(env),
            &self.into_stored(),
            Self::STORAGE,
        );
        self
    }

    /// Returns whether an entry identified by `args` exists.
    fn has(env: &Env, args: &Self::Args) -> bool {
        env.storage().has(&Self::key(env, args), Self::STORAGE)
    }

    /// Removes the entry identified by `args`.
    fn remove(env: &Env, args: &Self::Args) {
        env.storage().remove(&Self::key(env, args), Self::STORAGE);
    }

    /// Bumps this entry's TTL according to [`ContractData::TTL`], then returns it.
    fn bump(&self, env: &Env) -> &Self {
        let Some((threshold, extend_to)) = Self::TTL.threshold_and_extend_to() else {
            return self;
        };
        env.storage().extend_ttl(
            &self.key_from_value(env),
            Self::STORAGE,
            threshold,
            extend_to,
        );
        self
    }
}

/// A contract-data value with a fixed Symbol key.
pub trait FixedContractData: ContractData + IntoVal<Env, Val> + TryFromVal<Env, Val>
where
    Self::Error: Debug,
{
    /// The fixed Symbol key used for this value.
    const KEY: &'static str;

    /// Loads the value, if it exists.
    fn get(env: &Env) -> Option<Self> {
        env.storage()
            .get(&Symbol::new(env, Self::KEY), Self::STORAGE)
    }

    /// Mutates the stored value and saves it when `f` succeeds.
    ///
    /// Returns [`UpdateError::Missing`] when no value exists and propagates
    /// errors from `f` as [`UpdateError::User`].
    fn update<E>(
        env: &Env,
        f: impl FnOnce(&mut Self) -> Result<(), E>,
    ) -> Result<(), UpdateError<E>> {
        let mut value = Self::get(env).ok_or(UpdateError::Missing)?;
        f(&mut value).map_err(UpdateError::User)?;
        value.save(env);
        Ok(())
    }

    /// Stores this value and returns it for fluent calls such as `.save(env).bump(env)`.
    fn save(&self, env: &Env) -> &Self {
        env.storage()
            .set(&Symbol::new(env, Self::KEY), self, Self::STORAGE);
        self
    }

    /// Returns whether the value exists.
    fn has(env: &Env) -> bool {
        env.storage()
            .has(&Symbol::new(env, Self::KEY), Self::STORAGE)
    }

    /// Removes the value.
    fn remove(env: &Env) {
        env.storage()
            .remove(&Symbol::new(env, Self::KEY), Self::STORAGE);
    }

    /// Bumps this value's TTL according to [`ContractData::TTL`], then returns it.
    fn bump(&self, env: &Env) -> &Self {
        let Some((threshold, extend_to)) = Self::TTL.threshold_and_extend_to() else {
            return self;
        };
        env.storage().extend_ttl(
            &Symbol::new(env, Self::KEY),
            Self::STORAGE,
            threshold,
            extend_to,
        );
        self
    }
}
