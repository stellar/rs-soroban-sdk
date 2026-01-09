/// Encoded marker for spec entries.
///
/// The marker is encoded as:
/// - 4-byte magic prefix: "SpEc" (alternating case to avoid false positives)
/// - 8-byte truncated SHA256 hash of the spec entry XDR bytes (64 bits)
// TODO: Move the spec marker logic into a crate that can be shared with the CLI.
use sha2::{Digest, Sha256};

/// Magic prefix for spec markers to avoid false positives when scanning the data section.
/// Uses alternating case "SpEc" for a distinctive pattern.
pub const SPEC_MARKER_MAGIC: &[u8; 4] = b"SpEc";

/// Length of the hash portion (truncated SHA256 - first 8 bytes / 64 bits).
pub const SPEC_MARKER_HASH_LEN: usize = 8;

/// Length of the marker: 4-byte prefix + 8-byte truncated SHA256 hash.
pub const SPEC_MARKER_LEN: usize = 4 + SPEC_MARKER_HASH_LEN;

/// Encodes a spec marker: magic prefix followed by truncated SHA256 hash of the spec XDR.
pub fn encode_spec_marker(spec_xdr: &[u8]) -> [u8; SPEC_MARKER_LEN] {
    let mut marker = [0u8; SPEC_MARKER_LEN];

    // Write magic prefix (4 bytes)
    marker[..4].copy_from_slice(SPEC_MARKER_MAGIC);

    // Write truncated SHA256 hash (first 16 bytes)
    let mut hasher = Sha256::new();
    hasher.update(spec_xdr);
    let hash: [u8; 32] = hasher.finalize().into();
    marker[4..].copy_from_slice(&hash[..SPEC_MARKER_HASH_LEN]);

    marker
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_spec_marker() {
        let spec_xdr = b"some spec xdr bytes";
        let marker = encode_spec_marker(spec_xdr);

        // Check length: 4-byte prefix + 8-byte hash = 12 bytes
        assert_eq!(marker.len(), SPEC_MARKER_LEN);
        assert_eq!(SPEC_MARKER_LEN, 12);

        // Check magic prefix
        assert_eq!(&marker[..4], b"SpEc");

        // Same input produces same marker
        let marker2 = encode_spec_marker(spec_xdr);
        assert_eq!(marker, marker2);

        // Different input produces different marker
        let different_xdr = b"different spec xdr bytes";
        let different_marker = encode_spec_marker(different_xdr);
        assert_eq!(&different_marker[..4], b"SpEc"); // Same prefix
        assert_ne!(&marker[4..], &different_marker[4..]); // Different hash
    }
}
