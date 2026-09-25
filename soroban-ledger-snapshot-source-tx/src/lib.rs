//! Fork tests against real Stellar network state, at the granularity of a
//! single transaction.
//!
//! `TxSnapshotSource` is a `SnapshotSource` for the Soroban SDK's test
//! `Env`. Point it at a ledger, and optionally a transaction in that ledger,
//! and the contracts under test read the state the network had at that point.
//! Entries are fetched on demand as the test touches them, so there is no
//! footprint to work out ahead of time and no full snapshot to download.
//!
//! ### Experimental
//!
//! **This crate is an experimental implementation and is subject to breaking
//! change at any time, in any release, including patch releases.** Its API,
//! its on-disk cache formats, and the data sources it reads from may all
//! change or be removed without a deprecation period or a major version bump.
//! It is intended for tests, not for production code.
//!
//! ### Usage
//!
//! Add it as a dev-dependency, alongside `soroban-sdk` with the `testutils`
//! feature enabled:
//!
//! ```toml
//! [dev-dependencies]
//! soroban-ledger-snapshot-source-tx = "28"
//! soroban-sdk = { version = "28", features = ["testutils"] }
//! ```
//!
//! ```rust,no_run
//! use soroban_ledger_snapshot_source_tx::{Network, TxSnapshotSource};
//! use soroban_sdk::{token::TokenClient, Address, Env};
//!
//! // The state just before this transaction executed, in this ledger.
//! let tx: [u8; 32] = hex::decode("201a1e9cd0d48ed5e7facc7642785a0621f75a50c2c8fc06d1e936a82b642312")
//!     .unwrap()
//!     .try_into()
//!     .unwrap();
//! let source = TxSnapshotSource::new(Network::mainnet(None), 61340000, Some(tx));
//! let env = Env::from_ledger_snapshot(source);
//!
//! // Read that state through any contract, here the native asset contract.
//! let contract = Address::from_str(&env, "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA");
//! let client = TokenClient::new(&env, &contract);
//! let balance = client.balance(&Address::from_str(
//!     &env,
//!     "GCO45COWIBDZEGJ3DRGDGCCCJXK777F2S6D6HXQKXVB3EKQVCQU7B2WA",
//! ));
//! ```
//!
//! ### Point in time
//!
//! The ledger sequence passed to `TxSnapshotSource::new` means the *start* of
//! that ledger, and the transaction hash, when given, narrows that to the point
//! just before that transaction executed. Passing `None` for the transaction
//! gives the state at the *end* of the ledger, after every transaction in it.
//!
//! So to debug what a transaction did, fork at its ledger and its hash, and the
//! test sees exactly what that transaction saw.
//!
//! ### Networks
//!
//! `Network` carries the URLs the entries are fetched from. There are
//! constructors for the public networks:
//!
//! - `Network::mainnet` takes an optional RPC URL.
//! - `Network::testnet` takes the start date of the testnet epoch to read,
//!   because testnet is reset periodically and its meta is partitioned by
//!   epoch, so there is no stable default.
//! - `Network::local` reads from a local [stellar/quickstart] network run
//!   with `--enable rpc,galexie`.
//!
//! [stellar/quickstart]: https://github.com/stellar/quickstart
//!
//! Any field can be overridden, or the struct built directly, to read from
//! other infrastructure:
//!
//! ```rust
//! use soroban_ledger_snapshot_source_tx::Network;
//!
//! // Mainnet, but reading meta from somewhere else and never asking an RPC.
//! let network = Network {
//!     meta_url: "https://meta.example.com/pubnet".to_string(),
//!     rpc_url: None,
//!     ..Network::mainnet(None)
//! };
//! ```
//!
//! ### Where entries come from
//!
//! For each entry a test reads, the sources are tried in the order below, and
//! the first that answers wins. Each is slower than the one before it, so the
//! ordering is what keeps a fork fast:
//!
//! 1. The meta of the transactions already looked at in the queried ledger.
//! 2. The meta of the transactions before it in the same ledger.
//! 3. The RPC, if `rpc_url` is set, and only when what it returns is known to
//!    be correct for the queried ledger.
//! 4. The meta of preceding ledgers, back as far as the last checkpoint.
//! 5. The history archive checkpoint covering the ledger.
//!
//! An RPC is an optimisation rather than a requirement: it answers in one
//! request what would otherwise be a walk back through meta and archives. With
//! `rpc_url` set to `None` every entry is resolved from meta and archives
//! alone.
//!
//! ### Caching
//!
//! Fetching is expensive, so results are cached in two places:
//!
//! - `<workspace-root>/tests-snapshot-source/<network>/`, holding the ledger
//!   entries the tests in this workspace have read. **Commit this directory.**
//!   With it present the tests replay from disk, so CI reproduces the run
//!   without reaching the network at all.
//! - The system cache directory (`~/.cache/soroban-sdk/snapshot-source-tx/` on
//!   Linux), holding both the found entries and the raw files downloaded to
//!   find them. It is shared across workspaces and is safe to delete.
//!
//! The workspace root is located with `cargo metadata`, so tests must be run
//! from within a cargo workspace.
//!
//! Cached entries record the stellar-xdr schema they were written with, and a
//! test fails rather than reading entries written against a different schema.
//! If that happens, delete `tests-snapshot-source` and re-run to regenerate it.
//!
//! ### Logging
//!
//! Set `RUST_LOG` to see which source each entry was found in, which is the
//! quickest way to tell why a fork is slow or returning state you did not
//! expect:
//!
//! ```sh
//! RUST_LOG=soroban_ledger_snapshot_source_tx=debug cargo test -- --nocapture
//! ```

