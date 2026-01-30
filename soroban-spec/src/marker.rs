/// Spec marker generation for identifying used spec entries.
///
/// The marker is a byte array in the data section with a distinctive pattern:
/// - 4 bytes: "SpEc" prefix
/// - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
///
/// Markers are embedded in conversion/usage functions with a volatile read. When the type is used,
/// the function is called and the marker is included. When the type is unused, the function is
/// DCE'd along with its marker.
///
/// Post-processing tools (e.g. stellar-cli) can:
/// 1. Scan the WASM data section for "SpEc" patterns
/// 2. Extract the hash from each marker
/// 3. Match against specs in contractspecv0 section (by hashing each spec)
/// 4. Strip unused specs from contractspecv0
///
/// Today markers are only used in contracts written in Rust, leveraging how Rust can eliminate
/// dead code to make the markers are good signal for if a type gets used. It's not known if the
/// same pattern could be used in other languages, and so it is not a general part of the SEP-48
/// Contact Interface Specification. Markers are just a mechanism used by the Rust soroban-sdk and
/// the stellar-cli to achieve accurately scoped contract specs.
use sha2::{Digest, Sha256};

/// Total length of a spec marker (4-byte prefix + 8-byte hash).
pub const LEN: usize = 12;

/// A spec marker that identifies a spec entry.
///
/// Format: "SpEc" prefix (4 bytes) + first 8 bytes of SHA256 hash = 12 bytes total.
pub type SpecMarker = [u8; LEN];

/// Generates a spec marker for a spec entry XDR.
pub fn generate(spec_entry_xdr: &[u8]) -> SpecMarker {
    let hash: [u8; 32] = Sha256::digest(spec_entry_xdr).into();
    [
        b'S', b'p', b'E', b'c', hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6],
        hash[7],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let spec_xdr = b"some spec xdr bytes";
        let marker: SpecMarker = generate(spec_xdr);

        // Check prefix
        assert_eq!(&marker[..4], b"SpEc");

        // Check total length
        assert_eq!(marker.len(), LEN);
        assert_eq!(marker.len(), 12);

        // Same input produces same marker
        let marker2 = generate(spec_xdr);
        assert_eq!(marker, marker2);

        // Different input produces different marker
        let different_xdr = b"different spec xdr bytes";
        let different_marker = generate(different_xdr);
        assert_eq!(&different_marker[..4], b"SpEc");
        assert_ne!(marker, different_marker);
    }
}
