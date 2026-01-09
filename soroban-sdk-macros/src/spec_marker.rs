/// XDR-encoded marker for spec entries.
///
/// The marker is encoded as:
/// - 4-byte discriminant (0=struct, 1=union, 2=enum, 3=error, 4=event)
/// - XDR string for lib name
/// - XDR string for type/event name
///
/// XDR strings are: 4-byte big-endian length + bytes + padding to 4-byte boundary

/// Spec entry type discriminants.
pub const SPEC_MARKER_STRUCT: u32 = 0;
pub const SPEC_MARKER_UNION: u32 = 1;
pub const SPEC_MARKER_ENUM: u32 = 2;
pub const SPEC_MARKER_ERROR: u32 = 3;
pub const SPEC_MARKER_EVENT: u32 = 4;

/// Encodes a spec marker in XDR format.
pub fn encode_spec_marker(discriminant: u32, lib: &str, name: &str) -> Vec<u8> {
    let mut buf = Vec::new();

    // Write discriminant (4 bytes, big-endian)
    buf.extend_from_slice(&discriminant.to_be_bytes());

    // Write lib string
    write_xdr_string(&mut buf, lib);

    // Write name string
    write_xdr_string(&mut buf, name);

    buf
}

/// Writes an XDR string to the buffer.
/// XDR strings are: 4-byte big-endian length + bytes + padding to 4-byte boundary
fn write_xdr_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len() as u32;

    // Write length (4 bytes, big-endian)
    buf.extend_from_slice(&len.to_be_bytes());

    // Write bytes
    buf.extend_from_slice(bytes);

    // Add padding to 4-byte boundary
    let padding = (4 - (len % 4)) % 4;
    for _ in 0..padding {
        buf.push(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_spec_marker() {
        let marker = encode_spec_marker(SPEC_MARKER_STRUCT, "", "MyStruct");
        // discriminant: 0x00000000
        // lib length: 0x00000000
        // lib bytes: (none)
        // name length: 0x00000008
        // name bytes: "MyStruct"
        assert_eq!(
            marker,
            vec![
                0, 0, 0, 0, // discriminant = 0 (struct)
                0, 0, 0, 0, // lib length = 0
                0, 0, 0, 8, // name length = 8
                b'M', b'y', b'S', b't', b'r', b'u', b'c', b't', // "MyStruct"
            ]
        );
    }

    #[test]
    fn test_encode_spec_marker_with_lib() {
        let marker = encode_spec_marker(SPEC_MARKER_EVENT, "mylib", "Transfer");
        // discriminant: 0x00000004
        // lib length: 0x00000005
        // lib bytes: "mylib" + 3 bytes padding
        // name length: 0x00000008
        // name bytes: "Transfer"
        assert_eq!(
            marker,
            vec![
                0, 0, 0, 4, // discriminant = 4 (event)
                0, 0, 0, 5, // lib length = 5
                b'm', b'y', b'l', b'i', b'b', 0, 0, 0, // "mylib" + padding
                0, 0, 0, 8, // name length = 8
                b'T', b'r', b'a', b'n', b's', b'f', b'e', b'r', // "Transfer"
            ]
        );
    }
}
