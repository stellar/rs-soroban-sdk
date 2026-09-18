//! Shaking unused entries out of a contract's spec.
//!
//! This mirrors what `stellar contract build` does after it invokes cargo, so
//! that a contract built by any build system ends up with the same spec as one
//! built by the CLI. See [`soroban_spec::shaking`] for how the markers this
//! reads are produced and what they mean.

use std::collections::HashSet;
use std::io::Cursor;

use stellar_xdr::{Limited, Limits, ReadXdr, ScMetaEntry, ScSpecEntry, WriteXdr};

use crate::wasm::{self, META_SECTION_NAME, SPEC_SECTION_NAME};

/// Matches the depth limit the CLI and `soroban-env-host` decode spec and meta
/// with, so that any spec the network would accept decodes here too.
pub(crate) const XDR_DEPTH_LIMIT: u32 = 500;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("parsing wasm: {0}")]
    Wasm(#[from] wasmparser::BinaryReaderError),
    #[error("reading contract spec or meta: {0}")]
    Xdr(#[from] stellar_xdr::Error),
}

/// The outcome of a [`shake`] call, for reporting.
#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The wasm is not a contract built by an SDK that supports spec shaking,
    /// so it was left alone. This covers wasm that is not a contract at all.
    Skipped,
    /// The spec was shaken.
    Shaken {
        /// Entries in the spec before shaking.
        before: usize,
        /// Entries in the spec after shaking.
        after: usize,
    },
}

/// Returns `wasm` with unused entries removed from its spec, or `None` when
/// there is nothing to do.
///
/// An entry is kept when it is a function, because functions are the contract's
/// API, or when the data section still contains its marker. The SDK emits a
/// marker for every type and event alongside a use of it, so a marker survives
/// dead code elimination only if the type or event it describes is reachable.
/// Exact duplicate entries, which occur when more than one crate in the build
/// defines the same type, are collapsed to one.
///
/// The spec is left alone unless the contract meta says the SDK emitted markers
/// (spec shaking version 2). Without that, no markers exist and every type and
/// event would be dropped.
pub fn shake(wasm: &[u8]) -> Result<Option<(Vec<u8>, Outcome)>, Error> {
    let Some(spec_xdr) = wasm::custom_section(wasm, SPEC_SECTION_NAME)? else {
        return Ok(None);
    };

    let meta_xdr = wasm::custom_section(wasm, META_SECTION_NAME)?.unwrap_or_default();
    let meta = read_xdr::<ScMetaEntry>(&meta_xdr)?;
    if soroban_spec::shaking::spec_shaking_version_for_meta(&meta) != 2 {
        return Ok(Some((wasm.to_vec(), Outcome::Skipped)));
    }

    let entries = read_xdr::<ScSpecEntry>(&spec_xdr)?;
    let before = entries.len();
    let markers = soroban_spec::shaking::find_all(wasm);

    let mut seen = HashSet::new();
    let mut shaken_xdr = Vec::new();
    let mut after = 0;
    {
        let mut writer = Limited::new(Cursor::new(&mut shaken_xdr), Limits::depth(XDR_DEPTH_LIMIT));
        for entry in soroban_spec::shaking::filter(entries, &markers) {
            let entry_xdr = entry.to_xdr(Limits::depth(XDR_DEPTH_LIMIT))?;
            if seen.insert(entry_xdr) {
                entry.write_xdr(&mut writer)?;
                after += 1;
            }
        }
    }

    let wasm = wasm::replace_custom_section(wasm, SPEC_SECTION_NAME, &shaken_xdr)?;
    Ok(Some((wasm, Outcome::Shaken { before, after })))
}

pub(crate) fn read_xdr<T: ReadXdr>(xdr: &[u8]) -> Result<Vec<T>, stellar_xdr::Error> {
    let mut read = Limited::new(Cursor::new(xdr), Limits::depth(XDR_DEPTH_LIMIT));
    T::read_xdr_iter(&mut read).collect()
}
