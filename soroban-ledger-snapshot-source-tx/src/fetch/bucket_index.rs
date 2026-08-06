//! Persistent offset index over a history-archive bucket file.
//!
//! History-archive buckets are append-only streams of framed `BucketEntry`
//! records with no random access: answering "what is the value of key K in
//! this bucket?" otherwise requires decoding every entry in the bucket, and a
//! checkpoint's bucket set can run to many gigabytes. This module builds a
//! small, sorted, fixed-width side index that maps a ledger key to the byte
//! offset of the frame that holds it, so a lookup becomes a binary search plus
//! a single frame decode.
//!
//! The index is derived purely from the bucket's bytes, and buckets are
//! content addressed by hash, so an index is valid forever for a given bucket
//! hash. It is therefore stored in the same network-scoped machine cache the
//! bucket itself lives in, keyed by the bucket hash and namespaced by both the
//! index format version and the XDR schema version.

use crate::cache::{cache, CacheError};
use crate::xdr_schema_version;
use sha2::{Digest, Sha256};
use soroban_sdk::xdr::{
    BucketEntry, Frame, LedgerEntry, LedgerKey, Limited, Limits, ReadXdr, WriteXdr,
};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Semantic version of the on-disk index format.
///
/// Bump whenever the header layout, record layout, sort order, or the meaning
/// of a record changes. It is part of the index path, so old indexes are
/// orphaned under their old directory rather than misread by newer code, and
/// it is also stored in the header so a file that somehow ends up at a v1 path
/// with v2 content is rejected instead of misparsed.
const INDEX_FORMAT_VERSION: u32 = 1;

/// Fixed 8-byte file magic, checked before anything else is trusted.
const MAGIC: [u8; 8] = *b"SBKTIDX\0";

/// Fixed-width record: 32-byte key hash, 8-byte big-endian frame offset,
/// 1-byte entry kind.
const RECORD_LEN: usize = 41;
const RECORD_HASH: std::ops::Range<usize> = 0..32;
const RECORD_OFFSET: std::ops::Range<usize> = 32..40;
const RECORD_KIND: usize = 40;

/// Header: magic(8) + format version(4) + record width(4) + record count(8) +
/// bucket hash(32).
const HEADER_LEN: usize = 56;

/// The entry exists in this bucket (`LIVEENTRY` or `INITENTRY`).
const KIND_PRESENT: u8 = 1;
/// The entry was deleted in this bucket (`DEADENTRY`), which shadows any
/// older bucket.
const KIND_DELETED: u8 = 2;

/// Number of buckets in one radix pass (one hex nibble).
const NIBBLES: usize = 16;
/// A SHA-256 hash is 32 bytes, i.e. 64 nibbles.
const HASH_NIBBLES: usize = 64;

/// Approximate byte budget for the in-memory phase of the external sort.
///
/// A shard no larger than this is sorted in memory; anything larger is split
/// by the next hash nibble and sorted recursively. This bounds peak memory
/// independently of bucket size.
const DEFAULT_SORT_MEMORY_BUDGET: usize = 64 * 1024 * 1024;

/// Read buffer used when streaming a bucket end to end.
const BUCKET_READ_BUF: usize = 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("xdr error: {0}")]
    Xdr(#[from] soroban_sdk::xdr::Error),
    #[error("bucket index cache error: {0}")]
    Cache(#[from] CacheError<Box<dyn std::error::Error>>),
    #[error("invalid bucket hash {0:?}: expected 64 hex characters")]
    InvalidBucketHash(String),
    /// The index file is structurally invalid: its header or overall size do
    /// not describe a well-formed index. Detected before any record is
    /// trusted, so the index can safely be discarded and rebuilt from the
    /// bucket (which is authoritative and content addressed). Escapes to the
    /// caller only when a rebuild has already been attempted.
    #[error("bucket index {path} is structurally invalid ({detail}) and a rebuild did not fix it")]
    IndexStructure { path: PathBuf, detail: String },
    /// The index and the bucket disagree about what lives at a recorded
    /// offset. This is never treated as "key absent", and is not retried:
    /// something has corrupted either file in a way that a rebuild cannot be
    /// trusted to resolve.
    #[error("bucket index {path} disagrees with bucket {bucket} ({detail})")]
    IndexContent {
        path: PathBuf,
        bucket: PathBuf,
        detail: String,
    },
    #[error("bucket index shard {0} has a size that is not a multiple of the record width")]
    MalformedShard(PathBuf),
}

/// Result of looking a key up in a single bucket.
///
/// Deliberately a three-state enum rather than a nested `Option<Option<_>>`:
/// "not mentioned in this bucket" (keep searching older buckets), "present"
/// and "deleted" (stop searching; the key does not exist) are three distinct
/// outcomes and encoding them as nested options makes the caller's match arms
/// unreadable and easy to get backwards.
// A boxed `LedgerEntry` would only move an allocation onto a value every
// caller immediately unwraps.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lookup {
    Absent,
    Present(LedgerEntry),
    Deleted,
}

/// Path of the persistent index for `bucket_hash` under the network-scoped
/// machine cache at `cache_path`.
///
/// Namespaced by the index format version and by the XDR schema version, so
/// that a change to either yields a fresh path rather than reinterpreting
/// bytes written by different code.
pub fn index_path(cache_path: &Path, bucket_hash: &str) -> PathBuf {
    cache_path
        .join(format!(
            "bucket-index-v{INDEX_FORMAT_VERSION}-xdr-{}",
            xdr_schema_version(),
        ))
        .join(format!("bucket-{bucket_hash}.idx"))
}