mod cache;
mod fetch;

use cache::cache;
use cargo_metadata::MetadataCommand;
use directories::ProjectDirs;
use fetch::LedgerEntryFetcher;
pub use fetch::Network;
use sha2::{Digest, Sha256};
use soroban_sdk::testutils::SnapshotSourceInput;
use soroban_sdk::testutils::{HostError, SnapshotSource};
use soroban_sdk::xdr::{LedgerEntry, LedgerKey, Limits, WriteXdr};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Once, OnceLock};

static TRACING_INIT: Once = Once::new();
static WORKSPACE_ROOT: OnceLock<PathBuf> = OnceLock::new();

/// Initialize tracing subscriber if RUST_LOG environment variable is set.
/// This is called automatically when creating a TxSnapshotSource.
fn init_tracing() {
    TRACING_INIT.call_once(|| {
        if std::env::var("RUST_LOG").is_ok() {
            tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_writer(std::io::stderr)
                .fmt_fields(tracing_subscriber::fmt::format::PrettyFields::new())
                .init();
        }
    });
}

/// The stellar-xdr schema version that cached fixtures are serialized against.
///
/// Committed snapshot fixtures store ledger entries using stellar-xdr's serde
/// JSON representation. If that representation ever changes (a field rename,
/// add/remove, or enum-variant shift), fixtures produced by an older schema
/// would silently deserialize into wrong data — or panic — with no indication
/// of why. Embedding this fingerprint and rejecting mismatches turns that into
/// a clear, actionable failure instead.
///
/// Also reused by the machine-local history-result cache in the `fetch`
/// module, so both caches invalidate together on an XDR schema change.
pub(crate) fn xdr_schema_version() -> &'static str {
    stellar_xdr::VERSION.xdr
}

/// On-disk wrapper for a cached ledger-entry fixture, tagging the serialized
/// entry with the XDR schema version it was produced with.
#[derive(serde::Serialize, serde::Deserialize)]
struct CachedEntry {
    /// stellar-xdr schema version (see [`xdr_schema_version`]).
    xdr_schema_version: String,
    /// The cached ledger entry, or `None` if the entry was absent/removed.
    entry: Option<LedgerEntry>,
}

