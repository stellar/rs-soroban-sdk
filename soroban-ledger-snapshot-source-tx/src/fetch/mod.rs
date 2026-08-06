use crate::cache::{cache, CacheError};
use crate::xdr_schema_version;
use from_history_archive::{get_bucket, get_history, parse_history};
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

pub(crate) mod bucket_index;
pub(crate) mod from_history_archive;
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
    #[error("history archive error: {0}")]
    HistoryArchive(#[from] from_history_archive::Error),
    #[error("bucket index error: {0}")]
    BucketIndex(#[from] bucket_index::Error),
}

/// Network configuration for fetching ledger data
///
/// Contains URLs for SEP-54 meta storage, RPC, and history archive,
/// as well as the checkpoint frequency for the network.
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
    /// URL to the RPC (optional, used as optimization to skip searching meta/archive)
    pub rpc_url: Option<String>,
    /// URL to the History Archive storage
    pub archive_url: String,
    /// Number of ledgers between checkpoints
    pub archive_checkpoint_ledger_count: u32,
}

impl Network {
    /// Create a Network configuration for Stellar mainnet with default URLs
    ///
    /// Uses default mainnet URLs:
    /// - SEP-54 meta storage: AWS public blockchain
    /// - History archive: history.stellar.org
    ///
    /// # Arguments
    /// * `rpc_url` - Optional RPC URL, used as an optimization to skip searching meta/archive
    pub fn mainnet(rpc_url: Option<String>) -> Self {
        Self {
            name: "mainnet".to_string(),
            passphrase: "Public Global Stellar Network ; September 2015".to_string(),
            meta_url: "https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/pubnet".to_string(),
            rpc_url,
            archive_url: "https://history.stellar.org/prd/core-live/core_live_001".to_string(),
            archive_checkpoint_ledger_count: 64,
        }
    }

