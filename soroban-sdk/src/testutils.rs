#![cfg(any(test, feature = "testutils"))]
#![cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]

//! Utilities intended for use when testing.

mod sign;
pub use sign::ed25519;

mod mock_auth;
pub use mock_auth::{MockAuth, MockAuthContract, MockAuthInvoke};

use crate::{Env, RawVal, Vec};

#[doc(hidden)]
pub trait ContractFunctionSet {
    fn call(&self, func: &str, env: Env, args: &[RawVal]) -> Option<RawVal>;
}

#[doc(inline)]
pub use crate::env::internal::LedgerInfo;

/// Test utilities for [`Ledger`][crate::ledger::Ledger].
pub trait Ledger {
    /// Set ledger info.
    fn set(&self, l: LedgerInfo);

    /// Get ledger info.
    fn get(&self) -> LedgerInfo;

    /// Modify the ledger info.
    fn with_mut<F>(&self, f: F)
    where
        F: FnMut(&mut LedgerInfo);
}

pub mod budget {
    use core::fmt::{Debug, Display};

    #[doc(inline)]
    pub use crate::xdr::ContractCostType;

    /// Budget that tracks the resources consumed for the environment.
    ///
    /// The budget consistents of two cost dimensions:
    ///  - CPU instructions
    ///  - Memory
    ///
    /// Inputs feed into those cost dimensions.
    ///
    /// Note that all cost dimensions – CPU instructions, memory – and the VM
    /// cost type inputs are likely to be underestimated when running Rust code
    /// compared to running the WASM equivalent.
    ///
    /// ### Examples
    ///
    /// ```
    /// use soroban_sdk::{Env, Symbol};
    ///
    /// # #[cfg(feature = "testutils")]
    /// # fn main() {
    /// #     let env = Env::default();
    /// env.budget().reset_default();
    /// // ...
    /// println!("{}", env.budget());
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub struct Budget(crate::env::internal::budget::Budget);

    impl Display for Budget {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            writeln!(f, "{}", self.0)
        }
    }

    impl Debug for Budget {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            writeln!(f, "{:?}", self.0)
        }
    }

    impl Budget {
        pub(crate) fn new(b: crate::env::internal::budget::Budget) -> Self {
            Self(b)
        }

        /// Reset the budget.
        pub fn reset_default(&mut self) {
            self.0.reset_default();
        }

        pub fn reset_unlimited(&mut self) {
            self.0.reset_unlimited();
        }

        pub fn reset_limits(&mut self, cpu: u64, mem: u64) {
            self.0.reset_limits(cpu, mem);
        }

        pub fn reset_tracker(&mut self) {
            self.0.reset_tracker();
        }

        /// Returns the CPU instruction cost.
        ///
        /// Note that CPU instructions are likely to be underestimated when
        /// running Rust code compared to running the WASM equivalent.
        pub fn cpu_instruction_cost(&self) -> u64 {
            self.0.get_cpu_insns_count()
        }

        /// Returns the memory cost.
        ///
        /// Note that memory is likely to be underestimated when running Rust
        /// code compared to running the WASM equivalent.
        pub fn memory_bytes_cost(&self) -> u64 {
            self.0.get_mem_bytes_count()
        }

        /// Get the input tracker associated with the cost. The tracker tracks
        /// the cumulative (iterations, inputs). If the underlying model is a
        /// constant model, then inputs is `None` and only iterations matter.
        ///
        /// Note that VM cost types are likely to be underestimated when running
        /// Rust code compared to running the WASM equivalent.
        pub fn tracker(&self, cost_type: ContractCostType) -> (u64, Option<u64>) {
            self.0.get_tracker(cost_type)
        }

        /// Print the budget costs and inputs to stdout.
        pub fn print(&self) {
            println!("{}", self.0);
        }
    }
}

/// Test utilities for [`Events`][crate::events::Events].
pub trait Events {
    /// Returns all events that have been published by contracts.
    ///
    /// Returns a [`Vec`] of three element tuples containing:
    /// - Contract ID
    /// - Event Topics as a [`Vec<RawVal>`]
    /// - Event Data as a [`RawVal`]
    fn all(&self) -> Vec<(crate::Address, Vec<RawVal>, RawVal)>;
}

/// Test utilities for [`Logger`][crate::logging::Logger].
pub trait Logger {
    /// Returns all diagnostic events that have been logged.
    fn all(&self) -> std::vec::Vec<String>;
    /// Prints all diagnostic events to stdout.
    fn print(&self);
}

/// Test utilities for [`BytesN`][crate::BytesN].
pub trait BytesN<const N: usize> {
    // Generate a BytesN filled with random bytes.
    //
    // The value filled is not cryptographically secure.
    fn random(env: &Env) -> crate::BytesN<N>;
}

/// Generates an array of N random bytes.
///
/// The value returned is not cryptographically secure.
pub(crate) fn random<const N: usize>() -> [u8; N] {
    use rand::RngCore;
    let mut arr = [0u8; N];
    rand::thread_rng().fill_bytes(&mut arr);
    arr
}

pub trait Address {
    /// Create a random Address.
    ///
    /// Implementation note: this always builds the contract addresses now. This
    /// shouldn't normally matter though, as contracts should be agnostic to
    /// the underlying Address value.
    fn random(env: &Env) -> crate::Address;

    /// Creates an `Address` corresponding to the provided contract identifier.
    fn from_contract_id(contract_id: &crate::BytesN<32>) -> crate::Address;

    /// Get the contract ID of an Address as a BytesN<32>.
    ///
    /// ### Panics
    ///
    /// If the Address is not a Contract.
    fn contract_id(&self) -> crate::BytesN<32>;
}
