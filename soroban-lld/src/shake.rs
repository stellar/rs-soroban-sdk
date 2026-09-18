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
const XDR_DEPTH_LIMIT: u32 = 500;

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

fn read_xdr<T: ReadXdr>(xdr: &[u8]) -> Result<Vec<T>, stellar_xdr::Error> {
    let mut read = Limited::new(Cursor::new(xdr), Limits::depth(XDR_DEPTH_LIMIT));
    T::read_xdr_iter(&mut read).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm::tests::{custom, module};
    use stellar_xdr::{
        ScMetaV0, ScSpecEventDataFormat, ScSpecEventV0, ScSpecFunctionV0, ScSpecTypeDef,
        ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM, VecM,
    };

    fn function(name: &str) -> ScSpecEntry {
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: name.try_into().unwrap(),
            inputs: VecM::default(),
            outputs: VecM::default(),
        })
    }

    fn struct_(name: &str) -> ScSpecEntry {
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            fields: vec![ScSpecUdtStructFieldV0 {
                doc: StringM::default(),
                name: "f".try_into().unwrap(),
                type_: ScSpecTypeDef::U32,
            }]
            .try_into()
            .unwrap(),
        })
    }

    fn event(name: &str) -> ScSpecEntry {
        ScSpecEntry::EventV0(ScSpecEventV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            prefix_topics: VecM::default(),
            params: VecM::default(),
            data_format: ScSpecEventDataFormat::SingleValue,
        })
    }

    fn xdr<T: WriteXdr>(entries: &[T]) -> Vec<u8> {
        let mut out = Vec::new();
        for e in entries {
            out.extend(e.to_xdr(Limits::depth(XDR_DEPTH_LIMIT)).unwrap());
        }
        out
    }

    fn shaking_meta() -> Vec<ScMetaEntry> {
        vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: soroban_spec::shaking::META_KEY.try_into().unwrap(),
            val: soroban_spec::shaking::META_VALUE_V2.try_into().unwrap(),
        })]
    }

    /// Builds a data section holding the markers for `live`, the way the SDK's
    /// marker statics land in a linked contract.
    fn data_section(live: &[&ScSpecEntry]) -> Vec<u8> {
        let mut bytes = Vec::new();
        for entry in live {
            bytes.extend(soroban_spec::shaking::generate_marker_for_entry(entry));
        }
        let mut section = vec![0x01]; // one segment
        section.push(0x01); // passive
        section.push(u8::try_from(bytes.len()).unwrap());
        section.extend(bytes);
        section
    }

    fn contract(spec: &[ScSpecEntry], meta: &[ScMetaEntry], live: &[&ScSpecEntry]) -> Vec<u8> {
        module(&[
            (11, data_section(live)),
            (0, custom(SPEC_SECTION_NAME, &xdr(spec))),
            (0, custom(META_SECTION_NAME, &xdr(meta))),
        ])
    }

    fn spec_of(wasm: &[u8]) -> Vec<ScSpecEntry> {
        read_xdr(
            &crate::wasm::custom_section(wasm, SPEC_SECTION_NAME)
                .unwrap()
                .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn keeps_functions_and_used_types_drops_the_rest() {
        let (f, used, unused) = (function("hello"), struct_("Used"), struct_("Unused"));
        let (live_ev, dead_ev) = (event("Live"), event("Dead"));
        let spec = vec![f.clone(), used.clone(), unused, live_ev.clone(), dead_ev];
        let wasm = contract(&spec, &shaking_meta(), &[&used, &live_ev]);

        let (out, outcome) = shake(&wasm).unwrap().unwrap();
        assert_eq!(
            outcome,
            Outcome::Shaken {
                before: 5,
                after: 3
            }
        );
        // The function is kept without a marker, because functions are the API.
        assert_eq!(spec_of(&out), vec![f, used, live_ev]);
    }

    #[test]
    fn collapses_exact_duplicates() {
        // The same type defined by two crates in the build appears twice.
        let (f, used) = (function("hello"), struct_("Used"));
        let spec = vec![f.clone(), used.clone(), used.clone()];
        let wasm = contract(&spec, &shaking_meta(), &[&used]);

        let (out, outcome) = shake(&wasm).unwrap().unwrap();
        assert_eq!(
            outcome,
            Outcome::Shaken {
                before: 3,
                after: 2
            }
        );
        assert_eq!(spec_of(&out), vec![f, used]);
    }

    #[test]
    fn leaves_the_spec_alone_without_shaking_meta() {
        // An SDK that emits no markers. Shaking would drop every type.
        let spec = vec![function("hello"), struct_("Used")];
        let other_meta = vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: "rssdkver".try_into().unwrap(),
            val: "28.0.0".try_into().unwrap(),
        })];
        let wasm = contract(&spec, &other_meta, &[]);

        let (out, outcome) = shake(&wasm).unwrap().unwrap();
        assert_eq!(outcome, Outcome::Skipped);
        assert_eq!(spec_of(&out), spec);
    }

    #[test]
    fn leaves_the_spec_alone_with_no_meta_at_all() {
        let spec = vec![function("hello"), struct_("Used")];
        let wasm = module(&[(0, custom(SPEC_SECTION_NAME, &xdr(&spec)))]);
        assert_eq!(shake(&wasm).unwrap().unwrap().1, Outcome::Skipped);
    }

    #[test]
    fn ignores_wasm_that_is_not_a_contract() {
        // No spec section at all, as for any other wasm the linker produces.
        let wasm = module(&[(1, vec![0x01, 0x60, 0x00, 0x00])]);
        assert!(shake(&wasm).unwrap().is_none());
    }

    #[test]
    fn drops_every_type_when_nothing_is_live() {
        let f = function("hello");
        let spec = vec![f.clone(), struct_("Unused"), event("Dead")];
        let wasm = contract(&spec, &shaking_meta(), &[]);

        let (out, outcome) = shake(&wasm).unwrap().unwrap();
        assert_eq!(
            outcome,
            Outcome::Shaken {
                before: 3,
                after: 1
            }
        );
        assert_eq!(spec_of(&out), vec![f]);
    }

    #[test]
    fn preserves_other_sections() {
        let used = struct_("Used");
        let wasm = module(&[
            (1, vec![0x01, 0x60, 0x00, 0x00]),
            (11, data_section(&[&used])),
            (
                0,
                custom(SPEC_SECTION_NAME, &xdr(std::slice::from_ref(&used))),
            ),
            (0, custom(META_SECTION_NAME, &xdr(&shaking_meta()))),
            (0, custom("contractenvmetav0", b"envmeta")),
        ]);
        let (out, _) = shake(&wasm).unwrap().unwrap();
        assert_eq!(
            crate::wasm::custom_section(&out, "contractenvmetav0").unwrap(),
            Some(b"envmeta".to_vec()),
        );
        assert_eq!(
            crate::wasm::custom_section(&out, META_SECTION_NAME).unwrap(),
            Some(xdr(&shaking_meta())),
        );
        assert_eq!(spec_of(&out), vec![used]);
    }
}