/// Look `key` up in the bucket at `bucket_path`, building the persistent index
/// on first use.
///
/// `bucket_path` must already contain the fully decompressed bucket.
pub fn lookup(
    cache_path: &Path,
    bucket_hash: &str,
    bucket_path: &Path,
    key: &LedgerKey,
) -> Result<Lookup, Error> {
    lookup_with_budget(
        cache_path,
        bucket_hash,
        bucket_path,
        key,
        DEFAULT_SORT_MEMORY_BUDGET,
    )
}

fn lookup_with_budget(
    cache_path: &Path,
    bucket_hash: &str,
    bucket_path: &Path,
    key: &LedgerKey,
    budget: usize,
) -> Result<Lookup, Error> {
    let hash = parse_bucket_hash(bucket_hash)?;
    let index_path = index_path(cache_path, bucket_hash);

    ensure_index(&index_path, bucket_path, bucket_hash, budget)?;
    match lookup_in_index(&index_path, bucket_path, &hash, key) {
        Err(Error::IndexStructure { detail, .. }) => {
            // Structural damage is detected purely from the header and file
            // length, before any record has been used to answer anything, so
            // discarding the file and rebuilding it from the (authoritative,
            // content-addressed) bucket cannot produce a wrong result. Exactly
            // one attempt: if the freshly built index is structurally invalid
            // too, the problem is not a stale/torn file and retrying forever
            // would only hide it.
            tracing::warn!(
                index = %index_path.display(),
                detail,
                "bucket index is structurally invalid; rebuilding once",
            );
            match fs::remove_file(&index_path) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.into()),
            }
            ensure_index(&index_path, bucket_path, bucket_hash, budget)?;
            lookup_in_index(&index_path, bucket_path, &hash, key)
        }
        other => other,
    }
}

fn parse_bucket_hash(bucket_hash: &str) -> Result<[u8; 32], Error> {
    let bytes =
        hex::decode(bucket_hash).map_err(|_| Error::InvalidBucketHash(bucket_hash.to_string()))?;
    bytes
        .try_into()
        .map_err(|_| Error::InvalidBucketHash(bucket_hash.to_string()))
}

/// Build the index if it is not already present.
///
/// Construction goes through [`cache`] so that concurrent builders are
/// serialized by its lock, the index is written to a temp file and atomically
/// renamed into place, and a failed build leaves neither a partial index nor a
/// stray `.dl` file behind.
fn ensure_index(
    index_path: &Path,
    bucket_path: &Path,
    bucket_hash: &str,
    budget: usize,
) -> Result<(), Error> {
    let hash = parse_bucket_hash(bucket_hash)?;
    let reader = cache(index_path, |write| {
        build_index(bucket_path, &hash, index_path, write, budget)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    })?;
    // `cache` hands back a reader for the finished file; the index is read
    // through a seekable handle instead, so drop it immediately.
    drop(reader);
    Ok(())
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

/// Stream `bucket_path` and write a complete sorted index to `out`.
///
/// Memory is bounded regardless of bucket size: records are never collected
/// into a single list. Instead they are streamed straight into 16 on-disk
/// shards keyed by the first hash nibble, and each shard is then sorted
/// recursively (see [`sort_shard`]).
fn build_index(
    bucket_path: &Path,
    bucket_hash: &[u8; 32],
    index_path: &Path,
    out: &mut dyn Write,
    budget: usize,
) -> Result<(), Error> {
    let budget = budget.max(RECORD_LEN);
    let scratch = ScratchDir::new(index_path)?;

    let count = shard_bucket(bucket_path, scratch.path(), budget)?;

    let mut out = BufWriter::with_capacity(BUCKET_READ_BUF, out);
    write_header(&mut out, count, bucket_hash)?;
    // Emitting shards in nibble order concatenates into globally sorted
    // output, because a shard's nibble is the most significant part of every
    // hash it holds.
    for nibble in 0..NIBBLES {
        sort_shard(scratch.path(), &nibble_hex(nibble), 1, budget, &mut out)?;
    }
    out.flush()?;
    Ok(())
}

fn write_header(out: &mut dyn Write, count: u64, bucket_hash: &[u8; 32]) -> Result<(), Error> {
    out.write_all(&MAGIC)?;
    out.write_all(&INDEX_FORMAT_VERSION.to_be_bytes())?;
    out.write_all(&(RECORD_LEN as u32).to_be_bytes())?;
    out.write_all(&count.to_be_bytes())?;
    out.write_all(bucket_hash)?;
    Ok(())
}

/// Decode every frame in the bucket, emitting one record per indexable entry
/// into the depth-0 shards. Returns the number of records emitted.
fn shard_bucket(bucket_path: &Path, dir: &Path, budget: usize) -> Result<u64, Error> {
    let file = File::open(bucket_path)?;
    let mut reader = Limited::new(
        CountingReader::new(BufReader::with_capacity(BUCKET_READ_BUF, file)),
        Limits::none(),
    );
    let mut writers = ShardWriters::new(dir, "", budget)?;
    let mut count: u64 = 0;
    let mut record = [0u8; RECORD_LEN];

    loop {
        // Peek before decoding: the XDR readers use `read_exact`, which cannot
        // distinguish a clean end of stream from a truncated entry, so a
        // successful zero-byte fill is the only reliable "no more frames"
        // signal.
        if reader.fill_buf()?.is_empty() {
            break;
        }
        // The offset of the 4-byte frame header, captured before the frame is
        // consumed, is what a lookup later seeks to.
        let offset = reader.inner.position();
        let Frame(entry) = Frame::<BucketEntry>::read_xdr(&mut reader)?;
        let (key, kind) = match entry {
            BucketEntry::Liveentry(entry) | BucketEntry::Initentry(entry) => {
                (entry.to_key(), KIND_PRESENT)
            }
            BucketEntry::Deadentry(key) => (key, KIND_DELETED),
            // Meta entries carry bucket-level metadata, not ledger entries.
            BucketEntry::Metaentry(_) => continue,
        };
        let hash = key_hash(&key)?;
        record[RECORD_HASH].copy_from_slice(&hash);
        record[RECORD_OFFSET].copy_from_slice(&offset.to_be_bytes());
        record[RECORD_KIND] = kind;
        writers.write(nibble_at(&hash, 0), &record)?;
        count += 1;
    }

    writers.finish()?;
    Ok(count)
}

/// Sort one shard and append it to `out`.
///
/// The shard holds exactly those records whose hash starts with the nibbles in
/// `prefix`, in ascending source-offset order (the streaming pass appends in
/// bucket order, and every partitioning pass preserves relative order).
fn sort_shard(
    dir: &Path,
    prefix: &str,
    depth: usize,
    budget: usize,
    out: &mut dyn Write,
) -> Result<(), Error> {
    let path = shard_path(dir, prefix);
    let len = match fs::metadata(&path) {
        Ok(m) => m.len(),
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e.into()),
    };
    if len % RECORD_LEN as u64 != 0 {
        return Err(Error::MalformedShard(path));
    }
    if len == 0 {
        fs::remove_file(&path)?;
        return Ok(());
    }

    if len <= budget as u64 {
        let mut reader = BufReader::with_capacity(BUCKET_READ_BUF, File::open(&path)?);
        let mut records = Vec::with_capacity((len / RECORD_LEN as u64) as usize);
        let mut record = [0u8; RECORD_LEN];
        for _ in 0..len / RECORD_LEN as u64 {
            reader.read_exact(&mut record)?;
            records.push(record);
        }
        drop(reader);
        fs::remove_file(&path)?;
        // Frame offsets are unique, so (hash, offset) is a total order and an
        // unstable sort is deterministic here.
        records.sort_unstable_by(|a, b| {
            a[RECORD_HASH]
                .cmp(&b[RECORD_HASH])
                .then_with(|| a[RECORD_OFFSET].cmp(&b[RECORD_OFFSET]))
        });
        for record in &records {
            out.write_all(record)?;
        }
        return Ok(());
    }

    if depth >= HASH_NIBBLES {
        // Every nibble of the hash has been consumed, so every record here has
        // the identical hash and there is nothing left to order by except the
        // source offset — which is already ascending. Copy straight through
        // rather than recursing forever or buying an unbounded in-memory sort.
        let mut reader = BufReader::with_capacity(BUCKET_READ_BUF, File::open(&path)?);
        io::copy(&mut reader, out)?;
        drop(reader);
        fs::remove_file(&path)?;
        return Ok(());
    }

    {
        // At most 16 writers are open at a time: they are all closed before
        // any recursive call is made.
        let mut writers = ShardWriters::new(dir, prefix, budget)?;
        let mut reader = BufReader::with_capacity(BUCKET_READ_BUF, File::open(&path)?);
        let mut record = [0u8; RECORD_LEN];
        for _ in 0..len / RECORD_LEN as u64 {
            reader.read_exact(&mut record)?;
            writers.write(nibble_at(&record[RECORD_HASH], depth), &record)?;
        }
        writers.finish()?;
    }
    // Reclaim the parent's disk space before descending.
    fs::remove_file(&path)?;

    for nibble in 0..NIBBLES {
        let child = format!("{prefix}{}", nibble_hex(nibble));
        sort_shard(dir, &child, depth + 1, budget, out)?;
    }
    Ok(())
}

