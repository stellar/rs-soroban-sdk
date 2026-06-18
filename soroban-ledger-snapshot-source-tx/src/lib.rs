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
fn xdr_schema_version() -> &'static str {
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
        let workspace_root = WORKSPACE_ROOT.get_or_init(|| {
            MetadataCommand::new()
                .exec()
                .expect("failed to get cargo metadata")
                .workspace_root
                .into()
        });
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
    use super::ttl_for_key;
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
}
