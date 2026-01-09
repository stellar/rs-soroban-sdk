/// Encoded marker for spec entries.
///
/// The marker is encoded as:
/// - 4-byte magic prefix: "SpEc" (alternating case to avoid false positives)
/// - 1-byte discriminant (0=struct, 1=union, 2=enum, 3=error, 4=event)
/// - 1-byte lib name length + lib name bytes
/// - 1-byte type/event name length + name bytes

// TODO: Move the spec marker logic into a crate that can be shared with the CLI.

/// Magic prefix for spec markers to avoid false positives when scanning the data section.
/// Uses alternating case "SpEc" for a distinctive pattern.
pub const SPEC_MARKER_MAGIC: &[u8; 4] = b"SpEc";

/// Spec entry type discriminants.
pub const SPEC_MARKER_STRUCT: u8 = 0;
pub const SPEC_MARKER_UNION: u8 = 1;
pub const SPEC_MARKER_ENUM: u8 = 2;
pub const SPEC_MARKER_ERROR: u8 = 3;
pub const SPEC_MARKER_EVENT: u8 = 4;

/// Encodes a spec marker with a magic prefix.
pub fn encode_spec_marker(discriminant: u8, lib: &str, name: &str) -> Vec<u8> {
    let mut buf = Vec::new();

    // Write magic prefix (4 bytes)
    buf.extend_from_slice(SPEC_MARKER_MAGIC);

    // Write discriminant (1 byte)
    buf.push(discriminant);

    // Write lib string (1-byte length + bytes)
    buf.push(lib.len() as u8);
    buf.extend_from_slice(lib.as_bytes());

    // Write name string (1-byte length + bytes)
    buf.push(name.len() as u8);
    buf.extend_from_slice(name.as_bytes());

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_spec_marker() {
        let marker = encode_spec_marker(SPEC_MARKER_STRUCT, "", "MyStruct");
        // magic: "SpEc"
        // discriminant: 0 (struct)
        // lib length: 0
        // name length: 8
        // name bytes: "MyStruct"
        assert_eq!(
            marker,
            vec![
                b'S', b'p', b'E', b'c', // magic
                0,    // discriminant = struct
                0,    // lib length = 0
                8,    // name length = 8
                b'M', b'y', b'S', b't', b'r', b'u', b'c', b't', // "MyStruct"
            ]
        );
    }

    #[test]
    fn test_encode_spec_marker_with_lib() {
        let marker = encode_spec_marker(SPEC_MARKER_EVENT, "mylib", "Transfer");
        // magic: "SpEc"
        // discriminant: 4 (event)
        // lib length: 5
        // lib bytes: "mylib"
        // name length: 8
        // name bytes: "Transfer"
        assert_eq!(
            marker,
            vec![
                b'S', b'p', b'E', b'c', // magic
                4,    // discriminant = event
                5,    // lib length = 5
                b'm', b'y', b'l', b'i', b'b', // "mylib"
                8,    // name length = 8
                b'T', b'r', b'a', b'n', b's', b'f', b'e', b'r', // "Transfer"
            ]
        );
    }
}