fn cache_paths(
    workspace_root: &Path,
    system_cache_root: &Path,
    network: &Network,
) -> (PathBuf, PathBuf) {
    let network_name = network.name();
    (
        workspace_root
            .join("tests-snapshot-source")
            .join(&network_name),
        system_cache_root
            .join("snapshot-source-tx")
            .join(network_name),
    )
}

/// Snapshot source that downloads ledger meta and searches for ledger entries
/// based on a specific transaction context.
pub struct TxSnapshotSource {
    fetcher: LedgerEntryFetcher,
    tx_hash: Option<[u8; 32]>,
    cache_path: PathBuf,
}

impl TxSnapshotSource {
    /// Create a new TxSnapshotSource
    ///
    /// The cache path is automatically computed as
    /// `<workspace_root>/tests-snapshot-source/<network_name>`
    /// using cargo metadata to find the workspace root.
    ///
    /// # Arguments
    /// * `network` - Network configuration with URLs for meta storage, RPC, and history archive
    /// * `ledger` - Ledger sequence number
    /// * `tx_hash` - Optional transaction hash
    ///
    /// # Panics
    /// Panics if the workspace root cannot be determined via cargo metadata.
    pub fn new(network: Network, ledger: u32, tx_hash: Option<[u8; 32]>) -> Self {
        init_tracing();
        let workspace_root = WORKSPACE_ROOT.get_or_init(|| {
            MetadataCommand::new()
                .exec()
                .expect("failed to get cargo metadata")
                .workspace_root
                .into()
        });
        let project_dirs = ProjectDirs::from("org", "stellar", "soroban-sdk")
            .expect("failed to get project directories")
            .cache_dir()
            .to_path_buf();
        let (cache_path, fetcher_cache_path) = cache_paths(workspace_root, &project_dirs, &network);
        Self {
            fetcher: LedgerEntryFetcher::new(network, ledger, tx_hash, fetcher_cache_path),
            tx_hash,
            cache_path,
        }
    }

    /// Fetch a ledger entry, using workspace-level caching
    fn fetch(&self, key: &LedgerKey) -> Option<LedgerEntry> {
        // Compute cache file path: <cache_path>/<ledger>/<tx_hash_or_none>/<hash_of_key>.json
        let key_xdr = key.to_xdr(Limits::none()).expect("failed to encode key");
        let key_hash = Sha256::digest(&key_xdr);
        let ledger_cache_dir = self.cache_path.join(
            self.tx_hash
                .map(|h| {
                    let tx_hash_str: String = h.iter().map(|b| format!("{b:02x}")).collect();
                    format!("{}-{}-before", self.fetcher.ledger(), tx_hash_str)
                })
                .unwrap_or_else(|| format!("{}-after", self.fetcher.ledger())),
        );

        // Use cache function to handle reading/writing cache file
        let fetch_read = cache(
            ledger_cache_dir.join(format!("{:x}.json", key_hash)),
            |write| -> Result<(), Box<dyn std::error::Error>> {
                // Fetch the data from the underlying fetcher
                let entry = self.fetcher.fetch(key)?;

                // Serialize to JSON, tagged with the XDR schema version.
                let cached = CachedEntry {
                    xdr_schema_version: xdr_schema_version().to_string(),
                    entry,
                };
                serde_json::to_writer_pretty(write, &cached)?;

                Ok(())
            },
        )
        .expect("failed to cache entry");

        // Parse the cached result, rejecting fixtures produced by a different
        // XDR schema (which would otherwise deserialize into wrong data).
        let cached: CachedEntry =
            serde_json::from_reader(fetch_read).expect("failed to parse cached entry");
        assert_eq!(
            cached.xdr_schema_version,
            xdr_schema_version(),
            "cached snapshot {:x}.json was produced with stellar-xdr schema {} \
             but this build uses {}; delete the tests-snapshot-source directory \
             and re-run to regenerate the fixtures",
            key_hash,
            cached.xdr_schema_version,
            xdr_schema_version(),
        );
        cached.entry
    }
}

