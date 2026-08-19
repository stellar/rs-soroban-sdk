//! Executable refs contains functions for managing the executable reference
//! entries owned by the currently executing contract.

use crate::{env::internal, unwrap::UnwrapInfallible, BytesN, Env, String, Val};

#[cfg(doc)]
use crate::auth::ContractExecutable;

#[cfg(doc)]
use crate::deploy::{Deployer, DeployerWithAddress};

/// ExecutableRefs manages the executable reference entries owned by the
/// current contract.
///
/// An executable reference entry is a persistent contract data entry, keyed by
/// a tag, containing a Wasm hash. Contracts can use any contract's executable
/// reference entry as their own executable. Contracts that have an executable
/// reference as their executable will have their contract code loaded from the
/// executable reference's Wasm hash when invoked. Updating the entry to point at
/// a new Wasm hash causes the new Wasm to be used by all contracts that use the
/// entry as their executable.
///
/// Contracts use an executable reference entry as their executable by being
/// deployed with [DeployerWithAddress::deploy_contract], or by replacing
/// their own executable with [Deployer::update_current_contract], passing a
/// [ContractExecutable::ExternalRef].
///
/// Entries are stored in the owning contract's persistent storage under a
/// protocol-defined key type, `ExecutableTag`. They do not collide with
/// ordinary storage keys, including [String] keys with the same value.
///
/// Callers can construct `ExecutableTag` keys off-chain and pass them
/// to the contract as a [Val], which can collide with these entries if
/// used as a persistent storage key.
///
/// The protocol enforces rules on the entries, which is why they are managed
/// with these functions rather than the regular storage functions:
///
/// - The value must be the 32-byte hash of Wasm that has already been
///   uploaded. See [Deployer::upload_contract_wasm].
/// - The entries always have persistent durability.
/// - Once created, an entry can never be removed. Like any persistent entry
///   it can be archived if its TTL expires, and restored.
pub struct ExecutableRefs {
    env: Env,
}

impl ExecutableRefs {
    pub(crate) fn new(env: &Env) -> ExecutableRefs {
        ExecutableRefs { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Set the executable reference entry keyed by `tag` to point at
    /// `wasm_hash`, creating the entry if it does not exist.
    ///
    /// If the entry already exists, all contracts using the entry as their
    /// executable will use the new `wasm_hash` as their code at their
    /// next invocation.
    ///
    /// Once created, the entry can never be removed.
    ///
    /// ### Panics
    ///
    /// If `wasm_hash` is not the hash of Wasm that has already been uploaded.
    /// See [Deployer::upload_contract_wasm].
    pub fn set(&self, tag: &String, wasm_hash: &BytesN<32>) {
        let key = self.key(tag);
        self.env.storage().persistent().set(&key, wasm_hash);
    }

    /// Get the Wasm hash the executable reference entry keyed by `tag` points
    /// at, or None if the entry does not exist.
    pub fn get(&self, tag: &String) -> Option<BytesN<32>> {
        let key = self.key(tag);
        self.env.storage().persistent().get(&key)
    }

    /// Returns true if the executable reference entry keyed by `tag` exists.
    pub fn has(&self, tag: &String) -> bool {
        let key = self.key(tag);
        self.env.storage().persistent().has(&key)
    }

    /// Extend the TTL of the executable reference entry keyed by `tag`.
    ///
    /// Extends the TTL only if the TTL of the entry is below `threshold`
    /// ledgers. The TTL will then become `extend_to` ledgers.
    ///
    /// The TTL is the number of ledgers between the current ledger and the
    /// final ledger the entry can still be accessed.
    ///
    /// ### Panics
    ///
    /// If the entry does not exist.
    pub fn extend_ttl(&self, tag: &String, threshold: u32, extend_to: u32) {
        let key = self.key(tag);
        self.env
            .storage()
            .persistent()
            .extend_ttl(&key, threshold, extend_to);
    }

    /// Extend the TTL of the executable reference entry keyed by `tag`, with
    /// limits on the extension.
    ///
    /// Extends the TTL of the entry to be up to `extend_to` ledgers. The
    /// extension only happens if it exceeds `min_extension` ledgers, otherwise
    /// this is a no-op. The amount of extension will not exceed
    /// `max_extension` ledgers.
    ///
    /// The TTL is the number of ledgers between the current ledger and the
    /// final ledger the entry can still be accessed.
    ///
    /// ### Panics
    ///
    /// If the entry does not exist.
    pub fn extend_ttl_with_limits(
        &self,
        tag: &String,
        extend_to: u32,
        min_extension: u32,
        max_extension: u32,
    ) {
        let key = self.key(tag);
        self.env.storage().persistent().extend_ttl_with_limits(
            &key,
            extend_to,
            min_extension,
            max_extension,
        );
    }

    /// Returns the TTL of the executable reference entry keyed by `tag`.
    ///
    /// The TTL is the number of ledgers between the current ledger and the
    /// final ledger the entry can still be accessed.
    ///
    /// ### Panics
    ///
    /// If the entry does not exist.
    #[cfg(any(test, feature = "testutils"))]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
    pub fn get_ttl(&self, tag: &String) -> u32 {
        use crate::testutils::storage::Persistent as _;
        let key = self.key(tag);
        self.env.storage().persistent().get_ttl(&key)
    }

    /// Build the protocol-defined storage key for the entry keyed by `tag`.
    fn key(&self, tag: &String) -> Val {
        internal::Env::create_executable_tag(&self.env, tag.to_object())
            .unwrap_infallible()
            .to_val()
    }
}