/// The 16 shard writers for one partitioning pass.
struct ShardWriters {
    writers: Vec<BufWriter<File>>,
}

impl ShardWriters {
    fn new(dir: &Path, prefix: &str, budget: usize) -> Result<Self, Error> {
        // Share the budget across the open writers rather than adding to it.
        let capacity = (budget / (NIBBLES * 2)).clamp(4 * 1024, 256 * 1024);
        let mut writers = Vec::with_capacity(NIBBLES);
        for nibble in 0..NIBBLES {
            let path = shard_path(dir, &format!("{prefix}{}", nibble_hex(nibble)));
            writers.push(BufWriter::with_capacity(capacity, File::create(path)?));
        }
        Ok(Self { writers })
    }

    fn write(&mut self, nibble: usize, record: &[u8]) -> Result<(), Error> {
        self.writers[nibble].write_all(record)?;
        Ok(())
    }

    /// Flush explicitly: dropping a `BufWriter` silently discards write errors.
    fn finish(mut self) -> Result<(), Error> {
        for writer in &mut self.writers {
            writer.flush()?;
        }
        Ok(())
    }
}

fn shard_path(dir: &Path, prefix: &str) -> PathBuf {
    dir.join(format!("s{prefix}.shard"))
}

fn nibble_hex(nibble: usize) -> String {
    format!("{nibble:x}")
}

fn nibble_at(hash: &[u8], depth: usize) -> usize {
    let byte = hash[depth / 2];
    if depth.is_multiple_of(2) {
        (byte >> 4) as usize
    } else {
        (byte & 0x0f) as usize
    }
}

fn key_hash(key: &LedgerKey) -> Result<[u8; 32], Error> {
    Ok(Sha256::digest(key.to_xdr(Limits::none())?).into())
}

/// A scratch directory for the external sort, created next to the index being
/// built and removed on both success and failure.
struct ScratchDir(PathBuf);

impl ScratchDir {
    fn new(index_path: &Path) -> Result<Self, Error> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let parent = index_path.parent().unwrap_or(Path::new("."));
        let name = index_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "bucket".to_string());
        let dir = parent.join(format!("{name}.sort-{}-{n}", std::process::id()));
        fs::create_dir_all(&dir)?;
        Ok(Self(dir))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for ScratchDir {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.0) {
            if e.kind() != io::ErrorKind::NotFound {
                tracing::warn!(dir = %self.0.display(), error = %e, "failed to remove bucket index scratch dir");
            }
        }
    }
}

