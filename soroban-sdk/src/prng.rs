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

use crate::{
    env::internal,
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Bytes, BytesN, Env, IntoVal, Vec,
};

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
    /// ## `u64`
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
    /// assert_eq!(value, 8478755077819529274);
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    ///
    /// ## `[u8]`
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
    /// let mut value = [0u8; 32];
    /// env.prng().fill(&mut value);
    /// assert_eq!(
    ///   value,
    ///   [
    ///     58, 248, 248, 38, 210, 150, 170, 117, 122, 110, 9, 101, 244, 57,
    ///     221, 102, 164, 48, 43, 104, 222, 229, 242, 29, 25, 148, 88, 204,
    ///     130, 148, 2, 66
    ///   ],
    /// );
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn fill<T>(&self, v: &mut T)
    where
        T: Fill + ?Sized,
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
    /// ## `u64`
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
    /// assert_eq!(value, 8478755077819529274);
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    ///
    /// ## `[u8; N]`
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
    /// let value: [u8; 32] = env.prng().gen();
    /// assert_eq!(
    ///   value,
    ///   [
    ///     58, 248, 248, 38, 210, 150, 170, 117, 122, 110, 9, 101, 244, 57,
    ///     221, 102, 164, 48, 43, 104, 222, 229, 242, 29, 25, 148, 88, 204,
    ///     130, 148, 2, 66
    ///   ],
    /// );
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

    /// Returns a random value of the given type with the given length.
    ///
    /// # Panics
    ///
    /// If the length is greater than u32::MAX.
    ///
    /// # Warning
    ///
    /// **The PRNG is unsuitable for generating secrets or use in applications with
    /// low risk tolerance, see the module-level comment.**
    ///
    /// # Examples
    ///
    /// ## `Bytes`
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
    /// // Get a value of length 32 bytes.
    /// let value: Bytes = env.prng().gen_len(32);
    /// assert_eq!(value, Bytes::from_slice(
    ///   &env,
    ///   &[
    ///     58, 248, 248, 38, 210, 150, 170, 117, 122, 110, 9, 101, 244, 57,
    ///     221, 102, 164, 48, 43, 104, 222, 229, 242, 29, 25, 148, 88, 204,
    ///     130, 148, 2, 66
    ///   ],
    /// ));
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn gen_len<T>(&self, len: T::Len) -> T
    where
        T: GenLen,
    {
        T::gen_len(self, len)
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
    /// ## `u64`
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
    /// assert_eq!(value, 46);
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
    /// assert_eq!(value, 46);
    /// #     })
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    #[deprecated(note = "use env.prng().gen_range(...)")]
    pub fn u64_in_range(&self, r: impl RangeBounds<u64>) -> u64 {
        self.gen_range(r)
    }

    /// Shuffles a value using the Fisher-Yates algorithm.
    ///
    /// # Warning
    ///
    /// **The PRNG is unsuitable for generating secrets or use in applications with
    /// low risk tolerance, see the module-level comment.**
    pub fn shuffle<T>(&self, v: &mut T)
    where
        T: Shuffle,
    {
        v.shuffle(&self);
    }
}

impl<T> Shuffle for Vec<T> {
    fn shuffle(&mut self, prng: &Prng) {
        let env = prng.env();
        let obj = internal::Env::prng_vec_shuffle(env, self.to_object()).unwrap_infallible();
        *self = unsafe { Self::unchecked_new(env.clone(), obj) };
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

/// Implemented by types that support being generated of specific length by a
/// Prng.
pub trait GenLen {
    type Len;

    /// Generates a value of the given implementing type with length with the
    /// Prng.
    ///
    /// # Panics
    ///
    /// If the length is greater than u32::MAX.
    fn gen_len(prng: &Prng, len: Self::Len) -> Self;
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

/// Implemented by types that support being shuffled by a Prng.
pub trait Shuffle {
    /// Shuffles the value with the Prng.
    fn shuffle(&mut self, prng: &Prng);
}

/// Implemented by types that support being shuffled by a Prng.
pub trait ToShuffled {
    type Shuffled;
    fn to_shuffled(&self, prng: &Prng) -> Self::Shuffled;
}

impl<T: Shuffle + Clone> ToShuffled for T {
    type Shuffled = Self;
    fn to_shuffled(&self, prng: &Prng) -> Self {
        let mut copy = self.clone();
        copy.shuffle(prng);
        copy
    }
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

impl Fill for Bytes {
    /// Fills the Bytes with the Prng.
    ///
    /// # Panics
    ///
    /// If the length of Bytes is greater than u32::MAX in length.
    fn fill(&mut self, prng: &Prng) {
        let env = prng.env();
        let len: u32 = self.len().try_into().unwrap_optimized();
        let obj = internal::Env::prng_bytes_new(env, len.into()).unwrap_infallible();
        *self = unsafe { Bytes::unchecked_new(env.clone(), obj) };
    }
}

impl GenLen for Bytes {
    type Len = u32;
    /// Generates the Bytes with the Prng of the given length.
    fn gen_len(prng: &Prng, len: u32) -> Self {
        let env = prng.env();
        let obj = internal::Env::prng_bytes_new(env, len.into()).unwrap_infallible();
        unsafe { Bytes::unchecked_new(env.clone(), obj) }
    }
}

impl<const N: usize> Fill for BytesN<N> {
    /// Fills the BytesN with the Prng.
    ///
    /// # Panics
    ///
    /// If the length of BytesN is greater than u32::MAX in length.
    fn fill(&mut self, prng: &Prng) {
        let bytesn = Self::gen(prng);
        *self = bytesn;
    }
}

impl<const N: usize> Gen for BytesN<N> {
    /// Generates the BytesN with the Prng.
    ///
    /// # Panics
    ///
    /// If the length of BytesN is greater than u32::MAX in length.
    fn gen(prng: &Prng) -> Self {
        let env = prng.env();
        let len: u32 = N.try_into().unwrap_optimized();
        let obj = internal::Env::prng_bytes_new(env, len.into()).unwrap_infallible();
        unsafe { BytesN::unchecked_new(env.clone(), obj) }
    }
}

impl Fill for [u8] {
    /// Fills the slice with the Prng.
    ///
    /// # Panics
    ///
    /// If the slice is greater than u32::MAX in length.
    fn fill(&mut self, prng: &Prng) {
        let env = prng.env();
        let len: u32 = self.len().try_into().unwrap_optimized();
        let bytes: Bytes = internal::Env::prng_bytes_new(env, len.into())
            .unwrap_infallible()
            .into_val(env);
        bytes.copy_into_slice(self);
    }
}

impl<const N: usize> Fill for [u8; N] {
    /// Fills the array with the Prng.
    ///
    /// # Panics
    ///
    /// If the array is greater than u32::MAX in length.
    fn fill(&mut self, prng: &Prng) {
        let env = prng.env();
        let len: u32 = N.try_into().unwrap_optimized();
        let bytes: Bytes = internal::Env::prng_bytes_new(env, len.into())
            .unwrap_infallible()
            .into_val(env);
        bytes.copy_into_slice(self);
    }
}

impl<const N: usize> Gen for [u8; N] {
    /// Generates the array with the Prng.
    ///
    /// # Panics
    ///
    /// If the array is greater than u32::MAX in length.
    fn gen(prng: &Prng) -> Self {
        let mut v = [0u8; N];
        v.fill(prng);
        v
    }
}
