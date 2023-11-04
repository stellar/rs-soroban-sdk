//! Prng contains a pseudo-random number generator.
//!
//! ## Warning
//!
//! Do not use the PRNG in this module without a clear understanding of two
//! major limitations in the way it is deployed in the Stellar network:
//!
//!   1. The PRNG is seeded with data that is public as soon as each ledger is
//!      nominated. Therefore it **should never be used to generate secrets**.
//!
//!   2. The PRNG is seeded with data that is under the control of validators.
//!      Therefore it **should only be used in applications where the risk of
//!      validator influence is acceptable**.
//!
//! The PRNG in this module is a strong CSPRNG (ChaCha20) and can be manually
//! re-seeded by contracts, in order to support commit/reveal schemes, oracles,
//! or similar advanced types of pseudo-random contract behaviour. Any PRNG is
//! however only as strong as its seed.
//!
//! The network runs in strict consensus, so every node in the network seeds its
//! PRNG with a consensus value, **not a random entropy source**. It uses data
//! that is generally difficult to predict in advance, and generally difficult
//! for network **users** to bias to a specific value: the seed is derived from
//! the overall transaction-set hash and the hash-sorted position number of each
//! transaction within it. But this seed is **not secret** and **not
//! cryptographically hard to bias** if a corrupt **validator** were to choose
//! to do so (similar to the way a corrupt validator can bias overall
//! transaction admission in the network).
//!
//! In other words the network will provide a stronger seed than a contract
//! could likely derive on-chain using any other public data visible to it (eg.
//! better than using a timestamp, ledger number, counter, or a similarly weak
//! seed) but weaker than a contract could acquire using a commit/reveal scheme
//! with an off-chain source of trusted random entropy.
//!
//! You should carefully consider whether these limitations are acceptable for
//! your application before using this module.
//!
//! ## Operation
//!
//! The host has a single hidden "base" PRNG that is seeded by the network. The
//! base PRNG is then used to seed separate, independent "local" PRNGs for each
//! contract invocation. This independence has the following characteristics:
//!
//!   - Contract invocations can only access (use or reseed) their local PRNG.
//!   - Contract invocations cannot influence any other invocation's local PRNG,
//!     except by influencing the other invocation to make a call to its PRNG.
//!   - Contracts cannot influence the base PRNG that seeds local PRNGs, except
//!     by making calls and thereby creating new local PRNGs with new seeds.
//!   - A contract invocation's local PRNG maintains state through the life of
//!     the invocation.
//!   - That state is advanced by each call from the invocation to a PRNG
//!     function in this module.
//!   - A contract invocation's local PRNG is destroyed after the invocation.
//!   - Any re-entry of a contract counts as a separate invocation.
//!
//! ## Testing
//!
//! In local tests, the base PRNG of each host is seeded to zero when the host
//! is constructed, so each contract invocation's local PRNG seed (and all its
//! PRNG-derived calls) will be determined strictly by its order of invocation
//! in the test. Assuming this order is stable, each test run should see stable
//! output from the local PRNG.
use core::ops::{Bound, RangeBounds};

use crate::{env::internal, unwrap::UnwrapInfallible, Bytes, Env, IntoVal, TryIntoVal, Val, Vec};