/// Wraps a reader and tracks how many bytes have been consumed from it.
///
/// The bucket is a pure forward stream, so a running byte count is an exact
/// logical position, and unlike `Seek::stream_position` it costs no syscall
/// per frame (buckets hold millions of frames).
struct CountingReader<R> {
    inner: R,
    position: u64,
}

impl<R> CountingReader<R> {
    fn new(inner: R) -> Self {
        Self { inner, position: 0 }
    }

    fn position(&self) -> u64 {
        self.position
    }
}

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.position += n as u64;
        Ok(n)
    }
}

impl<R: BufRead> BufRead for CountingReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
        self.position += amt as u64;
    }
}

// ---------------------------------------------------------------------------
// Lookup
// ---------------------------------------------------------------------------

struct IndexFile {
    file: File,
    count: u64,
}

fn open_index(path: &Path, bucket_hash: &[u8; 32]) -> Result<IndexFile, Error> {
    let structure = |detail: &str| Error::IndexStructure {
        path: path.to_path_buf(),
        detail: detail.to_string(),
    };

    let mut file = File::open(path)?;
    let file_len = file.metadata()?.len();
    if file_len < HEADER_LEN as u64 {
        return Err(structure("file is shorter than the header"));
    }
    let mut header = [0u8; HEADER_LEN];
    file.read_exact(&mut header)?;

    if header[0..8] != MAGIC {
        return Err(structure("bad magic"));
    }
    let version = u32::from_be_bytes(header[8..12].try_into().expect("4 bytes"));
    if version != INDEX_FORMAT_VERSION {
        return Err(structure(&format!(
            "format version {version} != {INDEX_FORMAT_VERSION}"
        )));
    }
    let record_len = u32::from_be_bytes(header[12..16].try_into().expect("4 bytes"));
    if record_len as usize != RECORD_LEN {
        return Err(structure(&format!(
            "record width {record_len} != {RECORD_LEN}"
        )));
    }
    let count = u64::from_be_bytes(header[16..24].try_into().expect("8 bytes"));
    if header[24..56] != bucket_hash[..] {
        return Err(structure("header bucket hash does not match this bucket"));
    }
    let expected = count
        .checked_mul(RECORD_LEN as u64)
        .and_then(|n| n.checked_add(HEADER_LEN as u64))
        .ok_or_else(|| structure("record count overflows the addressable file size"))?;
    if file_len != expected {
        return Err(structure(&format!(
            "file length {file_len} != expected {expected} for {count} records"
        )));
    }

    Ok(IndexFile { file, count })
}

impl IndexFile {
    fn record(&mut self, i: u64) -> Result<[u8; RECORD_LEN], Error> {
        self.file
            .seek(SeekFrom::Start(HEADER_LEN as u64 + i * RECORD_LEN as u64))?;
        let mut record = [0u8; RECORD_LEN];
        self.file.read_exact(&mut record)?;
        Ok(record)
    }

    /// Index of the first record whose hash is >= `hash`.
    fn lower_bound(&mut self, hash: &[u8; 32]) -> Result<u64, Error> {
        let (mut lo, mut hi) = (0u64, self.count);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let record = self.record(mid)?;
            if record[RECORD_HASH] < hash[..] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        Ok(lo)
    }
}

fn lookup_in_index(
    index_path: &Path,
    bucket_path: &Path,
    bucket_hash: &[u8; 32],
    key: &LedgerKey,
) -> Result<Lookup, Error> {
    let content = |detail: String| Error::IndexContent {
        path: index_path.to_path_buf(),
        bucket: bucket_path.to_path_buf(),
        detail,
    };

    let mut index = open_index(index_path, bucket_hash)?;
    let hash = key_hash(key)?;
    let mut i = index.lower_bound(&hash)?;
    let bucket = File::open(bucket_path)?;

    // Records sharing a hash are ordered by ascending source offset, so
    // scanning the run forward yields the first (oldest-written, i.e. lowest
    // offset) matching entry, which is the one the previous linear scan of the
    // bucket would have found.
    while i < index.count {
        let record = index.record(i)?;
        if record[RECORD_HASH] != hash[..] {
            break;
        }
        let offset = u64::from_be_bytes(record[RECORD_OFFSET].try_into().expect("8 bytes"));
        let kind = record[RECORD_KIND];

        let entry = decode_entry_at(&bucket, offset)
            .map_err(|e| content(format!("frame at offset {offset} failed to decode: {e}")))?;
        let (decoded_key, decoded_kind, decoded_entry) = match entry {
            BucketEntry::Liveentry(entry) | BucketEntry::Initentry(entry) => {
                (entry.to_key(), KIND_PRESENT, Some(entry))
            }
            BucketEntry::Deadentry(key) => (key, KIND_DELETED, None),
            BucketEntry::Metaentry(_) => {
                return Err(content(format!(
                    "record points at a meta entry at offset {offset}, which is never indexed"
                )))
            }
        };

        let decoded_hash = key_hash(&decoded_key)?;
        if decoded_hash != hash {
            // The offset does not point at the frame the record claims. That
            // is corruption of the index or the bucket, never evidence that
            // the key is absent.
            return Err(content(format!(
                "record hash {} does not match the entry at offset {offset} (hash {})",
                hex::encode(hash),
                hex::encode(decoded_hash),
            )));
        }
        if decoded_kind != kind {
            return Err(content(format!(
                "record kind {kind} does not match the entry at offset {offset} (kind {decoded_kind})"
            )));
        }
        if decoded_key != *key {
            // Same SHA-256, different key: a real (astronomically unlikely)
            // collision rather than corruption. Keep scanning the run.
            tracing::warn!(
                offset,
                "bucket index hash collision between distinct ledger keys; skipping",
            );
            i += 1;
            continue;
        }

        return Ok(match decoded_entry {
            Some(entry) => Lookup::Present(entry),
            None => Lookup::Deleted,
        });
    }

    Ok(Lookup::Absent)
}

