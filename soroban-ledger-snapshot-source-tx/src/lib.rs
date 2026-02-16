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
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Once;

static TRACING_INIT: Once = Once::new();

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
    /// The cache path is automatically computed as `<workspace_root>/tests-snapshot-source/<network_id_hex>`
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
        let workspace_root: PathBuf = MetadataCommand::new()
            .exec()
            .expect("failed to get cargo metadata")
            .workspace_root
            .into();
        let cache_path = workspace_root
            .join("tests-snapshot-source")
            .join(network.network_id_hex());
        let fetcher_cache_path = ProjectDirs::from("org", "stellar", "soroban-sdk")
            .expect("failed to get project directories")
            .cache_dir()
            .join("snapshot-source-tx");
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

        // Ensure cache directory exists
        std::fs::create_dir_all(&ledger_cache_dir).expect("failed to create cache directory");

        // Use cache function to handle reading/writing cache file
        let fetch_read = cache(
            ledger_cache_dir.join(format!("{:x}.json", key_hash)),
            |write| -> Result<(), Box<dyn std::error::Error>> {
                // Fetch the data from the underlying fetcher
                let result = self.fetcher.fetch(key)?;

                // Serialize to JSON
                serde_json::to_writer_pretty(write, &result)?;

                Ok(())
            },
        )
        .expect("failed to cache entry");

        // Parse the cached result
        serde_json::from_reader(fetch_read).expect("failed to parse cached entry")
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
