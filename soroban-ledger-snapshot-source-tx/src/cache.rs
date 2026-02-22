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
