use std::{env, path::PathBuf};

/// Return an absolute path when given a relative path that is relative to the
/// Cargo manifest file.
///
/// If an absolute path is provided it is returned unaltered.
pub fn abs_from_rel_to_manifest(path: impl Into<PathBuf>) -> PathBuf {
    let path: PathBuf = path.into();
    if path.is_relative() {
        let root: PathBuf = env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable is required to be set")
            .into();
        root.join(path)
    } else {
        path
    }
}
