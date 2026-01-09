/// Spec marker generation for identifying used spec entries.
///
/// The marker is a byte array in the data section with a distinctive pattern:
/// - 4 bytes: "SpEc" prefix
/// - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
///
/// Markers are embedded in conversion/usage functions with a volatile read.
/// When the type is used, the function is called and the marker is included.
/// When the type is unused, the function is DCE'd along with its marker.
///
/// Post-processing tools can:
/// 1. Scan the WASM data section for "SpEc" patterns
/// 2. Extract the hash from each marker
/// 3. Match against specs in contractspecv0 section (by hashing each spec)
/// 4. Strip unused specs from contractspecv0
// TODO: Move the spec marker logic into a crate that can be shared with the CLI.
use sha2::{Digest, Sha256};

/// Prefix for spec markers.
pub const SPEC_MARKER_PREFIX: &[u8; 4] = b"SpEc";

/// Length of the hash portion (truncated SHA256 - first 8 bytes / 64 bits).
pub const SPEC_MARKER_HASH_LEN: usize = 8;

/// Total length of a spec marker.
pub const SPEC_MARKER_LEN: usize = SPEC_MARKER_PREFIX.len() + SPEC_MARKER_HASH_LEN;

/// Computes the hash portion of the spec marker (8 bytes / 64 bits).
pub fn spec_marker_hash(spec_xdr: &[u8]) -> [u8; SPEC_MARKER_HASH_LEN] {
    let hash: [u8; 32] = Sha256::digest(spec_xdr).into();
    let mut result = [0u8; SPEC_MARKER_HASH_LEN];
    result.copy_from_slice(&hash[..SPEC_MARKER_HASH_LEN]);
    result
}

/// Generates the full spec marker as a byte array.
/// Format: "SpEc" + 8 bytes of SHA256 hash = 12 bytes total.
pub fn spec_marker(spec_xdr: &[u8]) -> [u8; SPEC_MARKER_LEN] {
    let hash = spec_marker_hash(spec_xdr);
    let mut result = [0u8; SPEC_MARKER_LEN];
    result[..SPEC_MARKER_PREFIX.len()].copy_from_slice(SPEC_MARKER_PREFIX);
    result[SPEC_MARKER_PREFIX.len()..].copy_from_slice(&hash);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_marker() {
        let spec_xdr = b"some spec xdr bytes";
        let marker = spec_marker(spec_xdr);

        // Check prefix
        assert_eq!(&marker[..4], b"SpEc");

        // Check total length
        assert_eq!(marker.len(), SPEC_MARKER_LEN);
        assert_eq!(marker.len(), 12);

        // Same input produces same marker
        let marker2 = spec_marker(spec_xdr);
        assert_eq!(marker, marker2);

        // Different input produces different marker
        let different_xdr = b"different spec xdr bytes";
        let different_marker = spec_marker(different_xdr);
        assert_eq!(&different_marker[..4], b"SpEc");
        assert_ne!(marker, different_marker);
    }
}
