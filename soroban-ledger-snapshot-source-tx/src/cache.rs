use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

/// Cache a file, collecting bytes if not present, and return a reader
pub fn cache<P, C, CE>(path: P, collect: C) -> Result<impl Read, CacheError<CE>>
where
    P: AsRef<Path>,
    C: FnOnce(&mut dyn Write) -> Result<(), CE>,
{
    let path = path.as_ref();

    // Fast path: if file already exists, just read it (no lock needed).
    // Slow path: acquire lock, check again, collect and write if needed.
    if !path.exists() {
        let lock_path = path.with_extension("lock");
        let lock_file = File::create(&lock_path).map_err(CacheError::Io)?;
        // Acquire an exclusive advisory lock using std's native file locking
        // (stable since Rust 1.89), released automatically when `lock_file` is
        // dropped at the end of this scope.
        File::lock(&lock_file).map_err(CacheError::Io)?;

        if !path.exists() {
            // Write atomically via temp file and rename.
            let temp_path = path.with_extension("dl");
            let write_result = (|| {
                let mut temp = File::create(&temp_path).map_err(CacheError::Io)?;
                collect(&mut temp).map_err(CacheError::Collector)?;
                temp.sync_all().map_err(CacheError::Io)?;
                Ok(())
            })();
            if let Err(e) = write_result {
                // Don't leave a partial temp file behind on failure; otherwise a
                // transient collector/IO error would litter the cache directory
                // (and potentially the committed fixtures) with stray `.dl` files.
                let _ = fs::remove_file(&temp_path);
                return Err(e);
            }
            fs::rename(&temp_path, path).map_err(CacheError::Io)?;
        }
    }

    // Open data file for reading.
    let file = File::open(path).map_err(CacheError::Io)?;
    Ok(file)
}

/// Error type for cache operations
#[derive(Debug, thiserror::Error)]
pub enum CacheError<CE> {
    #[error("collector error: {0}")]
    Collector(CE),
    #[error("io error: {0}")]
    #[allow(dead_code)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod test {
    use super::{cache, CacheError};
    use std::io::Read;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    // Unique temp path per test invocation to avoid cross-test interference.
    fn unique_path(name: &str) -> PathBuf {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "soroban-cache-test-{}-{}-{}",
            std::process::id(),
            name,
            n
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("entry.json")
    }

    fn read_all(mut r: impl Read) -> Vec<u8> {
        let mut buf = Vec::new();
        r.read_to_end(&mut buf).unwrap();
        buf
    }

    #[test]
    fn miss_then_write_collects_and_returns_bytes() {
        let path = unique_path("miss");
        let reader = cache(&path, |w| {
            w.write_all(b"hello")?;
            Ok::<(), std::io::Error>(())
        })
        .unwrap();
        assert_eq!(read_all(reader), b"hello");
        assert!(path.exists());
        // The atomic-write temp file must not be left behind.
        assert!(!path.with_extension("dl").exists());
    }

    #[test]
    fn hit_uses_fast_path_without_invoking_collector() {
        let path = unique_path("hit");
        std::fs::write(&path, b"cached").unwrap();
        // Collector must not run when the file already exists.
        let reader = cache(&path, |_w| -> Result<(), std::io::Error> {
            panic!("collector should not be called on cache hit");
        })
        .unwrap();
        assert_eq!(read_all(reader), b"cached");
    }

    #[test]
    fn collector_error_is_propagated_and_no_data_file_written() {
        let path = unique_path("err");
        let result = cache(&path, |_w| {
            Err::<(), std::io::Error>(std::io::Error::other("nope"))
        });
        match result {
            Err(CacheError::Collector(e)) => assert_eq!(e.to_string(), "nope"),
            _ => panic!("expected Collector error"),
        }
        // On collector failure the data file must not be created (the rename
        // only happens after a successful collect).
        assert!(!path.exists());
        // The partial temp file must be cleaned up rather than left behind.
        assert!(!path.with_extension("dl").exists());
    }
}
