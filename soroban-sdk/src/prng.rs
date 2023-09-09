//! Prng contains functions for pseudo-random functions.
use crate::{
    env::internal, unwrap::UnwrapInfallible, Bytes, Env, IntoVal, TryIntoVal, Val,
    Vec,
};

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
    pub fn prng_reseed(&self, seed: &Bytes) {
        let env = self.env();
        internal::Env::prng_reseed(env, seed.into()).unwrap_infallible();
    }

    // Returns a random u64 in the range between `lower` and `upper` inclusive.
    pub fn prng_u64_in_inclusive_range(&self, lower: u64, upper: u64) -> u64 {
        let env = self.env();
        internal::Env::prng_u64_in_inclusive_range(env, lower.into(), upper.into())
            .unwrap_infallible()
            .into()
    }

    // Shuffles a given vector v using the Fisher-Yates algorithm.
    pub fn prng_vec_shuffle<V>(&self, v: V) -> Vec<Val>
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