fn decode_entry_at(bucket: &File, offset: u64) -> Result<BucketEntry, Error> {
    let mut reader = BufReader::with_capacity(64 * 1024, bucket);
    reader.seek(SeekFrom::Start(offset))?;
    let mut reader = Limited::new(reader, Limits::none());
    let Frame(entry) = Frame::<BucketEntry>::read_xdr(&mut reader)?;
    Ok(entry)
}

#[cfg(test)]
mod test {
    use super::{
        index_path, lookup_with_budget, sort_shard, CountingReader, Error, Lookup, HEADER_LEN,
        KIND_DELETED, KIND_PRESENT, MAGIC, RECORD_HASH, RECORD_KIND, RECORD_LEN, RECORD_OFFSET,
    };
    use sha2::{Digest, Sha256};
    use soroban_sdk::xdr::{
        AccountEntry, AccountEntryExt, AccountId, BucketEntry, BucketMetadata, BucketMetadataExt,
        Frame, LedgerEntry, LedgerEntryData, LedgerEntryExt, LedgerKey, LedgerKeyAccount, Limited,
        Limits, PublicKey, ReadXdr, SequenceNumber, String32, Thresholds, TtlEntry, Uint256, VecM,
        WriteXdr,
    };
    use std::fs::{self, File, OpenOptions};
    use std::io::{BufReader, Read, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU32, Ordering};