impl From<TxSnapshotSource> for SnapshotSourceInput {
    fn from(source: TxSnapshotSource) -> Self {
        Self {
            source: Rc::new(source),
            ledger_info: None,
            snapshot: None,
        }
    }
}

impl SnapshotSource for TxSnapshotSource {
    fn get(
        &self,
        key: &Rc<LedgerKey>,
    ) -> Result<Option<(Rc<LedgerEntry>, Option<u32>)>, HostError> {
        Ok(self.fetch(key).map(|e| (Rc::new(e), ttl_for_key(key))))
    }
}

/// Returns a TTL to use for a ledger entry in tests based on its key type.
///
/// Contract code and contract data entries get `u32::MAX` TTL (effectively
/// never expire), while other entry types get `None` because they do not
/// support TTLs.
fn ttl_for_key(key: &LedgerKey) -> Option<u32> {
    match key {
        LedgerKey::ContractCode(_) | LedgerKey::ContractData(_) => Some(u32::MAX),
        _ => None,
    }
}

#[cfg(test)]
mod test_ttl {
    use super::{cache_paths, ttl_for_key, Network};
    use soroban_sdk::xdr::{
        AccountId, ContractDataDurability, ContractId, Hash, LedgerKey, LedgerKeyAccount,
        LedgerKeyContractCode, LedgerKeyContractData, LedgerKeyTtl, PublicKey, ScAddress, ScVal,
        Uint256,
    };

    #[test]
    fn contract_code_and_data_get_max_ttl() {
        let code = LedgerKey::ContractCode(LedgerKeyContractCode {
            hash: Hash([0u8; 32]),
        });
        assert_eq!(ttl_for_key(&code), Some(u32::MAX));

        let data = LedgerKey::ContractData(LedgerKeyContractData {
            contract: ScAddress::Contract(ContractId(Hash([0u8; 32]))),
            key: ScVal::I32(0),
            durability: ContractDataDurability::Persistent,
        });
        assert_eq!(ttl_for_key(&data), Some(u32::MAX));
    }

    #[test]
    fn non_contract_keys_have_no_ttl() {
        let account = LedgerKey::Account(LedgerKeyAccount {
            account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([0u8; 32]))),
        });
        assert_eq!(ttl_for_key(&account), None);

        let ttl = LedgerKey::Ttl(LedgerKeyTtl {
            key_hash: Hash([0u8; 32]),
        });
        assert_eq!(ttl_for_key(&ttl), None);
    }

    #[test]
    fn cache_paths_are_namespaced_by_network_name() {
        let workspace = std::path::Path::new("/workspace");
        let system_cache = std::path::Path::new("/cache");
        let mainnet = Network::mainnet(None);
        let testnet = Network::testnet("2025-12-17".to_string());

        let mainnet_paths = cache_paths(workspace, system_cache, &mainnet);
        let testnet_paths = cache_paths(workspace, system_cache, &testnet);

        assert_ne!(mainnet_paths, testnet_paths);
        assert_eq!(
            mainnet_paths.0,
            workspace.join("tests-snapshot-source/mainnet")
        );
        assert_eq!(
            mainnet_paths.1,
            system_cache.join("snapshot-source-tx/mainnet")
        );
        assert_eq!(
            testnet_paths.0,
            workspace.join("tests-snapshot-source/testnet-2025-12-17")
        );
        assert_eq!(
            testnet_paths.1,
            system_cache.join("snapshot-source-tx/testnet-2025-12-17")
        );
    }

    #[test]
    fn cache_paths_use_safe_network_name() {
        let mut network = Network::mainnet(None);
        network.name = "../my mainnet!".to_string();
        let paths = cache_paths(
            std::path::Path::new("/workspace"),
            std::path::Path::new("/cache"),
            &network,
        );
        assert_eq!(
            paths.0,
            std::path::Path::new("/workspace/tests-snapshot-source/mymainnet")
        );
        assert_eq!(
            paths.1,
            std::path::Path::new("/cache/snapshot-source-tx/mymainnet")
        );
    }
}
