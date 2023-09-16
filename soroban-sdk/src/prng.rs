//! Prng contains a pseudo-random generator.
//!
//! # Warning
//!
//! **The pseudo-random generator contained in this module is not suitable for
//! security-sensitive work.**
//!
//! The entropy used to seed the generator is not strong. Every node in the
//! network executing a contract get exactly the same output.  The value is hard
//! to predict, but trivial to derive once the network has determined the inputs
//! into the ledger the invocation occurs in. The value is also controllable by
//! the node nominating. Therefore, the results of the pseudo-random number
//! generator are determinable once the inputs to a ledger are known.
//!
//! Every contract invocation gets its own, independent seed. If a contract
//! invocation fails, the seed from the failed invocation is not reused for the
//! next invocation of the contract.
//!
//! In tests, the contract invocation seed is consistently zero, and tests will
//! receive consistent results from the PRNG.
use core::ops::{Bound, RangeBounds};

use crate::{env::internal, unwrap::UnwrapInfallible, Bytes, Env, IntoVal, TryIntoVal, Val, Vec};

/// Prng is a pseudo-random generator.
///
/// # Warning
///
/// **The pseudo-random generator contained in this module is not suitable for
/// security-sensitive work.**
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

    /// Reseeds the PRNG with the provided value.
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

    /// Returns a random u64 in the range specified.
    ///
    /// # Panics
    ///
    /// If the range is empty.
    ///
    /// # Warning
    ///
    /// **The pseudo-random generator contained in this module is not suitable for
    /// security-sensitive work.**
    ///
    /// # Examples
    ///
    /// ```
    /// use soroban_sdk::{Env};
    ///
    /// # use soroban_sdk::{contract, contractimpl, symbol_short, Bytes};
    /// #
    /// # #[contract]
    /// # pub struct Contract;
    /// #
    /// # #[cfg(feature = "testutils")]
    /// # fn main() {
    /// #     let env = Env::default();
    /// #     let contract_id = env.register_contract(None, Contract);
    /// #     env.as_contract(&contract_id, || {
    /// #         env.prng().seed(Bytes::from_array(&env, &[1; 32]));
    /// // Get values in the range of 1 to 100, inclusive.
    /// let value = env.prng().u64_in_range(1..=100);
    /// assert_eq!(value, 77);
    /// let value = env.prng().u64_in_range(1..=100);
    /// assert_eq!(value, 66);
    /// let value = env.prng().u64_in_range(1..=100);
    /// assert_eq!(value, 72);
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
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
