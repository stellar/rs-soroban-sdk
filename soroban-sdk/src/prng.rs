//! Prng contains functions for pseudo-random functions.
use core::ops::{Bound, RangeBounds};

use crate::{env::internal, unwrap::UnwrapInfallible, Bytes, Env, IntoVal, TryIntoVal, Val, Vec};

/// Prng provides access to pseudo-random  functions.
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

    // Reseeds the pseudorandom number generator (PRNG) with the provided `seed` value.
    pub fn seed(&self, seed: Bytes) {
        let env = self.env();
        internal::Env::prng_reseed(env, seed.into()).unwrap_infallible();
    }

    // Returns a random u64 in the range between `lower` and `upper` inclusive.
    //
    // ### Panics
    //
    // If the range is empty.
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

    // Shuffles a given vector v using the Fisher-Yates algorithm.
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