    /// Create a Network configuration for Stellar testnet with default URLs
    ///
    /// Uses default testnet URLs:
    /// - RPC: soroban-testnet.stellar.org
    /// - History archive: history.stellar.org
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
            rpc_url: Some("https://soroban-testnet.stellar.org".to_string()),
            archive_url: "https://history.stellar.org/prd/core-testnet/core_testnet_001"
                .to_string(),
            archive_checkpoint_ledger_count: 64,
        }
    }

    /// Create a Network configuration for local Stellar Quickstart
    ///
    /// Uses default quickstart URLs:
    /// - SEP-54 meta storage: localhost:8000/meta-archive
    /// - RPC: localhost:8000/rpc
    /// - History archive: localhost:8000/archive
    pub fn local() -> Self {
        Self {
            name: "local".to_string(),
            passphrase: "Standalone Network ; February 2017".to_string(),
            meta_url: "http://localhost:8000/meta-archive".to_string(),
            rpc_url: Some("http://localhost:8000/rpc".to_string()),
            archive_url: "http://localhost:8000/archive".to_string(),
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
/// meta search walks before falling back to the history archive.
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

/// Version of the on-disk history-result cache's format/semantics.
///
/// This is independent of [`xdr_schema_version`]: it exists to invalidate
/// cached results when what gets *computed and stored* under a given
/// `(checkpoint, key)` changes, even though the XDR encoding of a
/// `LedgerEntry` did not. For example, adding the planned hot-archive
/// fallback (see the TODO in `fetch_from_archive_uncached`) would change the
/// meaning of a cached `None`: today it means "not found in the live
/// history-archive buckets", but once evicted entries are also consulted, a
/// stale `None` from before that change would be silently wrong. Bump this
/// constant whenever such a semantic change is made; because it is part of
/// the cache path (see [`history_result_cache_path`]), older cache entries
/// are simply orphaned under their old path rather than misread as if they
/// were produced by the new logic.
const HISTORY_RESULT_CACHE_VERSION: u32 = 1;

/// Compute the machine-local cache path for a resolved history-archive
/// result, keyed by the history checkpoint sequence and the ledger-key hash.
///
/// Deliberately excludes the target ledger and tx hash from the key: two
/// fetches whose search bottoms out at the same checkpoint (see
/// [`checkpoint_search_bounds`]) always resolve to the same archive result for
/// a given key, regardless of which target ledger or transaction triggered
/// the search, so keying on the checkpoint lets them share one cached result
/// instead of each re-decoding the checkpoint's buckets from scratch.
///
/// The path is additionally namespaced by [`HISTORY_RESULT_CACHE_VERSION`]
/// and [`xdr_schema_version`], so a bump to either one changes the path
/// itself rather than requiring an in-payload version check: old cache files
/// are simply missed (and left behind, harmlessly orphaned) instead of being
/// read and silently misinterpreted.
fn history_result_cache_path(
    cache_path: &Path,
    checkpoint: u32,
    key: &LedgerKey,
) -> Result<PathBuf, Error> {
    let key_xdr = key.to_xdr(Limits::none())?;
    let key_hash = Sha256::digest(&key_xdr);
    Ok(cache_path
        .join(format!(
            "history-result-v{HISTORY_RESULT_CACHE_VERSION}-xdr-{}",
            xdr_schema_version(),
        ))
        .join(format!("{checkpoint}-{key_hash:x}.json")))
}

/// Cache wrapper around a history-archive lookup for `key` at `checkpoint`,
/// invoking `uncached` only on a cache miss.
///
/// Factored out of [`LedgerEntryFetcher::fetch_from_archive`] (which supplies
/// `uncached` as a closure over `self.fetch_from_archive_uncached(..)`) so the
/// caching behavior — hit/miss, `Some`/`None` round-tripping, and how a
/// failed collection is handled — can be exercised directly in tests without
/// a real `LedgerEntryFetcher` or any network access.
fn fetch_from_archive_cached(
    cache_path: &Path,
    checkpoint: u32,
    key: &LedgerKey,
    uncached: impl FnOnce() -> Result<Option<LedgerEntry>, Error>,
) -> Result<Option<LedgerEntry>, Error> {
    let result_path = history_result_cache_path(cache_path, checkpoint, key)?;
    let result_read = cache(result_path, |write| {
        let result = uncached()?;
        serde_json::to_writer_pretty(write, &result)?;
        Ok(())
    })?;
    Ok(serde_json::from_reader(result_read)?)
}

fn write_usable_rpc_response<W: std::io::Write + ?Sized>(
    response: &[u8],
    ledger: u32,
    write: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    let usable = match parse_ledger_entry(Cursor::new(response))? {
        Some((entry, _ttl, latest_ledger)) => {
            entry.last_modified_ledger_seq < ledger && latest_ledger >= ledger
        }
        None => false,
    };
    if !usable {
        return Err("rpc response not usable for this ledger".into());
    }
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
    /// * `network` - Network configuration with URLs for meta storage, RPC, and history archive
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
        // Optimization: Try RPC for contract code entries only (before prefetch)
        if matches!(key, LedgerKey::ContractCode(_)) {
            if let Some(result) = self.fetch_from_rpc(cache_path, self.ledger, key)? {
                return Ok(result);
            }
        }

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

        // Optimization: Try RPC for all ledger entries
        if let Some(result) = self.fetch_from_rpc(cache_path, self.ledger, key)? {
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

        // Phase 3: Fetch from history archive at the previous checkpoint
        self.fetch_from_archive(&cache_path, prev_checkpoint, key)
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

    fn fetch_from_rpc(
        &self,
        cache_path: &Path,
        ledger: u32,
        key: &LedgerKey,
    ) -> Result<Option<Option<LedgerEntry>>, Error> {
        let Some(rpc_url) = &self.network.rpc_url else {
            return Ok(None);
        };
        tracing::debug!(ledger, "fetch from rpc");
        let key_xdr = key.to_xdr(Limits::none())?;
        let key_hash = Sha256::digest(&key_xdr);
        let rpc_cache_path = cache_path.join(format!("rpc-{ledger}-{key_hash:x}.json"));
        let rpc_read = match cache(rpc_cache_path, |write| {
            // Fetch the raw RPC response into memory first, so its usability can
            // be checked before it is committed to the cache.
            let mut buf = Vec::new();
            get_ledger_entry(rpc_url, key, &mut buf)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            // Only trust (and therefore cache) the RPC answer when the node has
            // actually observed the target ledger (latest_ledger >= ledger). If
            // the node is lagging, an entry whose last_modified is below `ledger`
            // might still be modified at or after `ledger` once the node catches
            // up, so the response could be stale. Doing this check here, inside
            // the cache collector, means a non-usable response is never written:
            // `cache()` discards the temp file when the collector errors, rather
            // than persisting a potentially-wrong answer that would be replayed
            // on every later run (permanently disabling the RPC fast path for
            // this (ledger, key) even after the node catches up).
            write_usable_rpc_response(&buf, ledger, write)
        }) {
            Ok(rpc_read) => rpc_read,
            // Not usable, not found, or a transient RPC error: fall back to the
            // authoritative meta/archive search below.
            Err(_) => return Ok(None),
        };
        // The cached response is known-usable (only usable responses are
        // persisted above), so return its entry.
        if let Some((entry, _ttl, _latest_ledger)) = parse_ledger_entry(rpc_read)? {
            tracing::debug!(
                last_modified = entry.last_modified_ledger_seq,
                "found from rpc"
            );
            return Ok(Some(Some(entry)));
        }

        Ok(None)
    }

    /// Fetch (and cache) the history-archive result for `key` at the
    /// checkpoint `ledger`.
    ///
    /// This is the entry point called after phases 1 and 2 (per-ledger meta
    /// search) have exhausted the range down to the previous checkpoint, so
    /// `ledger` here is always a checkpoint sequence, not an arbitrary target
    /// ledger. The result is cached keyed by `(checkpoint, key_hash)` rather
    /// than by the target ledger: fetch logic always first replays ledger-close
    /// meta from the target ledger back to immediately after the previous
    /// checkpoint (phases 1-2), then falls back to the immutable state
    /// represented by that checkpoint (this phase). That means the resolved
    /// archive result for a given key is identical for every target ledger
    /// (and every transaction within it) that shares the same checkpoint.
    /// Keying by the target ledger instead would prevent that reuse without
    /// improving correctness, forcing every fork snapshot in a checkpoint
    /// interval to re-decode the full (potentially many-GB) bucket set from
    /// scratch. Both `Some(LedgerEntry)` and `None` are cached, since a miss
    /// is just as expensive to determine as a hit.
    fn fetch_from_archive(
        &self,
        cache_path: &Path,
        checkpoint: u32,
        key: &LedgerKey,
    ) -> Result<Option<LedgerEntry>, Error> {
        fetch_from_archive_cached(cache_path, checkpoint, key, || {
            self.fetch_from_archive_uncached(cache_path, checkpoint, key)
        })
    }

    fn fetch_from_archive_uncached(
        &self,
        cache_path: &Path,
        ledger: u32,
        key: &LedgerKey,
    ) -> Result<Option<LedgerEntry>, Error> {
        tracing::debug!(ledger, "fetch from archive");
        // TODO: Fetching from archives should be replaced with a historical data source supporting
        // random access. But for now history archives will do, albeit slow.
        let history_read = cache(
            cache_path.join(format!("history-{}.json", ledger)),
            |write| {
                get_history(&self.network.archive_url, ledger, write)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            },
        )?;
        let history = parse_history(history_read)?;
        let buckets = history
            .current_buckets
            .iter()
            .flat_map(|b| [&b.curr, &b.snap])
            .filter(|b| *b != "0000000000000000000000000000000000000000000000000000000000000000");
        for bucket in buckets {
            let bucket_path = cache_path.join(format!("bucket-{bucket}.xdr"));
            tracing::debug!(bucket, "fetch bucket");
            let bucket_read = cache(bucket_path.clone(), |write| {
                let compressed_size = get_bucket(&self.network.archive_url, bucket, write)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
                if let Some(compressed_size) = compressed_size {
                    tracing::debug!(bucket, compressed_size, "fetch bucket (downloaded)");
                }
                Ok(())
            })?;
            // The bucket is read back through a seekable handle by the index,
            // so the sequential reader `cache` returns is only used to confirm
            // the download completed.
            drop(bucket_read);
            let size = std::fs::metadata(&bucket_path)
                .map(|m| m.len())
                .unwrap_or(0);
            tracing::debug!(bucket, size, "fetch bucket (decompressed)");
            // Bucket ordering is unchanged: buckets are visited newest level
            // first, so the first bucket that mentions the key decides the
            // answer. Only the inner linear scan of each bucket is replaced by
            // a binary search over that bucket's persistent offset index.
            match bucket_index::lookup(cache_path, bucket, &bucket_path, key)? {
                bucket_index::Lookup::Present(ledger_entry) => return Ok(Some(ledger_entry)),
                bucket_index::Lookup::Deleted => return Ok(None),
                bucket_index::Lookup::Absent => {}
            }
        }

        // TODO: If the entry isn't found by here, and the entry is an entry
        // that can be evicted to the hot archive (contract data that is persisted
        // only, or contract code), then get the hot archive buckets. It is already
        // expensive to download the live history archives, so we haven't
        // implemented also downloading the hot archives that could be much larger
        // over time.

        Ok(None)
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
    use super::{checkpoint_search_bounds, write_usable_rpc_response, Network};
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
            Network::mainnet(None).network_id_hex(),
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
        let n = Network::mainnet(None);
        assert_eq!(hex::encode(n.network_id()), n.network_id_hex());
        assert_eq!(n.network_id().len(), 32);
    }

    #[test]
    fn mainnet_defaults() {
        let n = Network::mainnet(None);
        assert_eq!(n.name, "mainnet");
        assert_eq!(
            n.passphrase,
            "Public Global Stellar Network ; September 2015"
        );
        assert_eq!(n.rpc_url, None);
        assert_eq!(n.archive_checkpoint_ledger_count, 64);
        assert!(n.meta_url.starts_with("https://"));
        assert!(n.archive_url.starts_with("https://history.stellar.org"));
    }

    #[test]
    fn mainnet_rpc_url_is_passed_through() {
        let n = Network::mainnet(Some("https://example.com/rpc".to_string()));
        assert_eq!(n.rpc_url.as_deref(), Some("https://example.com/rpc"));
    }

    #[test]
    fn name_is_filesystem_safe() {
        let mut n = Network::mainnet(None);
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
        assert_eq!(
            n.rpc_url.as_deref(),
            Some("https://soroban-testnet.stellar.org"),
        );
        assert_eq!(n.archive_checkpoint_ledger_count, 64);
    }

    #[test]
    fn local_defaults() {
        let n = Network::local();
        assert_eq!(n.name, "local");
        assert_eq!(n.passphrase, "Standalone Network ; February 2017");
        assert_eq!(n.rpc_url.as_deref(), Some("http://localhost:8000/rpc"));
        assert_eq!(n.meta_url, "http://localhost:8000/meta-archive");
        assert_eq!(n.archive_url, "http://localhost:8000/archive");
        assert_eq!(n.archive_checkpoint_ledger_count, 8);
    }

    #[test]
    fn lagging_rpc_response_is_rejected_before_write() {
        let xdr = LedgerEntryData::Ttl(TtlEntry {
            key_hash: Hash([7u8; 32]),
            live_until_ledger_seq: 123,
        })
        .to_xdr_base64(Limits::none())
        .unwrap();
        let lagging = format!(
            r#"{{"result":{{"latestLedger":99,"entries":[{{"xdr":"{xdr}","lastModifiedLedgerSeq":42}}]}}}}"#
        );
        let mut written = Vec::new();
        assert!(
            write_usable_rpc_response(lagging.as_bytes(), 100, &mut written).is_err(),
            "lagging RPC response must be rejected"
        );
        assert!(
            written.is_empty(),
            "lagging RPC response must be rejected before any cache bytes are written"
        );

        let usable = lagging.replace(r#""latestLedger":99"#, r#""latestLedger":100"#);
        write_usable_rpc_response(usable.as_bytes(), 100, &mut written).unwrap();
        assert_eq!(written, usable.as_bytes());
    }
}

#[cfg(test)]
mod test_history_result_cache {
    use super::{checkpoint_search_bounds, history_result_cache_path};
    use soroban_sdk::xdr::{
        AccountId, Hash, LedgerKey, LedgerKeyAccount, LedgerKeyContractCode, PublicKey, Uint256,
    };
    use std::path::Path;

    fn account_key(byte: u8) -> LedgerKey {
        LedgerKey::Account(LedgerKeyAccount {
            account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([byte; 32]))),
        })
    }

    #[test]
    fn distinct_target_ledgers_in_same_checkpoint_interval_reuse_the_same_key() {
        // Mainnet checkpoint frequency is 64; ledgers 61292100 and 61292152
        // both fall within the checkpoint interval enclosed by checkpoint
        // 61292095 (see checkpoint_bounds_typical in test_network above).
        let (checkpoint_a, _) = checkpoint_search_bounds(61292100, 64);
        let (checkpoint_b, _) = checkpoint_search_bounds(61292152, 64);
        assert_eq!(
            checkpoint_a, checkpoint_b,
            "both target ledgers should resolve to the same checkpoint"
        );

        let key = account_key(1);
        let cache_path = Path::new("/cache/mainnet");
        let path_a = history_result_cache_path(cache_path, checkpoint_a, &key).unwrap();
        let path_b = history_result_cache_path(cache_path, checkpoint_b, &key).unwrap();
        assert_eq!(
            path_a, path_b,
            "distinct target ledgers sharing a checkpoint must reuse the same history-result cache key"
        );
    }

    #[test]
    fn distinct_checkpoints_do_not_collide() {
        let key = account_key(1);
        let cache_path = Path::new("/cache/mainnet");
        let path_a = history_result_cache_path(cache_path, 127, &key).unwrap();
        let path_b = history_result_cache_path(cache_path, 191, &key).unwrap();
        assert_ne!(
            path_a, path_b,
            "different checkpoints must not collide on the same cache key"
        );
    }

    #[test]
    fn distinct_keys_do_not_collide() {
        let cache_path = Path::new("/cache/mainnet");
        let path_a = history_result_cache_path(cache_path, 127, &account_key(1)).unwrap();
        let path_b = history_result_cache_path(cache_path, 127, &account_key(2)).unwrap();
        assert_ne!(
            path_a, path_b,
            "different ledger keys must not collide on the same cache key"
        );

        let contract_code_key = LedgerKey::ContractCode(LedgerKeyContractCode {
            hash: Hash([1u8; 32]),
        });
        let path_c = history_result_cache_path(cache_path, 127, &contract_code_key).unwrap();
        assert_ne!(
            path_a, path_c,
            "different ledger key variants must not collide on the same cache key"
        );
    }

    #[test]
    fn cache_path_is_namespaced_under_the_provided_cache_dir() {
        let key = account_key(1);
        let mainnet_path =
            history_result_cache_path(Path::new("/cache/mainnet"), 127, &key).unwrap();
        let testnet_path =
            history_result_cache_path(Path::new("/cache/testnet"), 127, &key).unwrap();
        assert!(mainnet_path.starts_with("/cache/mainnet"));
        assert!(testnet_path.starts_with("/cache/testnet"));
        assert_ne!(
            mainnet_path, testnet_path,
            "cache paths under different network directories must not collide"
        );
    }
}

#[cfg(test)]
mod test_fetch_from_archive_cached {
    use super::{fetch_from_archive_cached, Error};
    use soroban_sdk::xdr::{
        AccountId, Hash, LedgerEntry, LedgerEntryData, LedgerEntryExt, LedgerKey, LedgerKeyAccount,
        PublicKey, TtlEntry, Uint256,
    };
    use std::cell::Cell;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU32, Ordering};

    /// A unique, self-cleaning scratch directory for a single test, so tests
    /// exercising the real on-disk cache don't interfere with each other or
    /// leave files behind.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            static COUNTER: AtomicU32 = AtomicU32::new(0);
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!(
                "soroban-fetch-from-archive-cached-test-{}-{name}-{n}",
                std::process::id(),
            ));
            std::fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn account_key(byte: u8) -> LedgerKey {
        LedgerKey::Account(LedgerKeyAccount {
            account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([byte; 32]))),
        })
    }

    /// A minimal, cheap-to-construct `LedgerEntry` for round-trip assertions.
    fn entry(byte: u8) -> LedgerEntry {
        LedgerEntry {
            last_modified_ledger_seq: 0,
            data: LedgerEntryData::Ttl(TtlEntry {
                key_hash: Hash([byte; 32]),
                live_until_ledger_seq: 100,
            }),
            ext: LedgerEntryExt::V0,
        }
    }

    #[test]
    fn cached_some_round_trips_and_bypasses_collector_on_second_lookup() {
        let dir = TempDir::new("some");
        let key = account_key(1);
        let calls = Cell::new(0u32);
        let want = entry(9);

        let first = fetch_from_archive_cached(dir.path(), 127, &key, || {
            calls.set(calls.get() + 1);
            Ok(Some(want.clone()))
        })
        .unwrap();
        assert_eq!(first, Some(want.clone()));
        assert_eq!(calls.get(), 1);

        let second = fetch_from_archive_cached(dir.path(), 127, &key, || {
            calls.set(calls.get() + 1);
            panic!("collector must not be called on a cache hit");
        })
        .unwrap();
        assert_eq!(second, Some(want));
        assert_eq!(
            calls.get(),
            1,
            "collector must not be invoked again once a Some(..) result is cached"
        );
    }

    #[test]
    fn cached_none_reuses_result_and_bypasses_collector() {
        let dir = TempDir::new("none");
        let key = account_key(2);
        let calls = Cell::new(0u32);

        let first = fetch_from_archive_cached(dir.path(), 127, &key, || {
            calls.set(calls.get() + 1);
            Ok(None)
        })
        .unwrap();
        assert_eq!(first, None);
        assert_eq!(calls.get(), 1);

        let second = fetch_from_archive_cached(dir.path(), 127, &key, || {
            calls.set(calls.get() + 1);
            panic!("collector must not be called on a cache hit for a cached None");
        })
        .unwrap();
        assert_eq!(second, None);
        assert_eq!(
            calls.get(),
            1,
            "a cached None must be reused without recomputation, just like a cached Some"
        );
    }

    #[test]
    fn collector_error_does_not_poison_cache_and_a_later_retry_can_succeed() {
        let dir = TempDir::new("err");
        let key = account_key(3);
        let want = entry(4);

        let failed = fetch_from_archive_cached(dir.path(), 127, &key, || {
            Err(Error::Io(std::io::Error::other("boom")))
        });
        assert!(failed.is_err(), "a collector error must propagate");

        // The failed attempt above must not have left behind a partial or
        // poisoned cache entry: a later, successful collection for the same
        // (checkpoint, key) must be free to run and its result must be
        // returned (and itself cached) normally.
        let retried =
            fetch_from_archive_cached(dir.path(), 127, &key, || Ok(Some(want.clone()))).unwrap();
        assert_eq!(retried, Some(want.clone()));

        // And that successful result is now the one that's cached.
        let cached = fetch_from_archive_cached(dir.path(), 127, &key, || {
            panic!("collector must not be called once a successful result is cached");
        })
        .unwrap();
        assert_eq!(cached, Some(want));
    }

    #[test]
    fn same_checkpoint_and_key_share_one_cache_entry() {
        let dir = TempDir::new("share");
        let key = account_key(5);
        let calls = Cell::new(0u32);
        let want = entry(6);

        // Simulates two different target ledgers whose search both bottom
        // out at checkpoint 200 for the same key: the second call must reuse
        // the first call's cached result rather than recomputing it.
        let a = fetch_from_archive_cached(dir.path(), 200, &key, || {
            calls.set(calls.get() + 1);
            Ok(Some(want.clone()))
        })
        .unwrap();
        let b = fetch_from_archive_cached(dir.path(), 200, &key, || {
            calls.set(calls.get() + 1);
            panic!("second lookup for the same checkpoint/key must hit the cache");
        })
        .unwrap();

        assert_eq!(a, Some(want.clone()));
        assert_eq!(b, Some(want));
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn different_checkpoints_or_keys_do_not_share_cache_entries() {
        let dir = TempDir::new("no-collide");
        let calls = Cell::new(0u32);
        let entry_a = entry(7);
        let entry_b = entry(8);
        let entry_c = entry(9);
        let key = account_key(6);

        let by_checkpoint_a = fetch_from_archive_cached(dir.path(), 300, &key, || {
            calls.set(calls.get() + 1);
            Ok(Some(entry_a.clone()))
        })
        .unwrap();
        let by_checkpoint_b = fetch_from_archive_cached(dir.path(), 301, &key, || {
            calls.set(calls.get() + 1);
            Ok(Some(entry_b.clone()))
        })
        .unwrap();
        let by_key = fetch_from_archive_cached(dir.path(), 300, &account_key(7), || {
            calls.set(calls.get() + 1);
            Ok(Some(entry_c.clone()))
        })
        .unwrap();

        assert_eq!(by_checkpoint_a, Some(entry_a));
        assert_eq!(by_checkpoint_b, Some(entry_b));
        assert_eq!(by_key, Some(entry_c));
        assert_eq!(
            calls.get(),
            3,
            "different checkpoints and different keys must each get their own cache entry"
        );
    }
}
