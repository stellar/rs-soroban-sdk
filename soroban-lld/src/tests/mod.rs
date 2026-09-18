//! Unit tests, and the fixtures they share.

mod cli;
mod shake;
mod wasm;

use crate::wasm::write_leb128_u32;

/// Builds a wasm module out of raw sections.
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