/// Prng is a pseudo-random generator.
///
/// # Warning
///
/// **The PRNG is unsuitable for generating secrets or use in applications with
/// low risk tolerance, see the module-level comment.**
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
    /// # Warning
    ///
    /// **The PRNG is unsuitable for generating secrets or use in applications with
    /// low risk tolerance, see the module-level comment.**
    pub fn seed(&self, seed: Bytes) {
        let env = self.env();
        internal::Env::prng_reseed(env, seed.into()).unwrap_infallible();
    }

    /// Fills the type with a random value.
    ///
    /// # Warning
    ///
    /// **The PRNG is unsuitable for generating secrets or use in applications with
    /// low risk tolerance, see the module-level comment.**
    ///
    /// # Examples
    ///
    /// ```
    /// # use soroban_sdk::{Env, contract, contractimpl, symbol_short, Bytes};
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
    /// let mut value: u64 = 0;
    /// env.prng().fill(&mut value);
    /// assert_eq!(value, 14156542310752927490);
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn fill<T>(&self, v: &mut T)
    where
        T: Fill,
    {
        v.fill(self);
    }

    /// Returns a random value of the given type.
    ///
    /// # Warning
    ///
    /// **The PRNG is unsuitable for generating secrets or use in applications with
    /// low risk tolerance, see the module-level comment.**
    ///
    /// # Examples
    ///
    /// ```
    /// # use soroban_sdk::{Env, contract, contractimpl, symbol_short, Bytes};
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
    /// let value: u64 = env.prng().gen();
    /// assert_eq!(value, 14156542310752927490);
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn gen<T>(&self) -> T
    where
        T: Gen,
    {
        T::gen(self)
    }

    /// Returns a random value of the given type in the range specified.
    ///
    /// # Panics
    ///
    /// If the start of the range is greater than the end.
    ///
    /// # Warning
    ///
    /// **The PRNG is unsuitable for generating secrets or use in applications with
    /// low risk tolerance, see the module-level comment.**
    ///
    /// # Examples
    ///
    /// ```
    /// # use soroban_sdk::{Env, contract, contractimpl, symbol_short, Bytes};
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
    /// // Get a value in the range of 1 to 100, inclusive.
    /// let value: u64 = env.prng().gen_range(1..=100);
    /// assert_eq!(value, 77);
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn gen_range<T>(&self, r: impl RangeBounds<T::RangeBound>) -> T
    where
        T: GenRange,
    {
        T::gen_range(self, r)
    }

    /// Returns a random u64 in the range specified.
    ///
    /// # Panics
    ///
    /// If the range is empty.
    ///
    /// # Warning
    ///
    /// **The PRNG is unsuitable for generating secrets or use in applications with
    /// low risk tolerance, see the module-level comment.**
    ///
    /// # Examples
    ///
    /// ```
    /// # use soroban_sdk::{Env, contract, contractimpl, symbol_short, Bytes};
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
    /// // Get a value in the range of 1 to 100, inclusive.
    /// let value = env.prng().u64_in_range(1..=100);
    /// assert_eq!(value, 77);
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    #[deprecated(note = "use env.prng().gen_range(...)")]
    pub fn u64_in_range(&self, r: impl RangeBounds<u64>) -> u64 {
        self.gen_range(r)
    }

    /// Shuffles a given vector v using the Fisher-Yates algorithm.
    ///
    /// # Warning
    ///
    /// **The PRNG is unsuitable for generating secrets or use in applications with
    /// low risk tolerance, see the module-level comment.**
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

/// Implemented by types that support being filled by a Prng.
pub trait Fill {
    /// Fills the given value with the Prng.
    fn fill(&mut self, prng: &Prng);
}

/// Implemented by types that support being generated by a Prng.
pub trait Gen {
    /// Generates a value of the implementing type with the Prng.
    fn gen(prng: &Prng) -> Self;
}

/// Implemented by types that support being generated in a specific range by a
/// Prng.
pub trait GenRange {
    type RangeBound;

    /// Generates a value of the implementing type with the Prng in the
    /// specified range.
    ///
    /// # Panics
    ///
    /// If the range is empty.
    fn gen_range(prng: &Prng, r: impl RangeBounds<Self::RangeBound>) -> Self;
}

impl Fill for u64 {
    fn fill(&mut self, prng: &Prng) {
        *self = Self::gen(prng);
    }
}

impl Gen for u64 {
    fn gen(prng: &Prng) -> Self {
        let env = prng.env();
        internal::Env::prng_u64_in_inclusive_range(env, u64::MIN.into(), u64::MAX.into())
            .unwrap_infallible()
            .into()
    }
}

impl GenRange for u64 {
    type RangeBound = u64;

    fn gen_range(prng: &Prng, r: impl RangeBounds<Self::RangeBound>) -> Self {
        let env = prng.env();
        let start_bound = match r.start_bound() {
            Bound::Included(b) => *b,
            Bound::Excluded(b) => *b + 1,
            Bound::Unbounded => u64::MIN,
        };
        let end_bound = match r.end_bound() {
            Bound::Included(b) => *b,
            Bound::Excluded(b) => *b - 1,
            Bound::Unbounded => u64::MAX,
        };
        internal::Env::prng_u64_in_inclusive_range(env, start_bound.into(), end_bound.into())
            .unwrap_infallible()
            .into()
    }
}
