use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use fs2::FileExt;

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
        lock_file.lock_exclusive().map_err(CacheError::Io)?;

        if !path.exists() {
            // Write atomically via temp file and rename.
            let temp_path = path.with_extension("dl");
            {
                let mut temp = File::create(&temp_path).map_err(CacheError::Io)?;
                collect(&mut temp).map_err(CacheError::Collector)?;
                temp.sync_all().map_err(CacheError::Io)?;
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
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A self-cleaning unique temp directory (avoids pulling in a dev-dependency).
    struct TempDir(std::path::PathBuf);

    impl TempDir {
        fn new() -> Self {
            let n = COUNTER.fetch_add(1, Ordering::SeqCst);
            let p = std::env::temp_dir().join(format!("sst-cache-test-{}-{n}", std::process::id()));
            fs::create_dir_all(&p).unwrap();
            TempDir(p)
        }

        fn join(&self, name: &str) -> std::path::PathBuf {
            self.0.join(name)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn cache_miss_invokes_collector_and_persists() {
        let dir = TempDir::new();
        let path = dir.join("entry");
        let mut reader = cache(&path, |w| -> Result<(), std::io::Error> {
            w.write_all(b"hello")?;
            Ok(())
        })
        .unwrap();
        let mut s = String::new();
        reader.read_to_string(&mut s).unwrap();
        assert_eq!(s, "hello");
        // The data file is persisted with the collected bytes.
        assert_eq!(fs::read(&path).unwrap(), b"hello");
    }

    #[test]
    fn cache_hit_does_not_invoke_collector() {
        let dir = TempDir::new();
        let path = dir.join("entry");
        fs::write(&path, b"existing").unwrap();
        let mut reader = cache(&path, |_w| -> Result<(), std::io::Error> {
            panic!("collector must not run on a cache hit");
        })
        .unwrap();
        let mut s = String::new();
        reader.read_to_string(&mut s).unwrap();
        assert_eq!(s, "existing");
    }

    #[test]
    fn cache_collector_error_does_not_persist() {
        let dir = TempDir::new();
        let path = dir.join("entry");
        let res = cache(&path, |_w| -> Result<(), std::io::Error> {
            Err(std::io::Error::other("boom"))
        });
        // A failed fetch surfaces as a collector error...
        assert!(matches!(res.err(), Some(CacheError::Collector(_))));
        // ...and must NOT leave a populated cache file (rename never happens).
        assert!(!path.exists());
    }
}
