//! Prng contains a pseudo-random generator.
//!
//! # Warning
//!
//! **The pseudo-random generator contained in this module is not suitable for
//! security-sensitive work.**
//!
//! The entropy used to seed the generator is not strong. Every node in the
//! network executing a contract get exactly the same output. The value is hard
//! to predict, but trivial to derive once the network has determined the
//! transaction set for the ledger the invocation occurs in. The value is also
//! controllable by the node nominating.
use core::ops::{Bound, RangeBounds};

use crate::{env::internal, unwrap::UnwrapInfallible, Bytes, Env, IntoVal, TryIntoVal, Val, Vec};

/// Prng contains a pseudo-random generator.
pub struct Prng {
    env: Env,
}

impl Prng {
    pub(crate) fn new(env: &Env) -> Prng {
        Prng { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Reseeds the PRNG with the provided `seed` value.
    ///
    /// The seed is combined with the seed assigned to the contract invocation.
    ///
    /// # Warning
    ///
    /// **The pseudo-random generator contained in this module is not suitable for
    /// security-sensitive work.**
    pub fn seed(&self, seed: Bytes) {
        let env = self.env();
        internal::Env::prng_reseed(env, seed.into()).unwrap_infallible();
    }

    /// Returns a random u64 in the range between `lower` and `upper` inclusive.
    ///
    /// ### Panics
    ///
    /// If the range is empty.
    ///
    /// # Warning
    ///
    /// **The pseudo-random generator contained in this module is not suitable for
    /// security-sensitive work.**
    pub fn u64_in_range(&self, r: impl RangeBounds<u64>) -> u64 {
        let start_bound = match r.start_bound() {
            Bound::Included(b) => *b,
            Bound::Excluded(b) => *b + 1,
            Bound::Unbounded => 0,
        };
        let end_bound = match r.end_bound() {
            Bound::Included(b) => *b,
            Bound::Excluded(b) => *b - 1,
            Bound::Unbounded => u64::MAX,
        };
        let env = self.env();
        internal::Env::prng_u64_in_inclusive_range(env, start_bound.into(), end_bound.into())
            .unwrap_infallible()
            .into()
    }

    /// Shuffles a given vector v using the Fisher-Yates algorithm.
    ///
    /// # Warning
    ///
    /// **The pseudo-random generator contained in this module is not suitable for
    /// security-sensitive work.**
    pub fn shuffle<V>(&self, v: V) -> Vec<Val>
    where
        V: IntoVal<Env, Vec<Val>>,
    {
        let env = self.env();
        let v_val = v.into_val(env);

        internal::Env::prng_vec_shuffle(env, v_val.to_object())
            .unwrap_infallible()
            .try_into_val(env)
            .unwrap_infallible()
    }
}
