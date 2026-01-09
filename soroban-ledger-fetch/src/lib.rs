use cache_to_file::{cache, CacheError};
use sha2::{Digest, Sha256};
use soroban_ledger_fetch_from_history_archive::{
    get_bucket, get_history, parse_bucket, parse_history,
};
use soroban_ledger_fetch_from_meta_storage::{get_ledger, parse_ledger};
use soroban_ledger_fetch_from_rpc::{get_ledger_entry, parse_ledger_entry};
use soroban_sdk::xdr::{BucketEntry, LedgerEntry, LedgerKey, Limited, Limits, WriteXdr};
use std::path::PathBuf;

mod iter;
pub use iter::{LedgerEntryChangesIterator, ProcessingPhase};

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
    MetaStorage(#[from] soroban_ledger_fetch_from_meta_storage::Error),
    #[error("rpc error: {0}")]
    Rpc(#[from] soroban_ledger_fetch_from_rpc::Error),
    #[error("history archive error: {0}")]
    HistoryArchive(#[from] soroban_ledger_fetch_from_history_archive::Error),
}

/// Returns true if the ledger is a checkpoint ledger for the given checkpoint frequency.
pub fn is_checkpoint_ledger(ledger: u32, checkpoint_frequency: u32) -> bool {
    (ledger + 1) % checkpoint_frequency == 0
}

/// Network configuration for fetching ledger data
///
/// Contains URLs for SEP-54 meta storage, RPC, and history archive,
/// as well as the checkpoint frequency for the network.
#[derive(Debug, Clone)]
pub struct Network {
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
    /// - RPC: mainnet.sorobanrpc.com
    /// - History archive: history.stellar.org
    pub fn mainnet() -> Self {
        Self {
            passphrase: "Public Global Stellar Network ; September 2015".to_string(),
            meta_url: "https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/pubnet".to_string(),
            rpc_url: Some("https://mainnet.sorobanrpc.com".to_string()),
            archive_url: "https://history.stellar.org/prd/core-live/core_live_001".to_string(),
            archive_checkpoint_ledger_count: 64,
        }
    }

    /// Create a Network configuration for Stellar testnet with default URLs
    ///
    /// Uses default testnet URLs:
    /// - SEP-54 meta storage: AWS public blockchain
    /// - RPC: soroban-testnet.stellar.org
    /// - History archive: history.stellar.org
    pub fn testnet() -> Self {
        Self {
            passphrase: "Test SDF Network ; September 2015".to_string(),
            meta_url: "https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/testnet/2025-12-17".to_string(),
            rpc_url: Some("https://soroban-testnet.stellar.org".to_string()),
            archive_url: "https://history.stellar.org/prd/core-testnet/core_testnet_001".to_string(),
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
            passphrase: "Standalone Network ; February 2017".to_string(),
            meta_url: "http://localhost:8000/meta-archive".to_string(),
            rpc_url: Some("http://localhost:8000/rpc".to_string()),
            archive_url: "http://localhost:8000/archive".to_string(),
            archive_checkpoint_ledger_count: 8,
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

/// Fetcher for ledger entries that downloads ledger meta and searches for entries
pub struct LedgerEntryFetcher {
    network: Network,
    ledger: u32,
    tx_hash: Option<[u8; 32]>,
    cache_path: PathBuf,
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
        tracing::info!(key = %serde_json::to_value(key)?, "fetch");
        let result = self.fetch_with_entry_cache(key);
        if let Ok(entry) = &result {
            tracing::info!(entry = %serde_json::to_value(entry)?, "found")
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

        // Ensure cache directory exists
        std::fs::create_dir_all(&ledger_cache_dir)?;

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
        cache_path: &PathBuf,
    ) -> Result<Option<LedgerEntry>, Error> {
        std::fs::create_dir_all(cache_path)?;

        // Optimization: Try RPC for contract code entries only (before prefetch)
        if matches!(key, LedgerKey::ContractCode(_)) {
            if let Some(result) = self.fetch_from_rpc(cache_path, self.ledger, key)? {
                return Ok(result);
            }
        }

        // Calculate checkpoint boundaries
        let checkpoint_count = self.network.archive_checkpoint_ledger_count;
        let prev_checkpoint = (self.ledger / checkpoint_count) * checkpoint_count - 1;
        let ledgers_to_checkpoint = self.ledger - prev_checkpoint;

        // Prefetch all meta for ledgers from starting ledger down to the checkpoint (in background)
        let prefetch_meta_url = self.network.meta_url.clone();
        let prefetch_cache_path = cache_path.clone();
        let prefetch_ledgers: Vec<u32> = (0..ledgers_to_checkpoint)
            .filter_map(|i| self.ledger.checked_sub(i))
            .collect();
        tracing::debug!(
            count = prefetch_ledgers.len(),
            first = prefetch_ledgers.first(),
            last = prefetch_ledgers.last(),
            "fetch from meta range"
        );
        std::thread::spawn(move || {
            Self::prefetch_meta(&prefetch_meta_url, &prefetch_cache_path, &prefetch_ledgers);
        });

        // Phase 1: Check the starting ledger
        if let Some(result) = self.fetch_from_meta(&cache_path, self.ledger, key)? {
            return Ok(result);
        }

        // Optimization: Try RPC for all ledger entries
        if let Some(result) = self.fetch_from_rpc(&cache_path, self.ledger, key)? {
            return Ok(result);
        }

        // Phase 2: Search through previous ledgers down to the previous checkpoint
        for ledger in (prev_checkpoint + 1..self.ledger).rev() {
            if let Some(result) = self.fetch_from_meta(&cache_path, ledger, key)? {
                return Ok(result);
            }
        }

        // Phase 3: Fetch from history archive at the previous checkpoint
        self.fetch_from_archive(&cache_path, prev_checkpoint, key)
    }

    fn prefetch_meta(meta_url: &str, cache_path: &PathBuf, ledgers: &[u32]) {
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
                            get_ledger(&meta_url, l, write)
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
        cache_path: &PathBuf,
        ledger: u32,
        key: &LedgerKey,
    ) -> Result<Option<Option<LedgerEntry>>, Error> {
        tracing::debug!(ledger, "fetch from meta");
        let meta_read = cache(cache_path.join(format!("ledger-{ledger}.xdr")), |write| {
            get_ledger(&self.network.meta_url, ledger, write)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        })?;
        let meta = parse_ledger(meta_read)?;

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
        cache_path: &PathBuf,
        ledger: u32,
        key: &LedgerKey,
    ) -> Result<Option<Option<LedgerEntry>>, Error> {
        let Some(rpc_url) = &self.network.rpc_url else {
            return Ok(None);
        };
        tracing::debug!(ledger, "fetch from rpc");
        let key_xdr = key.to_xdr(Limits::none())?;
        let key_hash = Sha256::digest(&key_xdr);
        let rpc_read = cache(
            cache_path.join(format!("rpc-{ledger}-{key_hash:x}.json")),
            |write| {
                get_ledger_entry(rpc_url, key, write)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            },
        )?;
        if let Some((entry, _ttl)) = parse_ledger_entry(rpc_read)? {
            let usable = entry.last_modified_ledger_seq < ledger;
            tracing::debug!(
                last_modified = entry.last_modified_ledger_seq,
                usable,
                "found from rpc"
            );
            if usable {
                return Ok(Some(Some(entry)));
            }
        }

        Ok(None)
    }

    fn fetch_from_archive(
        &self,
        cache_path: &PathBuf,
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
            let size = std::fs::metadata(&bucket_path)
                .map(|m| m.len())
                .unwrap_or(0);
            tracing::debug!(bucket, size, "fetch bucket (decompressed)");
            let mut limited_reader = Limited::new(bucket_read, Limits::none());
            let bucket_entries_iter = parse_bucket(&mut limited_reader);
            for entry_result in bucket_entries_iter {
                let entry = entry_result?.0;
                match entry {
                    BucketEntry::Liveentry(ledger_entry) | BucketEntry::Initentry(ledger_entry) => {
                        if ledger_entry.to_key() == *key {
                            return Ok(Some(ledger_entry));
                        }
                    }
                    BucketEntry::Deadentry(dead_entry) => {
                        if dead_entry == *key {
                            return Ok(None);
                        }
                    }
                    BucketEntry::Metaentry(_) => {}
                }
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
