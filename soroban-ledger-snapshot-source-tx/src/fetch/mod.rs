use crate::cache::{cache, CacheError};
use from_meta_storage::{get_config, get_ledger, parse_config, parse_ledger};
use from_rpc::{get_ledger_entry, parse_ledger_entry};
use sha2::{Digest, Sha256};
use soroban_sdk::xdr::{LedgerEntry, LedgerKey, Limits, WriteXdr};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::thread::JoinHandle;

mod iter;
pub use iter::LedgerEntryChangesIterator;

pub(crate) mod from_meta_storage;
pub(crate) mod from_rpc;

/// Error type for LedgerEntryFetcher operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cache error: {0}")]
    Cache(#[from] CacheError<Box<dyn std::error::Error>>),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("xdr error: {0}")]
    Xdr(#[from] soroban_sdk::xdr::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("meta storage error: {0}")]
    MetaStorage(#[from] from_meta_storage::Error),
    #[error("rpc error: {0}")]
    Rpc(#[from] from_rpc::Error),
}

/// Network configuration for fetching ledger data
///
/// Contains URLs for SEP-54 meta storage and RPC, as well as the checkpoint
/// frequency for the network.
#[derive(Debug, Clone)]
pub struct Network {
    /// Human-readable name used to namespace caches.
    ///
    /// Should uniquely identify the network epoch. See [`Network::name`] for
    /// how this value is made filesystem-safe.
    pub name: String,
    /// Network passphrase (e.g., "Public Global Stellar Network ; September 2015")
    pub passphrase: String,
    /// URL to the SEP-54 ledger meta storage
    pub meta_url: String,
    /// URL to an RPC that supports the `ledger` parameter of
    /// `getLedgerEntries`, and that holds the checkpoint ledgers being read.
    pub rpc_url: String,
    /// Number of ledgers between checkpoints. The meta of the ledgers back to
    /// the previous checkpoint is searched before the RPC is asked for the
    /// state at that checkpoint.
    pub archive_checkpoint_ledger_count: u32,
}

impl Network {
    /// Create a Network configuration for Stellar mainnet with default URLs
    ///
    /// Uses default mainnet URLs:
    /// - SEP-54 meta storage: AWS public blockchain
    ///
    /// # Arguments
    /// * `rpc_url` - URL to an RPC that supports the `ledger` parameter of
    ///   `getLedgerEntries`
    pub fn mainnet(rpc_url: String) -> Self {
        Self {
            name: "mainnet".to_string(),
            passphrase: "Public Global Stellar Network ; September 2015".to_string(),
            meta_url: "https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/pubnet".to_string(),
            rpc_url,
            archive_checkpoint_ledger_count: 64,
        }
    }

    /// Create a Network configuration for Stellar testnet with default URLs
    ///
    /// Uses default testnet URLs:
    /// - RPC: soroban-testnet.stellar.org
    ///
    /// # Arguments
    /// * `testnet_start_date` - The reset-epoch start date, formatted
    ///   `YYYY-MM-DD`, identifying the testnet meta partition on the AWS public
    ///   dataset (e.g. `2025-12-17`). Testnet is periodically reset and the meta
    ///   is partitioned by epoch, so there is no stable default; the caller must
    ///   supply the start date for the epoch containing the ledgers under test.
    pub fn testnet(testnet_start_date: String) -> Self {
        Self {
            name: format!("testnet-{testnet_start_date}"),
            passphrase: "Test SDF Network ; September 2015".to_string(),
            meta_url: format!(
                "https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/testnet/{testnet_start_date}"
            ),
            rpc_url: "https://soroban-testnet.stellar.org".to_string(),
            archive_checkpoint_ledger_count: 64,
        }
    }

    /// Create a Network configuration for local Stellar Quickstart
    ///
    /// Uses default quickstart URLs:
    /// - SEP-54 meta storage: localhost:8000/ledger-meta
    /// - RPC: localhost:8000/rpc
    pub fn local() -> Self {
        Self {
            name: "local".to_string(),
            passphrase: "Standalone Network ; February 2017".to_string(),
            meta_url: "http://localhost:8000/ledger-meta".to_string(),
            rpc_url: "http://localhost:8000/rpc".to_string(),
            archive_checkpoint_ledger_count: 8,
        }
    }

    /// Returns a filesystem-safe network name for cache paths.
    ///
    /// Characters other than ASCII letters, digits, `-`, and `_` are removed.
    /// If no characters remain, the network ID is returned instead.
    pub fn name(&self) -> String {
        let name: String = self
            .name
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || matches!(*c, '-' | '_'))
            .collect();
        if name.is_empty() {
            self.network_id_hex()
        } else {
            name
        }
    }

    /// Returns the network ID, which is the SHA256 hash of the network passphrase.
    pub fn network_id(&self) -> [u8; 32] {
        Sha256::digest(self.passphrase.as_bytes()).into()
    }

    /// Returns the network ID as a hex-encoded string.
    pub fn network_id_hex(&self) -> String {
        hex::encode(self.network_id())
    }
}

/// Compute the bounds of the per-ledger meta search for a target ledger.
///
/// Returns `(prev_checkpoint, ledgers_to_checkpoint)` where `prev_checkpoint`
/// is the checkpoint ledger at or below `ledger` (checkpoints fall on
/// `k * count - 1`), and `ledgers_to_checkpoint` is how many ledgers back the
/// meta search walks before falling back to the RPC.
///
/// Saturating arithmetic is used so that ledgers in the first checkpoint
/// window (`ledger < count`), where no previous checkpoint exists, degrade to
/// a search down to ledger 0 instead of underflowing `u32` (which would panic
/// under the crate's `overflow-checks = true`).
fn checkpoint_search_bounds(ledger: u32, count: u32) -> (u32, u32) {
    let prev_checkpoint = ((ledger / count) * count).saturating_sub(1);
    let ledgers_to_checkpoint = ledger.saturating_sub(prev_checkpoint);
    (prev_checkpoint, ledgers_to_checkpoint)
}

/// Write an RPC response to the cache, if it is a result rather than an error.
///
/// An error, such as the ledger being outside what the RPC holds, is returned
/// instead of written, so it is never replayed from the cache.
fn write_rpc_response<W: std::io::Write + ?Sized>(
    response: &[u8],
    write: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    parse_ledger_entry(Cursor::new(response))?;
    write.write_all(response)?;
    Ok(())
}

/// Fetcher for ledger entries that downloads ledger meta and searches for entries
pub struct LedgerEntryFetcher {
    network: Network,
    ledger: u32,
    tx_hash: Option<[u8; 32]>,
    cache_path: PathBuf,
    prefetch_handle: Mutex<Option<JoinHandle<()>>>,
}

impl LedgerEntryFetcher {
    /// Create a new LedgerEntryFetcher
    ///
    /// # Arguments
    /// * `network` - Network configuration with URLs for meta storage and RPC
    /// * `ledger` - Ledger sequence number
    /// * `tx_hash` - Optional transaction hash
    /// * `cache_path` - Path to store cache files
    pub fn new(
        network: Network,
        ledger: u32,
        tx_hash: Option<[u8; 32]>,
        cache_path: PathBuf,
    ) -> Self {
        Self {
            network,
            ledger,
            tx_hash,
            cache_path,
            prefetch_handle: Mutex::new(None),
        }
    }

    /// Returns the ledger sequence number this fetcher is configured for.
    pub fn ledger(&self) -> u32 {
        self.ledger
    }

    /// Fetch a ledger entry by key
    ///
    /// This method uses several layers of caching to the system cache directory to avoid refetching entries.
    pub fn fetch(&self, key: &LedgerKey) -> Result<Option<LedgerEntry>, Error> {
        // Serialization here is purely for logging; never let a logging failure
        // change the outcome of the fetch, so use `unwrap_or_default` (which
        // yields JSON null) rather than `?`.
        tracing::info!(key = %serde_json::to_value(key).unwrap_or_default(), "fetch");
        let result = self.fetch_with_entry_cache(key);
        if let Ok(entry) = &result {
            match entry {
                Some(_) => {
                    tracing::info!(entry = %serde_json::to_value(entry).unwrap_or_default(), "found")
                }
                None => tracing::info!("not found"),
            }
        }
        result
    }

    fn fetch_with_entry_cache(&self, key: &LedgerKey) -> Result<Option<LedgerEntry>, Error> {
        let cache_path = &self.cache_path;

        // Compute cache file path: <cache_path>/<ledger>-<tx_hash>-after/<hash_of_key>.json
        // or <cache_path>/<ledger>-after/<hash_of_key>.json if no tx_hash
        let key_xdr = key.to_xdr(Limits::none())?;
        let key_hash = Sha256::digest(&key_xdr);
        let ledger_cache_dir = cache_path.join(
            self.tx_hash
                .map(|h| {
                    let tx_hash_str: String = h.iter().map(|b| format!("{b:02x}")).collect();
                    format!("{}-{}-before", self.ledger, tx_hash_str)
                })
                .unwrap_or_else(|| format!("{}-after", self.ledger)),
        );

        // Use cache function to handle reading/writing cache file
        let fetch_read = cache(
            ledger_cache_dir.join(format!("{:x}.json", key_hash)),
            |write| {
                // Fetch the data
                let result = self.fetch_with_dl_cache(key, &cache_path)?;

                // Serialize to JSON
                serde_json::to_writer_pretty(write, &result)?;

                Ok(())
            },
        )?;

        // Parse the cached result
        Ok(serde_json::from_reader(fetch_read)?)
    }

    fn fetch_with_dl_cache(
        &self,
        key: &LedgerKey,
        cache_path: &Path,
    ) -> Result<Option<LedgerEntry>, Error> {
        // Read the SEP-54 storage layout (cached) so meta object paths are
        // derived from the deployment's actual partition/batch sizes rather than
        // hardcoded constants.
        let (ledgers_per_batch, batches_per_partition) = self.meta_config(cache_path)?;

        // Calculate checkpoint boundaries
        let checkpoint_count = self.network.archive_checkpoint_ledger_count;
        let (prev_checkpoint, ledgers_to_checkpoint) =
            checkpoint_search_bounds(self.ledger, checkpoint_count);

        // Prefetch all meta for ledgers from starting ledger down to the checkpoint.
        let prefetch_ledgers: Vec<u32> = (0..ledgers_to_checkpoint)
            .filter_map(|i| self.ledger.checked_sub(i))
            .collect();
        tracing::debug!(
            count = prefetch_ledgers.len(),
            first = prefetch_ledgers.first(),
            last = prefetch_ledgers.last(),
            "fetch from meta range"
        );
        self.start_meta_prefetch(
            cache_path,
            prefetch_ledgers,
            ledgers_per_batch,
            batches_per_partition,
        );

        // Phase 1: Check the starting ledger
        if let Some(result) = self.fetch_from_meta(
            cache_path,
            self.ledger,
            key,
            ledgers_per_batch,
            batches_per_partition,
        )? {
            return Ok(result);
        }

        // Phase 2: Search through previous ledgers down to the previous checkpoint
        for ledger in (prev_checkpoint + 1..self.ledger).rev() {
            if let Some(result) = self.fetch_from_meta(
                cache_path,
                ledger,
                key,
                ledgers_per_batch,
                batches_per_partition,
            )? {
                return Ok(result);
            }
        }

        // Phase 3: Fetch from the RPC as of the previous checkpoint. In the
        // first checkpoint window there is no ledger before the ones searched,
        // so an entry not found by here did not exist.
        if prev_checkpoint == 0 {
            return Ok(None);
        }
        self.fetch_from_rpc(cache_path, prev_checkpoint, key)
    }

    fn start_meta_prefetch(
        &self,
        cache_path: &Path,
        ledgers: Vec<u32>,
        ledgers_per_batch: u32,
        batches_per_partition: u32,
    ) {
        let mut prefetch_handle = match self.prefetch_handle.lock() {
            Ok(handle) => handle,
            Err(poisoned) => {
                tracing::warn!("meta prefetch handle mutex was poisoned; recovering");
                poisoned.into_inner()
            }
        };
        if prefetch_handle.is_some() {
            return;
        }

        let meta_url = self.network.meta_url.clone();
        let cache_path = cache_path.to_path_buf();
        match std::thread::Builder::new()
            .name("snapshot-source-prefetch-meta".to_string())
            .spawn(move || {
                Self::prefetch_meta(
                    &meta_url,
                    &cache_path,
                    &ledgers,
                    ledgers_per_batch,
                    batches_per_partition,
                );
            }) {
            Ok(handle) => *prefetch_handle = Some(handle),
            Err(e) => tracing::warn!(error = %e, "failed to spawn meta prefetch thread"),
        }
    }

    /// Fetch (and cache) the SEP-54 storage configuration, returning the
    /// `(ledgers_per_batch, batches_per_partition)` used to derive meta object
    /// paths. The config is small and constant for a network, so it is cached
    /// like any other fetched artifact and read from disk on subsequent calls.
    fn meta_config(&self, cache_path: &Path) -> Result<(u32, u32), Error> {
        let read = cache(cache_path.join("meta-config.json"), |write| {
            get_config(&self.network.meta_url, write)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        })?;
        let config = parse_config(read)?;
        Ok((config.ledgers_per_batch, config.batches_per_partition))
    }

    fn prefetch_meta(
        meta_url: &str,
        cache_path: &Path,
        ledgers: &[u32],
        ledgers_per_batch: u32,
        batches_per_partition: u32,
    ) {
        // Process in chunks of 10 to avoid too many open files
        const MAX_CONCURRENT_DOWNLOADS: usize = 10;
        for chunk in ledgers.chunks(MAX_CONCURRENT_DOWNLOADS) {
            let handles: Vec<_> = chunk
                .iter()
                .map(|&l| {
                    let meta_url = meta_url.to_string();
                    let path = cache_path.join(format!("ledger-{l}.xdr"));
                    std::thread::spawn(move || {
                        let _ = cache(path, |write| {
                            get_ledger(
                                &meta_url,
                                l,
                                ledgers_per_batch,
                                batches_per_partition,
                                write,
                            )
                            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                        });
                    })
                })
                .collect();

            for handle in handles {
                let _ = handle.join();
            }
        }
    }

    fn fetch_from_meta(
        &self,
        cache_path: &Path,
        ledger: u32,
        key: &LedgerKey,
        ledgers_per_batch: u32,
        batches_per_partition: u32,
    ) -> Result<Option<Option<LedgerEntry>>, Error> {
        tracing::debug!(ledger, "fetch from meta");
        let meta_read = cache(cache_path.join(format!("ledger-{ledger}.xdr")), |write| {
            get_ledger(
                &self.network.meta_url,
                ledger,
                ledgers_per_batch,
                batches_per_partition,
                write,
            )
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        })?;
        let meta = parse_ledger(ledger, meta_read)?;

        // Only pass tx_hash for the starting ledger; for earlier ledgers, iterate fully
        let tx_hash_filter = if ledger == self.ledger {
            self.tx_hash
        } else {
            None
        };
        let changes = LedgerEntryChangesIterator::new(&meta, tx_hash_filter);
        for (_phase, _tx_hash, change_key, change_entry) in changes {
            if &change_key == key {
                if let Some(entry) = change_entry {
                    return Ok(Some(Some(entry)));
                } else {
                    return Ok(Some(None));
                }
            }
        }

        Ok(None)
    }

    /// Fetch (and cache) `key` as it was at the end of `ledger` from the RPC.
    ///
    /// State at a past ledger never changes, so a response is cached for good,
    /// including one that reports the entry did not exist. The result is
    /// keyed by the ledger rather than the target ledger, so every target
    /// ledger and transaction whose search bottoms out at the same checkpoint
    /// shares it.
    fn fetch_from_rpc(
        &self,
        cache_path: &Path,
        ledger: u32,
        key: &LedgerKey,
    ) -> Result<Option<LedgerEntry>, Error> {
        tracing::debug!(ledger, "fetch from rpc");
        let key_xdr = key.to_xdr(Limits::none())?;
        let key_hash = Sha256::digest(&key_xdr);
        let rpc_read = cache(
            cache_path.join(format!("rpc-{ledger}-{key_hash:x}.json")),
            |write| {
                let mut buf = Vec::new();
                get_ledger_entry(&self.network.rpc_url, key, ledger, &mut buf)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
                write_rpc_response(&buf, write)
            },
        )?;
        let entry = parse_ledger_entry(rpc_read)?.map(|(entry, _ttl)| entry);
        if let Some(entry) = &entry {
            tracing::debug!(
                last_modified = entry.last_modified_ledger_seq,
                "found from rpc"
            );
        }
        Ok(entry)
    }
}

impl Drop for LedgerEntryFetcher {
    fn drop(&mut self) {
        let handle = match self.prefetch_handle.get_mut() {
            Ok(handle) => handle.take(),
            Err(poisoned) => {
                tracing::warn!("meta prefetch handle mutex was poisoned during drop; recovering");
                poisoned.into_inner().take()
            }
        };
        if handle.is_some_and(|handle| handle.join().is_err()) {
            tracing::warn!("meta prefetch thread panicked");
        }
    }
}

#[cfg(test)]
mod test_network {
    use super::{checkpoint_search_bounds, write_rpc_response, Network};
    use soroban_sdk::xdr::{Hash, LedgerEntryData, Limits, TtlEntry, WriteXdr};

    #[test]
    fn checkpoint_bounds_typical() {
        // Mainnet checkpoint frequency is 64; checkpoints fall on k*64 - 1.
        // For 61292152 the enclosing checkpoint is 61292096 - 1 = 61292095.
        assert_eq!(checkpoint_search_bounds(61292152, 64), (61292095, 57));
        // A ledger exactly on a 64-aligned boundary.
        assert_eq!(checkpoint_search_bounds(128, 64), (127, 1));
        // Local network uses a checkpoint frequency of 8.
        assert_eq!(checkpoint_search_bounds(1845, 8), (1839, 6));
    }

    #[test]
    fn checkpoint_bounds_first_window_does_not_underflow() {
        // Ledgers below the checkpoint count have no previous checkpoint; the
        // bounds must degrade to a search down to ledger 0 rather than
        // panicking on u32 underflow (overflow-checks is on for this crate).
        assert_eq!(checkpoint_search_bounds(10, 64), (0, 10));
        assert_eq!(checkpoint_search_bounds(63, 64), (0, 63));
        assert_eq!(checkpoint_search_bounds(0, 64), (0, 0));
        assert_eq!(checkpoint_search_bounds(7, 8), (0, 7));
    }

    // The network id is the SHA-256 of the network passphrase. These values are
    // pinned because the IDs are part of Stellar's protocol-level network
    // identity.
    #[test]
    fn network_id_hex_is_stable() {
        assert_eq!(
            Network::mainnet("".to_string()).network_id_hex(),
            "7ac33997544e3175d266bd022439b22cdb16508c01163f26e5cb2a3e1045a979",
        );
        assert_eq!(
            // network_id depends only on the passphrase, so the start date here
            // is irrelevant to the hash.
            Network::testnet("2025-12-17".to_string()).network_id_hex(),
            "cee0302d59844d32bdca915c8203dd44b33fbb7edc19051ea37abedf28ecd472",
        );
        assert_eq!(
            Network::local().network_id_hex(),
            "baefd734b8d3e48472cff83912375fedbc7573701912fe308af730180f97d74a",
        );
    }

    #[test]
    fn network_id_hex_matches_network_id_bytes() {
        let n = Network::mainnet("".to_string());
        assert_eq!(hex::encode(n.network_id()), n.network_id_hex());
        assert_eq!(n.network_id().len(), 32);
    }

    #[test]
    fn mainnet_defaults() {
        let n = Network::mainnet("".to_string());
        assert_eq!(n.name, "mainnet");
        assert_eq!(
            n.passphrase,
            "Public Global Stellar Network ; September 2015"
        );
        assert_eq!(n.archive_checkpoint_ledger_count, 64);
        assert!(n.meta_url.starts_with("https://"));
    }

    #[test]
    fn mainnet_rpc_url_is_passed_through() {
        let n = Network::mainnet("https://example.com/rpc".to_string());
        assert_eq!(n.rpc_url, "https://example.com/rpc");
    }

    #[test]
    fn name_is_filesystem_safe() {
        let mut n = Network::mainnet("".to_string());
        n.name = "../my testnet: 2025/12/17".to_string();
        assert_eq!(n.name(), "mytestnet20251217");

        n.name = "../".to_string();
        assert_eq!(n.name(), n.network_id_hex());
    }

    #[test]
    fn testnet_defaults() {
        let n = Network::testnet("2025-12-17".to_string());
        assert_eq!(n.name, "testnet-2025-12-17");
        assert_eq!(n.passphrase, "Test SDF Network ; September 2015");
        // The start date is appended to the AWS testnet meta partition path.
        assert_eq!(
            n.meta_url,
            "https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/testnet/2025-12-17",
        );
        assert_eq!(n.rpc_url, "https://soroban-testnet.stellar.org");
        assert_eq!(n.archive_checkpoint_ledger_count, 64);
    }

    #[test]
    fn local_defaults() {
        let n = Network::local();
        assert_eq!(n.name, "local");
        assert_eq!(n.passphrase, "Standalone Network ; February 2017");
        assert_eq!(n.rpc_url, "http://localhost:8000/rpc");
        assert_eq!(n.meta_url, "http://localhost:8000/ledger-meta");
        assert_eq!(n.archive_checkpoint_ledger_count, 8);
    }

    #[test]
    fn rpc_error_response_is_rejected_before_write() {
        let error = r#"{"error":{"code":-32602,"message":"ledger 5 is below the oldest stored ledger 10"}}"#;
        let mut written = Vec::new();
        assert!(
            write_rpc_response(error.as_bytes(), &mut written).is_err(),
            "rpc error response must be rejected"
        );
        assert!(
            written.is_empty(),
            "rpc error response must be rejected before any cache bytes are written"
        );

        let xdr = LedgerEntryData::Ttl(TtlEntry {
            key_hash: Hash([7u8; 32]),
            live_until_ledger_seq: 123,
        })
        .to_xdr_base64(Limits::none())
        .unwrap();
        let found = format!(
            r#"{{"result":{{"latestLedger":99,"entries":[{{"xdr":"{xdr}","lastModifiedLedgerSeq":42}}]}}}}"#
        );
        write_rpc_response(found.as_bytes(), &mut written).unwrap();
        assert_eq!(written, found.as_bytes());

        let absent = r#"{"result":{"latestLedger":99,"entries":[]}}"#;
        let mut written = Vec::new();
        write_rpc_response(absent.as_bytes(), &mut written).unwrap();
        assert_eq!(written, absent.as_bytes());
    }
}
