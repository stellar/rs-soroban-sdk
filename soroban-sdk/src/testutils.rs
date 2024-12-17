#![cfg(any(test, feature = "testutils"))]
#![cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]

//! Utilities intended for use when testing.

pub mod arbitrary;

mod sign;
use std::rc::Rc;

pub use sign::ed25519;

mod mock_auth;
pub use mock_auth::{
    AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthContract, MockAuthInvoke,
};
use soroban_env_host::TryIntoVal;

pub mod storage;

pub mod cost_estimate;

use crate::{xdr, ConstructorArgs, Env, Val, Vec};
use soroban_ledger_snapshot::LedgerSnapshot;

pub use crate::env::EnvTestConfig;

pub trait Register {
    fn register<'i, I, A>(self, env: &Env, id: I, args: A) -> crate::Address
    where
        I: Into<Option<&'i crate::Address>>,
        A: ConstructorArgs;
}

impl<C> Register for C
where
    C: ContractFunctionSet + 'static,
{
    fn register<'i, I, A>(self, env: &Env, id: I, args: A) -> crate::Address
    where
        I: Into<Option<&'i crate::Address>>,
        A: ConstructorArgs,
    {
        env.register_contract_with_constructor(id, self, args)
    }
}

impl<'w> Register for &'w [u8] {
    fn register<'i, I, A>(self, env: &Env, id: I, args: A) -> crate::Address
    where
        I: Into<Option<&'i crate::Address>>,
        A: ConstructorArgs,
    {
        env.register_contract_wasm_with_constructor(id, self, args)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Snapshot {
    pub generators: Generators,
    pub auth: AuthSnapshot,
    pub ledger: LedgerSnapshot,
    pub events: EventsSnapshot,
}

impl Snapshot {
    // Read in a [`Snapshot`] from a reader.
    pub fn read(r: impl std::io::Read) -> Result<Snapshot, std::io::Error> {
        Ok(serde_json::from_reader::<_, Snapshot>(r)?)
    }

    // Read in a [`Snapshot`] from a file.
    pub fn read_file(p: impl AsRef<std::path::Path>) -> Result<Snapshot, std::io::Error> {
        Self::read(std::fs::File::open(p)?)
    }

    // Write a [`Snapshot`] to a writer.
    pub fn write(&self, w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(serde_json::to_writer_pretty(w, self)?)
    }

    // Write a [`Snapshot`] to file.
    pub fn write_file(&self, p: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        let p = p.as_ref();
        if let Some(dir) = p.parent() {
            if !dir.exists() {
                std::fs::create_dir_all(dir)?;
            }
        }
        self.write(std::fs::File::create(p)?)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventsSnapshot(pub std::vec::Vec<EventSnapshot>);

impl EventsSnapshot {
    // Read in a [`EventsSnapshot`] from a reader.
    pub fn read(r: impl std::io::Read) -> Result<EventsSnapshot, std::io::Error> {
        Ok(serde_json::from_reader::<_, EventsSnapshot>(r)?)
    }

    // Read in a [`EventsSnapshot`] from a file.
    pub fn read_file(p: impl AsRef<std::path::Path>) -> Result<EventsSnapshot, std::io::Error> {
        Self::read(std::fs::File::open(p)?)
    }

    // Write a [`EventsSnapshot`] to a writer.
    pub fn write(&self, w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(serde_json::to_writer_pretty(w, self)?)
    }

    // Write a [`EventsSnapshot`] to file.
    pub fn write_file(&self, p: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        let p = p.as_ref();
        if let Some(dir) = p.parent() {
            if !dir.exists() {
                std::fs::create_dir_all(dir)?;
            }
        }
        self.write(std::fs::File::create(p)?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventSnapshot {
    pub event: xdr::ContractEvent,
    pub failed_call: bool,
}

impl From<crate::env::internal::events::HostEvent> for EventSnapshot {
    fn from(v: crate::env::internal::events::HostEvent) -> Self {
        Self {
            event: v.event,
            failed_call: v.failed_call,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthSnapshot(
    pub std::vec::Vec<std::vec::Vec<(xdr::ScAddress, xdr::SorobanAuthorizedInvocation)>>,
);

impl AuthSnapshot {
    // Read in a [`AuthSnapshot`] from a reader.
    pub fn read(r: impl std::io::Read) -> Result<AuthSnapshot, std::io::Error> {
        Ok(serde_json::from_reader::<_, AuthSnapshot>(r)?)
    }

    // Read in a [`AuthSnapshot`] from a file.
    pub fn read_file(p: impl AsRef<std::path::Path>) -> Result<AuthSnapshot, std::io::Error> {
        Self::read(std::fs::File::open(p)?)
    }

    // Write a [`AuthSnapshot`] to a writer.
    pub fn write(&self, w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(serde_json::to_writer_pretty(w, self)?)
    }

    // Write a [`AuthSnapshot`] to file.
    pub fn write_file(&self, p: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        let p = p.as_ref();
        if let Some(dir) = p.parent() {
            if !dir.exists() {
                std::fs::create_dir_all(dir)?;
            }
        }
        self.write(std::fs::File::create(p)?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Generators {
    address: u64,
    nonce: u64,
}

impl Default for Generators {
    fn default() -> Generators {
        Generators {
            address: 0,
            nonce: 0,
        }
    }
}

impl Generators {
    // Read in a [`Generators`] from a reader.
    pub fn read(r: impl std::io::Read) -> Result<Generators, std::io::Error> {
        Ok(serde_json::from_reader::<_, Generators>(r)?)
    }

    // Read in a [`Generators`] from a file.
    pub fn read_file(p: impl AsRef<std::path::Path>) -> Result<Generators, std::io::Error> {
        Self::read(std::fs::File::open(p)?)
    }

    // Write a [`Generators`] to a writer.
    pub fn write(&self, w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(serde_json::to_writer_pretty(w, self)?)
    }

    // Write a [`Generators`] to file.
    pub fn write_file(&self, p: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        let p = p.as_ref();
        if let Some(dir) = p.parent() {
            if !dir.exists() {
                std::fs::create_dir_all(dir)?;
            }
        }
        self.write(std::fs::File::create(p)?)
    }
}

impl Generators {
    pub fn address(&mut self) -> [u8; 32] {
        self.address = self.address.checked_add(1).unwrap();
        let b: [u8; 8] = self.address.to_be_bytes();
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b[0], b[1],
            b[2], b[3], b[4], b[5], b[6], b[7],
        ]
    }

    pub fn nonce(&mut self) -> i64 {
        self.nonce = self.nonce.checked_add(1).unwrap();
        self.nonce as i64
    }
}

#[doc(hidden)]
pub type ContractFunctionF = dyn Send + Sync + Fn(Env, &[Val]) -> Val;
#[doc(hidden)]
pub trait ContractFunctionRegister {
    fn register(name: &'static str, func: &'static ContractFunctionF);
}
#[doc(hidden)]
pub trait ContractFunctionSet {
    fn call(&self, func: &str, env: Env, args: &[Val]) -> Option<Val>;
}

#[doc(inline)]
pub use crate::env::internal::LedgerInfo;

/// Test utilities for [`Ledger`][crate::ledger::Ledger].
pub trait Ledger {
    /// Set ledger info.
    fn set(&self, l: LedgerInfo);

    /// Sets the protocol version.
    fn set_protocol_version(&self, protocol_version: u32);

    /// Sets the sequence number.
    fn set_sequence_number(&self, sequence_number: u32);

    /// Sets the timestamp.
    fn set_timestamp(&self, timestamp: u64);

    /// Sets the network ID.
    fn set_network_id(&self, network_id: [u8; 32]);

    /// Sets the base reserve.
    fn set_base_reserve(&self, base_reserve: u32);

    /// Sets the minimum temporary entry time-to-live.
    fn set_min_temp_entry_ttl(&self, min_temp_entry_ttl: u32);

    /// Sets the minimum persistent entry time-to-live.
    fn set_min_persistent_entry_ttl(&self, min_persistent_entry_ttl: u32);

    /// Sets the maximum entry time-to-live.
    fn set_max_entry_ttl(&self, max_entry_ttl: u32);

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
    use crate::env::internal::budget::CostTracker;
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
    /// env.cost_estimate().budget().reset_default();
    /// // ...
    /// println!("{}", env.cost_estimate().budget());
    /// # }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub struct Budget(pub(crate) crate::env::internal::budget::Budget);

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
            self.0.reset_default().unwrap();
        }

        pub fn reset_unlimited(&mut self) {
            self.0.reset_unlimited().unwrap();
        }

        pub fn reset_limits(&mut self, cpu: u64, mem: u64) {
            self.0.reset_limits(cpu, mem).unwrap();
        }

        pub fn reset_tracker(&mut self) {
            self.0.reset_tracker().unwrap();
        }

        /// Returns the CPU instruction cost.
        ///
        /// Note that CPU instructions are likely to be underestimated when
        /// running Rust code compared to running the WASM equivalent.
        pub fn cpu_instruction_cost(&self) -> u64 {
            self.0.get_cpu_insns_consumed().unwrap()
        }

        /// Returns the memory cost.
        ///
        /// Note that memory is likely to be underestimated when running Rust
        /// code compared to running the WASM equivalent.
        pub fn memory_bytes_cost(&self) -> u64 {
            self.0.get_mem_bytes_consumed().unwrap()
        }

        /// Get the cost tracker associated with the cost type. The tracker
        /// tracks the cumulative iterations and inputs and derived cpu and
        /// memory. If the underlying model is a constant model, then inputs is
        /// `None` and only iterations matter.
        ///
        /// Note that VM cost types are likely to be underestimated when running
        /// natively as Rust code inside tests code compared to running the WASM
        /// equivalent.
        pub fn tracker(&self, cost_type: ContractCostType) -> CostTracker {
            self.0.get_tracker(cost_type).unwrap()
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
    /// - Event Topics as a [`Vec<Val>`]
    /// - Event Data as a [`Val`]
    fn all(&self) -> Vec<(crate::Address, Vec<Val>, Val)>;
}

/// Test utilities for [`Logs`][crate::logs::Logs].
pub trait Logs {
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
    /// Generate a new Address.
    ///
    /// Implementation note: this always builds the contract addresses now. This
    /// shouldn't normally matter though, as contracts should be agnostic to
    /// the underlying Address value.
    fn generate(env: &Env) -> crate::Address;
}

pub trait Deployer {
    /// Gets the TTL of the given contract's instance.
    ///
    /// TTL is the number of ledgers left until the instance entry is considered
    /// expired, excluding the current ledger.
    ///
    /// Panics if there is no instance corresponding to the provided address,
    /// or if the instance has expired.
    fn get_contract_instance_ttl(&self, contract: &crate::Address) -> u32;

    /// Gets the TTL of the given contract's Wasm code entry.
    ///
    /// TTL is the number of ledgers left until the contract code entry
    /// is considered expired, excluding the current ledger.
    ///
    /// Panics if there is no contract instance/code corresponding to
    /// the provided address, or if the instance/code has expired.
    fn get_contract_code_ttl(&self, contract: &crate::Address) -> u32;
}

pub use xdr::AccountFlags as IssuerFlags;

#[derive(Clone)]
pub struct StellarAssetIssuer {
    env: Env,
    account_id: xdr::AccountId,
}

impl StellarAssetIssuer {
    pub(crate) fn new(env: Env, account_id: xdr::AccountId) -> Self {
        Self { env, account_id }
    }

    /// Returns the flags for the issuer.
    pub fn flags(&self) -> u32 {
        self.env
            .host()
            .with_mut_storage(|storage| {
                let k = Rc::new(xdr::LedgerKey::Account(xdr::LedgerKeyAccount {
                    account_id: self.account_id.clone(),
                }));

                let entry = storage.get(
                    &k,
                    soroban_env_host::budget::AsBudget::as_budget(self.env.host()),
                )?;

                match entry.data {
                    xdr::LedgerEntryData::Account(ref e) => Ok(e.flags.clone()),
                    _ => panic!("expected account entry but got {:?}", entry.data),
                }
            })
            .unwrap()
    }

    /// Adds the flag specified to the existing issuer flags
    pub fn set_flag(&self, flag: IssuerFlags) {
        self.overwrite_issuer_flags(self.flags() | (flag as u32))
    }

    /// Clears the flag specified from the existing issuer flags
    pub fn clear_flag(&self, flag: IssuerFlags) {
        self.overwrite_issuer_flags(self.flags() & (!(flag as u32)))
    }

    pub fn address(&self) -> crate::Address {
        xdr::ScAddress::Account(self.account_id.clone())
            .try_into_val(&self.env.clone())
            .unwrap()
    }

    /// Sets the issuer flags field.
    /// Each flag is a bit with values corresponding to [xdr::AccountFlags]
    ///
    /// Use this to test interactions between trustlines/balances and the issuer flags.
    fn overwrite_issuer_flags(&self, flags: u32) {
        if u64::from(flags) > xdr::MASK_ACCOUNT_FLAGS_V17 {
            panic!(
                "issuer flags value must be at most {}",
                xdr::MASK_ACCOUNT_FLAGS_V17
            );
        }

        self.env
            .host()
            .with_mut_storage(|storage| {
                let k = Rc::new(xdr::LedgerKey::Account(xdr::LedgerKeyAccount {
                    account_id: self.account_id.clone(),
                }));

                let mut entry = storage
                    .get(
                        &k,
                        soroban_env_host::budget::AsBudget::as_budget(self.env.host()),
                    )?
                    .as_ref()
                    .clone();

                match entry.data {
                    xdr::LedgerEntryData::Account(ref mut e) => e.flags = flags,
                    _ => panic!("expected account entry but got {:?}", entry.data),
                }

                storage.put(
                    &k,
                    &Rc::new(entry),
                    None,
                    soroban_env_host::budget::AsBudget::as_budget(self.env.host()),
                )?;
                Ok(())
            })
            .unwrap();
    }
}

pub struct StellarAssetContract {
    address: crate::Address,
    issuer: StellarAssetIssuer,
}

impl StellarAssetContract {
    pub(crate) fn new(address: crate::Address, issuer: StellarAssetIssuer) -> Self {
        Self { address, issuer }
    }

    pub fn address(&self) -> crate::Address {
        self.address.clone()
    }

    pub fn issuer(&self) -> StellarAssetIssuer {
        self.issuer.clone()
    }
}