    /// Scratch directory for a test, kept inside the build output tree (never
    /// the system temp dir) and removed when the test finishes.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            static COUNTER: AtomicU32 = AtomicU32::new(0);
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../target/bucket-index-tests")
                .join(format!("{}-{name}-{n}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn account_key(byte: u8) -> LedgerKey {
        LedgerKey::Account(LedgerKeyAccount {
            account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([byte; 32]))),
        })
    }

    fn account_entry(byte: u8, seq: i64) -> LedgerEntry {
        LedgerEntry {
            last_modified_ledger_seq: seq as u32,
            data: LedgerEntryData::Account(AccountEntry {
                account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([byte; 32]))),
                balance: 100,
                seq_num: SequenceNumber(seq),
                num_sub_entries: 0,
                inflation_dest: None,
                flags: 0,
                home_domain: String32::default(),
                thresholds: Thresholds([0; 4]),
                signers: VecM::default(),
                ext: AccountEntryExt::V0,
            }),
            ext: LedgerEntryExt::V0,
        }
    }

    fn meta_entry() -> BucketEntry {
        BucketEntry::Metaentry(BucketMetadata {
            ledger_version: 20,
            ext: BucketMetadataExt::V0,
        })
    }

    fn ttl_entry(byte: u8) -> LedgerEntry {
        LedgerEntry {
            last_modified_ledger_seq: 1,
            data: LedgerEntryData::Ttl(TtlEntry {
                key_hash: soroban_sdk::xdr::Hash([byte; 32]),
                live_until_ledger_seq: 100,
            }),
            ext: LedgerEntryExt::V0,
        }
    }

    /// Manually frame a `BucketEntry` the way the archive does: a 4-byte
    /// record marker with the last-fragment bit set, followed by the body.
    /// `Frame<BucketEntry>` only implements `ReadXdr`, so tests must write the
    /// header themselves.
    fn frame_bytes(entry: &BucketEntry) -> Vec<u8> {
        let body = entry.to_xdr(Limits::none()).unwrap();
        let mut out = ((body.len() as u32) | 0x8000_0000).to_be_bytes().to_vec();
        out.extend_from_slice(&body);
        out
    }

    /// Write a synthetic bucket file, returning the frame offsets and the hex
    /// content hash used to address it.
    fn write_bucket(dir: &Path, entries: &[BucketEntry]) -> (PathBuf, Vec<u64>, String) {
        let mut bytes = Vec::new();
        let mut offsets = Vec::new();
        for entry in entries {
            offsets.push(bytes.len() as u64);
            bytes.extend_from_slice(&frame_bytes(entry));
        }
        let hash = hex::encode(Sha256::digest(&bytes));
        let path = dir.join(format!("bucket-{hash}.xdr"));
        fs::write(&path, &bytes).unwrap();
        (path, offsets, hash)
    }

    fn read_index(path: &Path) -> Vec<u8> {
        fs::read(path).unwrap()
    }

    fn records(index: &[u8]) -> Vec<&[u8]> {
        index[HEADER_LEN..].chunks_exact(RECORD_LEN).collect()
    }

    const TINY_BUDGET: usize = RECORD_LEN * 2;

    #[test]
    fn live_init_dead_meta_and_absent_keys() {
        let dir = TempDir::new("kinds");
        let live = account_entry(1, 10);
        let init = account_entry(2, 20);
        let entries = vec![
            meta_entry(),
            BucketEntry::Liveentry(live.clone()),
            BucketEntry::Initentry(init.clone()),
            BucketEntry::Deadentry(account_key(3)),
        ];
        let (bucket, _, hash) = write_bucket(dir.path(), &entries);

        let got = |key: &LedgerKey| {
            lookup_with_budget(
                dir.path(),
                &hash,
                &bucket,
                key,
                super::DEFAULT_SORT_MEMORY_BUDGET,
            )
            .unwrap()
        };

        assert_eq!(got(&account_key(1)), Lookup::Present(live));
        assert_eq!(got(&account_key(2)), Lookup::Present(init));
        assert_eq!(got(&account_key(3)), Lookup::Deleted);
        assert_eq!(got(&account_key(9)), Lookup::Absent);
    }

    #[test]
    fn meta_entries_are_not_indexed() {
        let dir = TempDir::new("meta");
        let entries = vec![
            meta_entry(),
            meta_entry(),
            BucketEntry::Liveentry(account_entry(1, 1)),
        ];
        let (bucket, _, hash) = write_bucket(dir.path(), &entries);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap();

        let index = read_index(&index_path(dir.path(), &hash));
        assert_eq!(
            records(&index).len(),
            1,
            "only the single live entry should be indexed"
        );
    }

    #[test]
    fn stored_offsets_decode_the_intended_frame() {
        let dir = TempDir::new("offsets");
        // Entries of deliberately different encoded sizes, so an off-by-one in
        // offset tracking cannot accidentally still land on a frame boundary.
        let entries = vec![
            meta_entry(),
            BucketEntry::Liveentry(ttl_entry(7)),
            BucketEntry::Liveentry(account_entry(1, 1)),
            BucketEntry::Deadentry(account_key(2)),
            BucketEntry::Initentry(account_entry(3, 3)),
        ];
        let (bucket, offsets, hash) = write_bucket(dir.path(), &entries);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap();

        let index = read_index(&index_path(dir.path(), &hash));
        let file = File::open(&bucket).unwrap();
        for record in records(&index) {
            let offset = u64::from_be_bytes(record[RECORD_OFFSET].try_into().unwrap());
            assert!(
                offsets.contains(&offset),
                "offset {offset} is not a frame boundary; boundaries are {offsets:?}",
            );
            let decoded = super::decode_entry_at(&file, offset).unwrap();
            let key = match &decoded {
                BucketEntry::Liveentry(e) | BucketEntry::Initentry(e) => e.to_key(),
                BucketEntry::Deadentry(k) => k.clone(),
                BucketEntry::Metaentry(_) => panic!("meta entries are never indexed"),
            };
            assert_eq!(
                Sha256::digest(key.to_xdr(Limits::none()).unwrap()).as_slice(),
                &record[RECORD_HASH],
                "the frame at the stored offset must be the one the record describes",
            );
        }
    }

    #[test]
    fn records_are_sorted_and_binary_search_finds_every_key() {
        let dir = TempDir::new("sorted");
        let entries: Vec<BucketEntry> = (0u8..64)
            .map(|i| BucketEntry::Liveentry(account_entry(i, i as i64)))
            .collect();
        let (bucket, _, hash) = write_bucket(dir.path(), &entries);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(0), TINY_BUDGET).unwrap();

        let index = read_index(&index_path(dir.path(), &hash));
        let records = records(&index);
        assert_eq!(records.len(), 64);
        for pair in records.windows(2) {
            assert!(
                pair[0][RECORD_HASH] <= pair[1][RECORD_HASH],
                "index records must be sorted by key hash",
            );
        }

        for i in 0u8..64 {
            assert_eq!(
                lookup_with_budget(dir.path(), &hash, &bucket, &account_key(i), TINY_BUDGET)
                    .unwrap(),
                Lookup::Present(account_entry(i, i as i64)),
            );
        }
        assert_eq!(
            lookup_with_budget(dir.path(), &hash, &bucket, &account_key(200), TINY_BUDGET).unwrap(),
            Lookup::Absent,
        );
    }

    #[test]
    fn duplicate_keys_resolve_to_the_lowest_offset() {
        let dir = TempDir::new("dupes");
        let first = account_entry(1, 10);
        let second = account_entry(1, 20);
        let third = account_entry(1, 30);
        let entries = vec![
            BucketEntry::Liveentry(first.clone()),
            BucketEntry::Liveentry(second),
            BucketEntry::Liveentry(third),
        ];
        let (bucket, offsets, hash) = write_bucket(dir.path(), &entries);

        assert_eq!(
            lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap(),
            Lookup::Present(first),
            "the first matching entry by ascending source offset must win",
        );

        // The duplicates must also be stored in ascending offset order.
        let index = read_index(&index_path(dir.path(), &hash));
        let stored: Vec<u64> = records(&index)
            .iter()
            .map(|r| u64::from_be_bytes(r[RECORD_OFFSET].try_into().unwrap()))
            .collect();
        assert_eq!(stored, offsets);
    }

    #[test]
    fn tiny_memory_budget_forces_recursive_sharding() {
        let dir = TempDir::new("shard");
        let entries: Vec<BucketEntry> = (0u8..40)
            .map(|i| BucketEntry::Liveentry(account_entry(i, i as i64)))
            .collect();
        let (bucket, _, hash) = write_bucket(dir.path(), &entries);

        // One record per shard file forces the sort to recurse well past the
        // first nibble for any hashes sharing a prefix.
        assert_eq!(
            lookup_with_budget(dir.path(), &hash, &bucket, &account_key(5), RECORD_LEN).unwrap(),
            Lookup::Present(account_entry(5, 5)),
        );

        let index_path = index_path(dir.path(), &hash);
        let index = read_index(&index_path);
        let records = records(&index);
        assert_eq!(records.len(), 40);
        for pair in records.windows(2) {
            assert!(pair[0][RECORD_HASH] <= pair[1][RECORD_HASH]);
        }

        // The scratch directory used by the external sort must be gone.
        let leftovers: Vec<_> = fs::read_dir(index_path.parent().unwrap())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|n| n.contains(".sort-"))
            .collect();
        assert!(
            leftovers.is_empty(),
            "scratch dirs left behind: {leftovers:?}"
        );
    }

    #[test]
    fn deep_recursion_preserves_order_for_identical_hashes() {
        // Many records with the identical key hash exercise the depth-64 path
        // where no nibble is left to shard on.
        let dir = TempDir::new("identical");
        let entries: Vec<BucketEntry> = (0..20)
            .map(|i| BucketEntry::Liveentry(account_entry(1, i)))
            .collect();
        let (bucket, offsets, hash) = write_bucket(dir.path(), &entries);

        assert_eq!(
            lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), RECORD_LEN).unwrap(),
            Lookup::Present(account_entry(1, 0)),
        );
        let index = read_index(&index_path(dir.path(), &hash));
        let stored: Vec<u64> = records(&index)
            .iter()
            .map(|r| u64::from_be_bytes(r[RECORD_OFFSET].try_into().unwrap()))
            .collect();
        assert_eq!(stored, offsets, "identical hashes stay in source order");
    }

    #[test]
    fn corrupt_magic_is_rebuilt_once_and_still_answers_correctly() {
        let dir = TempDir::new("magic");
        let live = account_entry(1, 1);
        let entries = vec![BucketEntry::Liveentry(live.clone())];
        let (bucket, _, hash) = write_bucket(dir.path(), &entries);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap();

        let index_path = index_path(dir.path(), &hash);
        let mut file = OpenOptions::new().write(true).open(&index_path).unwrap();
        file.write_all(b"XXXXXXXX").unwrap();
        drop(file);

        assert_eq!(
            lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap(),
            Lookup::Present(live),
        );
        assert_eq!(&read_index(&index_path)[0..8], &MAGIC);
    }

    #[test]
    fn truncated_index_is_rebuilt_once_and_still_answers_correctly() {
        let dir = TempDir::new("truncated-index");
        let live = account_entry(1, 1);
        let entries = vec![
            BucketEntry::Liveentry(live.clone()),
            BucketEntry::Liveentry(account_entry(2, 2)),
        ];
        let (bucket, _, hash) = write_bucket(dir.path(), &entries);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap();

        let index_path = index_path(dir.path(), &hash);
        let len = fs::metadata(&index_path).unwrap().len();
        OpenOptions::new()
            .write(true)
            .open(&index_path)
            .unwrap()
            .set_len(len - 1)
            .unwrap();

        assert_eq!(
            lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap(),
            Lookup::Present(live),
        );
        assert_eq!(fs::metadata(&index_path).unwrap().len(), len);
    }

    #[test]
    fn wrong_bucket_hash_in_header_is_rebuilt() {
        let dir = TempDir::new("wrong-hash");
        let live = account_entry(1, 1);
        let (bucket, _, hash) = write_bucket(dir.path(), &[BucketEntry::Liveentry(live.clone())]);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap();

        let index_path = index_path(dir.path(), &hash);
        let mut file = OpenOptions::new().write(true).open(&index_path).unwrap();
        file.seek(SeekFrom::Start(24)).unwrap();
        file.write_all(&[0xab; 32]).unwrap();
        drop(file);

        assert_eq!(
            lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap(),
            Lookup::Present(live),
        );
        let rebuilt = read_index(&index_path);
        assert_eq!(&rebuilt[24..56], &hex::decode(&hash).unwrap()[..]);
    }

    #[test]
    fn corrupt_offset_is_a_hard_error_not_an_absent_key() {
        let dir = TempDir::new("bad-offset");
        let entries = vec![
            BucketEntry::Liveentry(account_entry(1, 1)),
            BucketEntry::Liveentry(account_entry(2, 2)),
        ];
        let (bucket, offsets, hash) = write_bucket(dir.path(), &entries);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap();

        let index_path = index_path(dir.path(), &hash);
        let pristine = read_index(&index_path);
        // Locate the record for key 1 by its hash; index order is by hash, not
        // by bucket order.
        let key_hash = Sha256::digest(account_key(1).to_xdr(Limits::none()).unwrap());
        let record_start = HEADER_LEN
            + RECORD_LEN
                * records(&pristine)
                    .iter()
                    .position(|r| r[RECORD_HASH] == key_hash[..])
                    .expect("key 1 must be indexed");

        // Case 1: the offset points at a different, perfectly valid frame.
        let mut index = pristine.clone();
        index[record_start + RECORD_OFFSET.start..record_start + RECORD_OFFSET.end]
            .copy_from_slice(&offsets[1].to_be_bytes());
        fs::write(&index_path, &index).unwrap();
        let err = lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET)
            .expect_err("corruption must not be reported as an absent key");
        assert!(
            matches!(err, Error::IndexContent { .. }),
            "expected a typed content error, got {err:?}",
        );
        // The corrupt index must not have been silently replaced.
        assert_eq!(read_index(&index_path), index);

        // Case 2: the offset points into the middle of a frame, so the decode
        // itself fails. That is still corruption, not an absent key.
        let mut index = pristine;
        index[record_start + RECORD_OFFSET.start..record_start + RECORD_OFFSET.end]
            .copy_from_slice(&3u64.to_be_bytes());
        fs::write(&index_path, &index).unwrap();
        let err = lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET)
            .expect_err("an undecodable offset must not be reported as an absent key");
        assert!(
            matches!(err, Error::IndexContent { .. }),
            "expected a typed content error, got {err:?}",
        );
    }

    #[test]
    fn corrupt_kind_is_a_hard_error() {
        let dir = TempDir::new("bad-kind");
        let (bucket, _, hash) =
            write_bucket(dir.path(), &[BucketEntry::Liveentry(account_entry(1, 1))]);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap();

        let index_path = index_path(dir.path(), &hash);
        let mut index = read_index(&index_path);
        assert_eq!(index[HEADER_LEN + RECORD_KIND], KIND_PRESENT);
        index[HEADER_LEN + RECORD_KIND] = KIND_DELETED;
        fs::write(&index_path, &index).unwrap();

        let err = lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET)
            .expect_err("a kind mismatch must not be reported as a deletion");
        assert!(
            matches!(err, Error::IndexContent { .. }),
            "expected a typed content error, got {err:?}",
        );
    }

    #[test]
    fn truncated_bucket_fails_the_build_and_leaves_no_index_behind() {
        let dir = TempDir::new("truncated-bucket");
        let (bucket, _, hash) = write_bucket(
            dir.path(),
            &[
                BucketEntry::Liveentry(account_entry(1, 1)),
                BucketEntry::Liveentry(account_entry(2, 2)),
            ],
        );
        let len = fs::metadata(&bucket).unwrap().len();
        OpenOptions::new()
            .write(true)
            .open(&bucket)
            .unwrap()
            .set_len(len - 4)
            .unwrap();

        let err = lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET)
            .expect_err("a truncated bucket must fail the index build");
        assert!(
            matches!(err, Error::Cache(_)),
            "expected the build failure to surface through the cache, got {err:?}",
        );

        let index_path = index_path(dir.path(), &hash);
        assert!(!index_path.exists(), "no index file may be left behind");
        assert!(
            !index_path.with_extension("dl").exists(),
            "no partial download file may be left behind",
        );
        let leftovers: Vec<_> = fs::read_dir(index_path.parent().unwrap())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|n| n.contains(".sort-"))
            .collect();
        assert!(
            leftovers.is_empty(),
            "scratch dirs left behind: {leftovers:?}"
        );
    }

    #[test]
    fn index_paths_separate_by_version_schema_network_and_bucket() {
        let mainnet = Path::new("/cache/mainnet");
        let testnet = Path::new("/cache/testnet");
        let a = "a".repeat(64);
        let b = "b".repeat(64);

        let path = index_path(mainnet, &a);
        assert_eq!(
            path,
            mainnet
                .join(format!(
                    "bucket-index-v{}-xdr-{}",
                    super::INDEX_FORMAT_VERSION,
                    crate::xdr_schema_version(),
                ))
                .join(format!("bucket-{a}.idx")),
        );
        assert_ne!(path, index_path(testnet, &a), "networks must not collide");
        assert_ne!(path, index_path(mainnet, &b), "buckets must not collide");
    }

    #[test]
    fn empty_bucket_yields_an_empty_index_and_absent_lookups() {
        let dir = TempDir::new("empty");
        let (bucket, _, hash) = write_bucket(dir.path(), &[]);
        assert_eq!(
            lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap(),
            Lookup::Absent,
        );
        assert_eq!(read_index(&index_path(dir.path(), &hash)).len(), HEADER_LEN);
    }

    #[test]
    fn counting_reader_tracks_frame_boundaries() {
        let entries = vec![
            BucketEntry::Liveentry(ttl_entry(1)),
            BucketEntry::Liveentry(account_entry(2, 2)),
        ];
        let mut bytes = Vec::new();
        let mut offsets = Vec::new();
        for entry in &entries {
            offsets.push(bytes.len() as u64);
            bytes.extend_from_slice(&frame_bytes(entry));
        }

        let mut reader = Limited::new(
            CountingReader::new(BufReader::new(std::io::Cursor::new(bytes.clone()))),
            Limits::none(),
        );
        for (i, want) in offsets.iter().enumerate() {
            assert_eq!(reader.inner.position(), *want, "frame {i}");
            let Frame(entry) = Frame::<BucketEntry>::read_xdr(&mut reader).unwrap();
            assert_eq!(entry, entries[i]);
        }
        assert_eq!(reader.inner.position(), bytes.len() as u64);
    }

    #[test]
    fn malformed_shard_size_is_rejected() {
        let dir = TempDir::new("malformed-shard");
        fs::write(dir.path().join("s0.shard"), [0u8; RECORD_LEN + 1]).unwrap();
        let mut out = Vec::new();
        let err = sort_shard(dir.path(), "0", 1, RECORD_LEN * 8, &mut out)
            .expect_err("a shard that is not a whole number of records must be rejected");
        assert!(matches!(err, Error::MalformedShard(_)), "got {err:?}");
    }

    #[test]
    fn index_is_reused_on_a_second_lookup() {
        let dir = TempDir::new("reuse");
        let live = account_entry(1, 1);
        let (bucket, _, hash) = write_bucket(dir.path(), &[BucketEntry::Liveentry(live.clone())]);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap();

        // A second lookup must hit the existing index rather than rebuilding
        // it; `cache` only invokes the collector when the file is missing, so
        // an unchanged file is the observable signal.
        let index_path = index_path(dir.path(), &hash);
        let before = fs::metadata(&index_path).unwrap().len();
        assert_eq!(
            lookup_with_budget(dir.path(), &hash, &bucket, &account_key(1), TINY_BUDGET).unwrap(),
            Lookup::Present(live),
        );
        assert_eq!(fs::metadata(&index_path).unwrap().len(), before);
    }

    #[test]
    fn index_header_records_count_and_width() {
        let dir = TempDir::new("header");
        let entries: Vec<BucketEntry> = (0u8..5)
            .map(|i| BucketEntry::Liveentry(account_entry(i, i as i64)))
            .collect();
        let (bucket, _, hash) = write_bucket(dir.path(), &entries);
        lookup_with_budget(dir.path(), &hash, &bucket, &account_key(0), TINY_BUDGET).unwrap();

        let mut file = File::open(index_path(dir.path(), &hash)).unwrap();
        let mut header = [0u8; HEADER_LEN];
        file.read_exact(&mut header).unwrap();
        assert_eq!(&header[0..8], &MAGIC);
        assert_eq!(
            u32::from_be_bytes(header[8..12].try_into().unwrap()),
            super::INDEX_FORMAT_VERSION
        );
        assert_eq!(
            u32::from_be_bytes(header[12..16].try_into().unwrap()),
            RECORD_LEN as u32
        );
        assert_eq!(u64::from_be_bytes(header[16..24].try_into().unwrap()), 5);
        assert_eq!(&header[24..56], &hex::decode(&hash).unwrap()[..]);
    }
}
