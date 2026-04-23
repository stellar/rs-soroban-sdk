//! Post-build stripping for WASMs built with all-`keep_reachable` spec shaking.
//!
//! In that design, contract function wrappers contain no inlined `MARKER` byte
//! loads — every `SpEcV1…` read lives inside a standalone `() -> ()` marker
//! function referenced only via the indirect function table. Post-link those
//! functions and the `SpEcV1…` bytes are dead weight on-chain: they exist
//! purely to keep the `SpEcV1…` data alive through `rustc`/LLVM's dead-code
//! elimination, and the bytes are never read at runtime.
//!
//! [`strip_keep_reachable`] performs the cleanup:
//! 1. Identifies standalone marker functions (signature `() -> ()`, reachable
//!    only via the element section, never directly called or exported).
//! 2. Removes them from the code and element sections.
//! 3. Zeroes the `SpEcV1…` bytes inside data segments.
//! 4. Rewrites the `contractspecv0` custom section to keep only entries whose
//!    marker was observed in the scan.
//!
//! The inlined `keep_reachable` sequences in contract-fn wrappers
//! (`i32.const <idx>; i32.store; i32.load; drop`) are left in place. They
//! execute as a harmless stack round-trip of an immediate that now points at
//! an empty (`ref.null`) table slot; no `call_indirect` consumes it, so WASM
//! validation and runtime behaviour are unaffected.
//!
//! Note on data-section size: the MARKER bytes are zeroed in place rather
//! than removed, so segment offsets for surrounding live data stay stable.
//! True byte reduction would require splitting segments around each marker
//! range, which this pass does not attempt.

use std::collections::HashSet;

use stellar_xdr::curr::{self, Limited, Limits, ReadXdr, ScMetaEntry, ScSpecEntry, WriteXdr};
use walrus::{
    ir::{dfs_in_order, Instr, InstrLocId, Visitor},
    DataId, ElementItems, ExportItem, FunctionId, Module, ModuleConfig, RawCustomSection,
};

use crate::read::{parse_raw, FromWasmError};
use crate::shaking::{self, Marker};

const MAGIC: &[u8; 6] = b"SpEcV1";
const MARKER_LEN: usize = 14;
const SPEC_CUSTOM_SECTION: &str = "contractspecv0";
const META_CUSTOM_SECTION: &str = "contractmetav0";
const SUPPORTED_SPEC_SHAKING_VERSION: u32 = 2;

#[derive(Debug, thiserror::Error)]
pub enum StripError {
    #[error("parsing wasm: {0}")]
    Parse(String),
    #[error("reading spec section")]
    Spec(#[from] FromWasmError),
    #[error("encoding spec entry xdr")]
    Xdr(#[from] curr::Error),
    #[error(
        "wasm was not built with spec shaking v2; expected rssdk_spec_shaking=2 in \
         contractmetav0. Stripping older wasms would drop spec entries for types \
         whose usage the SDK did not mark."
    )]
    UnsupportedShakingVersion,
}

/// Strip marker functions, element entries, `SpEcV1…` data bytes, and unused
/// `contractspecv0` entries from a WASM built with the all-`keep_reachable`
/// variant of soroban-sdk's spec shaking. See module docs for assumptions.
pub fn strip_keep_reachable(wasm: &[u8]) -> Result<Vec<u8>, StripError> {
    // Don't let walrus inject a `producers` custom section into the output;
    // Soroban contract wasms don't carry one by default and adding it would
    // be an unexpected side effect of stripping.
    let mut cfg = ModuleConfig::new();
    cfg.generate_producers_section(false);
    let mut module = Module::from_buffer_with_config(wasm, &cfg)
        .map_err(|e| StripError::Parse(e.to_string()))?;

    // 1. Refuse to strip wasms that weren't built with spec shaking v2.
    //    Without v2 markers there's no way to tell which spec entries are
    //    live, so filtering `contractspecv0` would drop every type.
    if read_spec_shaking_version(&module) != SUPPORTED_SPEC_SHAKING_VERSION {
        return Err(StripError::UnsupportedShakingVersion);
    }

    // 2. Scan data segments once: collect the live marker set (for spec
    //    filtering) and the per-segment byte ranges (for zeroing).
    let (used_markers, marker_ranges) = scan_markers(&module);

    // 3. Identify standalone marker functions: `() -> ()`, referenced via
    //    the element table, never exported, never directly called.
    let marker_fns = identify_marker_fns(&module);

    // 4. Detach marker functions from element segments BEFORE deletion
    //    (walrus would otherwise flag them as still-referenced on emit).
    let elem_ids: Vec<_> = module.elements.iter().map(|e| e.id()).collect();
    for elem_id in elem_ids {
        if let ElementItems::Functions(ref mut fns) = module.elements.get_mut(elem_id).items {
            fns.retain(|f| !marker_fns.contains(f));
        }
    }

    // 5. Delete the marker function bodies.
    for fn_id in &marker_fns {
        module.funcs.delete(*fn_id);
    }

    // 6. Zero `SpEcV1…` bytes inside data segments. Segment offsets stay
    //    stable so live code that references other bytes in the same
    //    segment still finds them.
    for (data_id, ranges) in &marker_ranges {
        let data = module.data.get_mut(*data_id);
        for &(start, end) in ranges {
            data.value[start..end].fill(0);
        }
    }

    // 7. Rewrite contractspecv0 to keep only entries with live markers.
    rewrite_contractspecv0(&mut module, &used_markers)?;

    Ok(module.emit_wasm())
}

