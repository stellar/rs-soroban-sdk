//! Minimal wasm custom section reading and rewriting.
//!
//! Only enough of the binary format is handled to read the custom sections a
//! contract carries, and to replace one of them while copying every other
//! section through byte-for-byte.

use wasmparser::BinaryReaderError;

/// The custom section holding the contract's spec, as XDR encoded
/// [`stellar_xdr::ScSpecEntry`] values concatenated together.
pub const SPEC_SECTION_NAME: &str = "contractspecv0";

/// The custom section holding the contract's meta, as XDR encoded
/// [`stellar_xdr::ScMetaEntry`] values concatenated together.
pub const META_SECTION_NAME: &str = "contractmetav0";

/// Returns the concatenated contents of every custom section with `name`.
///
/// Returns `None` when the wasm has no such section. Sections are concatenated
/// rather than taking the first, because the contents of each is a stream of
/// XDR values, and a build may produce more than one section — the linker
/// merges the SDK's `link_section` statics into one, but tools append
/// additional sections after the fact.
pub fn custom_section(wasm: &[u8], name: &str) -> Result<Option<Vec<u8>>, BinaryReaderError> {
    let mut out: Option<Vec<u8>> = None;
    for payload in wasmparser::Parser::new(0).parse_all(wasm) {
        if let wasmparser::Payload::CustomSection(section) = payload? {
            if section.name() == name {
                out.get_or_insert_with(Vec::new)
                    .extend_from_slice(section.data());
            }
        }
    }
    Ok(out)
}

/// Returns `wasm` with every custom section named `name` removed, and a single
/// custom section with `name` and `contents` appended in their place.
///
/// Every other section is copied through verbatim, so anything this code does
/// not understand — including sections a future toolchain emits — survives
/// unchanged.
pub fn replace_custom_section(
    wasm: &[u8],
    name: &str,
    contents: &[u8],
) -> Result<Vec<u8>, BinaryReaderError> {
    // The 8 byte header: magic number and version.
    let mut out = wasm[..8].to_vec();

    for payload in wasmparser::Parser::new(0).parse_all(wasm) {
        let payload = payload?;
        let is_target = matches!(
            &payload,
            wasmparser::Payload::CustomSection(s) if s.name() == name,
        );
        if is_target {
            continue;
        }
        // `as_section` yields the id and the byte range of the section's
        // contents, for payloads that are sections. Everything else (the
        // header, the end of the module) is not copied.
        if let Some((id, range)) = payload.as_section() {
            out.push(id);
            write_leb128_u32(&mut out, u32::try_from(range.len()).unwrap());
            out.extend_from_slice(&wasm[range]);
        }
    }

    // Append the replacement section: a custom section is a name, length
    // prefixed, followed by the contents.
    let mut section = Vec::with_capacity(5 + name.len() + contents.len());
    write_leb128_u32(&mut section, u32::try_from(name.len()).unwrap());
    section.extend_from_slice(name.as_bytes());
    section.extend_from_slice(contents);
    out.push(0); // custom section id
    write_leb128_u32(&mut out, u32::try_from(section.len()).unwrap());
    out.extend_from_slice(&section);

    Ok(out)
}

fn write_leb128_u32(out: &mut Vec<u8>, mut n: u32) {
    loop {
        let byte = (n & 0x7f) as u8;
        n >>= 7;
        if n == 0 {
            out.push(byte);
            return;
        }
        out.push(byte | 0x80);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Builds a wasm module out of raw sections, for tests.
    pub(crate) fn module(sections: &[(u8, Vec<u8>)]) -> Vec<u8> {
        let mut out = b"\0asm\x01\0\0\0".to_vec();
        for (id, contents) in sections {
            out.push(*id);
            write_leb128_u32(&mut out, u32::try_from(contents.len()).unwrap());
            out.extend_from_slice(contents);
        }
        out
    }

    /// Builds the contents of a custom section with `name`.
    pub(crate) fn custom(name: &str, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        write_leb128_u32(&mut out, u32::try_from(name.len()).unwrap());
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(data);
        out
    }

    #[test]
    fn leb128() {
        let enc = |n| {
            let mut v = Vec::new();
            write_leb128_u32(&mut v, n);
            v
        };
        assert_eq!(enc(0), vec![0x00]);
        assert_eq!(enc(127), vec![0x7f]);
        assert_eq!(enc(128), vec![0x80, 0x01]);
        assert_eq!(enc(624_485), vec![0xe5, 0x8e, 0x26]);
    }

    #[test]
    fn custom_section_concatenates_all_with_the_name() {
        let wasm = module(&[
            (0, custom("contractspecv0", b"aaa")),
            (0, custom("other", b"xxx")),
            (0, custom("contractspecv0", b"bbb")),
        ]);
        assert_eq!(
            custom_section(&wasm, "contractspecv0").unwrap(),
            Some(b"aaabbb".to_vec()),
        );
        assert_eq!(custom_section(&wasm, "absent").unwrap(), None);
    }

    #[test]
    fn replace_custom_section_replaces_all_and_keeps_the_rest() {
        let wasm = module(&[
            (0, custom("contractspecv0", b"old")),
            (0, custom("keep-me", b"xxx")),
            (0, custom("contractspecv0", b"also-old")),
        ]);
        let out = replace_custom_section(&wasm, "contractspecv0", b"new").unwrap();
        assert_eq!(
            custom_section(&out, "contractspecv0").unwrap(),
            Some(b"new".to_vec()),
        );
        assert_eq!(
            custom_section(&out, "keep-me").unwrap(),
            Some(b"xxx".to_vec()),
        );
    }

    #[test]
    fn replace_custom_section_adds_when_absent_and_copies_non_custom_sections() {
        // A type section, which must survive untouched.
        let type_section = vec![0x01, 0x60, 0x00, 0x00];
        let wasm = module(&[(1, type_section.clone())]);
        let out = replace_custom_section(&wasm, "contractspecv0", b"new").unwrap();
        assert_eq!(
            custom_section(&out, "contractspecv0").unwrap(),
            Some(b"new".to_vec()),
        );
        assert_eq!(out[..8], *b"\0asm\x01\0\0\0");
        // The type section is still there, byte for byte.
        assert!(out.windows(type_section.len()).any(|w| w == type_section));
    }

    #[test]
    fn replace_custom_section_round_trips_a_large_section() {
        // Longer than 127 bytes, so the length needs more than one leb128 byte.
        let big = vec![7u8; 1000];
        let wasm = module(&[(0, custom("contractspecv0", b"old"))]);
        let out = replace_custom_section(&wasm, "contractspecv0", &big).unwrap();
        assert_eq!(custom_section(&out, "contractspecv0").unwrap(), Some(big));
    }
}
