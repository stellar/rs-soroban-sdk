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

pub(crate) fn write_leb128_u32(out: &mut Vec<u8>, mut n: u32) {
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
