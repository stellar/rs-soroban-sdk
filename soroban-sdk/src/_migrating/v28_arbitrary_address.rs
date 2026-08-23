//! The [`Address`] prototype generates account addresses as well as contract addresses, and
//! [`Address`] has a prototype for each kind.
//!
//! [`SorobanArbitrary`] gives every contract type a _prototype_ type that implements
//! [`Arbitrary`], and a fuzz test or property test converts the prototype into the contract type
//! with an [`Env`]. The prototype of [`Address`], [`ArbitraryAddress`], only ever built contract
//! (`C...`) addresses, so a fuzz test never exercised the account (`G...`) branch of a contract's
//! logic.
//!
//! In v28 `Address` has three prototypes:
//!
//! - [`ArbitraryAddress`], the default named by `<Address as SorobanArbitrary>::Prototype`, which
//!   generates account and contract addresses in roughly equal proportion.
//! - [`ArbitraryAddressAccount`], which generates only account addresses.
//! - [`ArbitraryAddressContract`], which generates only contract addresses, and so generates what
//!   `ArbitraryAddress` generated before v28.
//!
//! All three convert to an [`Address`] with [`IntoVal`] or [`FromVal`], and any of them can be
//! named wherever a fuzz test or property test needs a specific kind of address, including inside
//! the [`Vec`], [`Map`], and [`Option`] prototypes.
//!
//! ## Migrating
//!
//! Most fuzz tests need no change: an `Address` prototype now covers both kinds of address, which
//! is more coverage than before, and a contract that accepts an `Address` should handle both.
//!
//! Two things to check when upgrading:
//!
//! 1. **Review fuzz tests and property tests that assume an address is a contract.** Such a test
//!    may fail now that account addresses are generated. A test that holds a Stellar asset is the
//!    common case: a contract address holds an asset with nothing set up beforehand, while an
//!    account address holds one only once the account exists and has an authorized trustline for
//!    the asset, which the test must create itself. A test can change to using
//!    [`ArbitraryAddressContract`] to avoid the setup needs of an account.
//!
//! 2. **Regenerate fuzz corpora that contain `Address` values.** The prototype's byte layout has
//!    changed, so the corpus entries a fuzzer accumulated still decode, but decode to different
//!    values than they did before, and the coverage a minimized corpus recorded no longer holds.
//!    Corpora keep working; the crashes they reproduced may need re-triaging. A test that would
//!    rather keep its corpus than gain account addresses can name [`ArbitraryAddressContract`]
//!    wherever it named [`ArbitraryAddress`], which has the byte layout the default had before
//!    v28, so its corpus decodes to the addresses it decoded to before.
//!
//! [`ArbitraryAddress`] and [`ArbitraryMuxedAddress`] are also `#[non_exhaustive]`, so a `match` on
//! either needs a wildcard arm.
//!
//! To generate only contract addresses, which is what the default prototype generated before v28,
//! name [`ArbitraryAddressContract`]:
//!
//! ```
//! # macro_rules! fuzz_target {
//! #     (|$data:ident: $dty: ty| $body:block) => { };
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//! use soroban_sdk::testutils::arbitrary::{Arbitrary, ArbitraryAddressContract};
//! use soroban_sdk::{Address, Env, IntoVal};
//!
//! #[derive(Arbitrary, Debug)]
//! struct Input {
//!     // 👇 👀 Always a contract address.
//!     spender: ArbitraryAddressContract,
//!     amount: i128,
//! }
//!
//! fuzz_target!(|input: Input| {
//!     let env = Env::default();
//!     let spender: Address = input.spender.into_val(&env);
//!     // call the contract with `spender` and `input.amount`
//! });
//! # }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! [`Address`]: crate::Address
//! [`Arbitrary`]: crate::testutils::arbitrary::Arbitrary
//! [`ArbitraryAddress`]: crate::testutils::arbitrary::ArbitraryAddress
//! [`ArbitraryAddressAccount`]: crate::testutils::arbitrary::ArbitraryAddressAccount
//! [`ArbitraryAddressContract`]: crate::testutils::arbitrary::ArbitraryAddressContract
//! [`ArbitraryMuxedAddress`]: crate::testutils::arbitrary::ArbitraryMuxedAddress
//! [`Env`]: crate::Env
//! [`FromVal`]: crate::FromVal
//! [`IntoVal`]: crate::IntoVal
//! [`Map`]: crate::Map
//! [`SorobanArbitrary`]: crate::testutils::arbitrary::SorobanArbitrary
//! [`Vec`]: crate::Vec