/// Scan data segments for `SpEcV1…` markers. Returns both the set of 14-byte
/// markers found (for spec filtering) and the per-segment byte ranges to
/// zero out.
fn scan_markers(module: &Module) -> (HashSet<Marker>, Vec<(DataId, Vec<(usize, usize)>)>) {
    let mut used = HashSet::new();
    let mut ranges_by_segment = Vec::new();
    for data in module.data.iter() {
        let bytes = &data.value;
        let mut ranges = Vec::new();
        let mut i = 0;
        while i + MARKER_LEN <= bytes.len() {
            if &bytes[i..i + MAGIC.len()] == MAGIC {
                let mut m = [0u8; MARKER_LEN];
                m.copy_from_slice(&bytes[i..i + MARKER_LEN]);
                used.insert(m);
                ranges.push((i, i + MARKER_LEN));
                i += MARKER_LEN;
            } else {
                i += 1;
            }
        }
        if !ranges.is_empty() {
            ranges_by_segment.push((data.id(), ranges));
        }
    }
    (used, ranges_by_segment)
}

/// Identify standalone marker functions. The four filters together are
/// conservative: any function that fails one stays put, so false positives
/// cannot accidentally strip live code.
fn identify_marker_fns(module: &Module) -> HashSet<FunctionId> {
    // If no `() -> ()` type is declared, no function in the module has that
    // signature — nothing to strip.
    let Some(empty_ty) = module.types.find(&[], &[]) else {
        return HashSet::new();
    };

    let exported: HashSet<FunctionId> = module
        .exports
        .iter()
        .filter_map(|e| match e.item {
            ExportItem::Function(f) => Some(f),
            _ => None,
        })
        .collect();

    let mut elem_referenced: HashSet<FunctionId> = HashSet::new();
    for elem in module.elements.iter() {
        if let ElementItems::Functions(fns) = &elem.items {
            elem_referenced.extend(fns.iter().copied());
        }
    }

    let directly_called = collect_direct_callees(module);

    module
        .funcs
        .iter()
        .filter(|f| f.ty() == empty_ty)
        .filter(|f| elem_referenced.contains(&f.id()))
        .filter(|f| !exported.contains(&f.id()))
        .filter(|f| !directly_called.contains(&f.id()))
        .map(|f| f.id())
        .collect()
}

/// Walk every local function body and collect the set of functions reached
/// via a `call <idx>` instruction. `call_indirect` and `ref.func` are
/// intentionally excluded — we want to know which functions are genuinely
/// called, not which ones are addressed.
fn collect_direct_callees(module: &Module) -> HashSet<FunctionId> {
    struct Collector(HashSet<FunctionId>);
    impl<'a> Visitor<'a> for Collector {
        fn visit_instr(&mut self, instr: &'a Instr, _loc: &'a InstrLocId) {
            if let Instr::Call(call) = instr {
                self.0.insert(call.func);
            }
        }
    }
    let mut c = Collector(HashSet::new());
    for (_, local) in module.funcs.iter_local() {
        dfs_in_order(&mut c, local, local.entry_block());
    }
    c.0
}

/// Rewrite the `contractspecv0` custom section, keeping only entries whose
/// marker was observed in the data-section scan. `FunctionV0` entries are
/// always kept — they define the contract ABI and have no per-type marker.
fn rewrite_contractspecv0(module: &mut Module, used: &HashSet<Marker>) -> Result<(), StripError> {
    let Some(raw) = module.customs.remove_raw(SPEC_CUSTOM_SECTION) else {
        return Ok(());
    };

    let entries = parse_raw(&raw.data).map_err(FromWasmError::Parse)?;
    let kept: Vec<ScSpecEntry> = shaking::filter(entries, used).collect();

    let mut bytes = Vec::new();
    for entry in &kept {
        bytes.extend_from_slice(&entry.to_xdr(Limits::none())?);
    }

    module.customs.add(RawCustomSection {
        name: SPEC_CUSTOM_SECTION.to_string(),
        data: bytes,
    });
    Ok(())
}

/// Read the `rssdk_spec_shaking` key out of the `contractmetav0` custom
/// section. Returns the declared version, or `1` if the section or key is
/// absent — matching the convention used by [`shaking::spec_shaking_version_for_meta`].
fn read_spec_shaking_version(module: &Module) -> u32 {
    let Some(section) = module
        .customs
        .iter()
        .find_map(|(_id, s)| (s.name() == META_CUSTOM_SECTION).then_some(s))
    else {
        return 1;
    };
    let data = section.data(&Default::default());
    let mut cursor = std::io::Cursor::new(&data[..]);
    let mut limited = Limited::new(
        &mut cursor,
        Limits {
            depth: 500,
            len: 0x100_0000,
        },
    );
    let entries: Vec<ScMetaEntry> = ScMetaEntry::read_xdr_iter(&mut limited)
        .filter_map(Result::ok)
        .collect();
    shaking::spec_shaking_version_for_meta(&entries)
}
