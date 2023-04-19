#![cfg(any(test, feature = "testutils"))]
#![cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]

//! Utilities intended for use when testing.

mod sign;

pub use sign::ed25519;

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
    use core::fmt::Display;

    #[doc(inline)]
    pub use crate::env::internal::budget::CostType;

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
            writeln!(f, "Costs:")?;
            writeln!(f, "- CPU Instructions: {}", self.cpu_instruction_cost())?;
            writeln!(f, "- Memory Bytes: {}", self.memory_bytes_cost())?;
            writeln!(f, "Inputs:")?;
            for cost_type in CostType::variants() {
                writeln!(f, "- {cost_type:?}: {}", self.input(*cost_type))?;
            }
            Ok(())
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

        pub fn reset_inputs(&mut self) {
            self.0.reset_inputs();
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

        /// Get inputs that have led to the cost.
        ///
        /// Note that VM cost types are likely to be underestimated when running
        /// Rust code compared to running the WASM equivalent.
        pub fn input(&self, cost_type: CostType) -> u64 {
            self.0.get_input(cost_type)
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
    fn all(&self) -> Vec<(crate::BytesN<32>, Vec<RawVal>, RawVal)>;
}

/// Test utilities for [`Logger`][crate::logging::Logger].
pub trait Logger {
    /// Returns all debug events that have been logged.
    fn all(&self) -> std::vec::Vec<String>;
    /// Prints all debug events to stdout.
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
    /// Create a random contract Address.
    fn random_contract(env: &Env) -> crate::Address;

    /// Create a random account Address.
    fn random_account(env: &Env) -> crate::Address;
}
