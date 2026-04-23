//! Post-build stripping for WASMs built with spec shaking v2.
//!
//! In v2, the SDK plumbs spec-entry reachability purely through a static
//! `MarkerNode` graph rooted by a volatile read at each contract-fn boundary.
//! All `SpEcV1…` marker bytes live inside `MarkerNode` structs in the data
//! section; no marker functions and no indirect-function-table surface exist.
//! Once the post-build tool has scanned the data section to decide which
//! `contractspecv0` entries to keep, the entire graph is dead weight on-chain.
//!
//! [`strip_spec_shaking`] performs the cleanup:
//! 1. Scans data segments for `SpEcV1…` patterns to collect the live marker
//!    set (for spec filtering).
//! 2. Walks the full `MarkerNode` graph starting from each SpEcV1 seed,
//!    following `children` slice pointers transitively. Zeroes each
//!    `MarkerNode` struct (24 bytes on wasm32) and each `children` slice
//!    backing in place. This reaches container-wrapper nodes whose own marker
//!    bytes are all-zero (invisible to a bare byte scan) via the pointers
//!    from UDT nodes that are found by the scan.
//! 3. Rewrites the `contractspecv0` custom section to keep only entries whose
//!    marker was observed in the scan.
//!
//! Data-section offsets stay stable — we zero ranges in place rather than
//! removing them — so live code that references other bytes in the same
//! segment still finds them. The boundary wrappers only ever read the root
//! node *pointer* (an address that they immediately drop), never the node's
//! bytes, so zeroing is safe.

use std::collections::{HashMap, HashSet};

use stellar_xdr::curr::{self, Limited, Limits, ReadXdr, ScMetaEntry, ScSpecEntry, WriteXdr};
use walrus::{ir::Value, ConstExpr, DataId, DataKind, Module, ModuleConfig, RawCustomSection};

use crate::read::{parse_raw, FromWasmError};
use crate::shaking::{self, Marker};

const MAGIC: &[u8; 6] = b"SpEcV1";
const MARKER_LEN: usize = 14;

// MarkerNode layout on wasm32 (matches soroban-sdk/src/spec_shaking.rs):
//   [0..14)  marker bytes  ("SpEcV1…" for UDTs; all-zero for container wrappers)
//   [14..16) padding (4-byte alignment for the slice reference below)
//   [16..20) slice pointer (u32 little-endian; 0 for empty slice)
//   [20..24) slice length (u32 little-endian)
// Children slice backing: `len * PTR_SIZE` bytes, one `*const MarkerNode` each.
const NODE_SIZE: usize = 24;
const SLICE_PTR_OFFSET: usize = 16;
const SLICE_LEN_OFFSET: usize = 20;
const PTR_SIZE: usize = 4;

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

/// Strip the MarkerNode graph, `SpEcV1…` marker bytes, and unused
/// `contractspecv0` entries from a WASM built with soroban-sdk's spec
/// shaking v2. See module docs for assumptions.
pub fn strip_spec_shaking(wasm: &[u8]) -> Result<Vec<u8>, StripError> {
    // Don't let walrus inject a `producers` custom section into the output;
    // Soroban contract wasms don't carry one by default and adding it would
    // be an unexpected side effect of stripping.
    let mut cfg = ModuleConfig::new();
    cfg.generate_producers_section(false);
    let mut module = Module::from_buffer_with_config(wasm, &cfg)
        .map_err(|e| StripError::Parse(e.to_string()))?;

    // 1. Refuse to strip wasms that weren't built with spec shaking v2.
    if read_spec_shaking_version(&module) != SUPPORTED_SPEC_SHAKING_VERSION {
        return Err(StripError::UnsupportedShakingVersion);
    }

    // 2. Scan data segments for `SpEcV1…` markers. Returns both the marker
    //    set (for spec filtering) and the (data_id, offset) of each SpEcV1
    //    location — these are the seeds for the MarkerNode graph walk.
    let (used_markers, udt_node_seeds) = scan_markers(&module);

    // 3. Walk the full MarkerNode graph from the SpEcV1 seeds, following
    //    children pointers into container-wrapper nodes. Returns the byte
    //    ranges to zero, grouped by data segment.
    let ranges_by_segment = walk_marker_graph(&module, &udt_node_seeds);

    // 4. Zero each collected range in place. Segment offsets stay stable so
    //    surrounding live data keeps its addresses.
    for (data_id, ranges) in ranges_by_segment {
        let data = module.data.get_mut(data_id);
        for (start, end) in ranges {
            data.value[start..end].fill(0);
        }
    }

    // 5. Compact data segments by splitting around long runs of zero bytes.
    //    Zeroing a MarkerNode doesn't reduce the emitted wasm size by itself —
    //    zero bytes still serialize — but because live code references nothing
    //    inside the zeroed ranges, we can split segments around them so those
    //    bytes simply aren't emitted. The threshold covers the per-segment
    //    header overhead (mode + offset expr + length LEB128); below it,
    //    splitting would cost more bytes than it saves.
    compact_zero_runs(&mut module, /* threshold = */ 16);

    // 6. Rewrite contractspecv0 to keep only entries with live markers.
    rewrite_contractspecv0(&mut module, &used_markers)?;

    Ok(module.emit_wasm())
}

/// Scan data segments for `SpEcV1…` markers. Returns:
/// - the set of 14-byte markers found (for spec filtering);
/// - the `(data_id, offset)` of each SpEcV1 occurrence, which is the offset
///   of a UDT `MarkerNode` (the `SpEcV1…` marker occupies bytes [0..14) of
///   the struct).
fn scan_markers(module: &Module) -> (HashSet<Marker>, Vec<(DataId, usize)>) {
    let mut used = HashSet::new();
    let mut seeds = Vec::new();
    for data in module.data.iter() {
        let bytes = &data.value;
        if bytes.len() < MARKER_LEN {
            continue;
        }
        let mut i = 0;
        while i + MARKER_LEN <= bytes.len() {
            if &bytes[i..i + MAGIC.len()] == MAGIC {
                let mut m = [0u8; MARKER_LEN];
                m.copy_from_slice(&bytes[i..i + MARKER_LEN]);
                used.insert(m);
                seeds.push((data.id(), i));
                i += MARKER_LEN;
            } else {
                i += 1;
            }
        }
    }
    (used, seeds)
}

/// Map each active data segment to its base memory address, so that we can
/// resolve pointer values (which are memory addresses) back to
/// `(DataId, offset_in_segment)`. Returns a list sorted by base address.
///
/// Passive data segments and active segments with non-literal offsets
/// (e.g. relative to a global) are skipped — any pointer into them can't be
/// resolved, so children reached through them are left alone. Rust-compiled
/// contracts use active segments with immediate `i32.const` offsets, so in
/// practice everything is resolvable.
fn build_segment_map(module: &Module) -> Vec<(u32, usize, DataId)> {
    let mut segs: Vec<(u32, usize, DataId)> = module
        .data
        .iter()
        .filter_map(|data| match &data.kind {
            DataKind::Active { offset, .. } => match offset {
                ConstExpr::Value(Value::I32(v)) => Some((*v as u32, data.value.len(), data.id())),
                _ => None,
            },
            DataKind::Passive => None,
        })
        .collect();
    segs.sort_by_key(|&(base, _, _)| base);
    segs
}

/// Resolve a memory address to `(DataId, offset_in_segment)`. Returns `None`
/// if the address doesn't fall inside any known active data segment.
fn resolve_addr(segs: &[(u32, usize, DataId)], addr: u32) -> Option<(DataId, usize)> {
    for &(base, len, id) in segs {
        let end = base as u64 + len as u64;
        if (addr as u64) >= base as u64 && (addr as u64) < end {
            return Some((id, (addr - base) as usize));
        }
    }
    None
}

/// Check whether the 14-byte marker at the given offset is a plausible
/// `MarkerNode` prefix: either it starts with `SpEcV1` (UDT node) or is all
/// zeros (container-wrapper node). This is the minimum validation before we
/// zero 24 bytes at that offset.
fn is_plausible_marker_node_prefix(bytes: &[u8]) -> bool {
    if bytes.len() < MARKER_LEN {
        return false;
    }
    &bytes[..MAGIC.len()] == MAGIC || bytes[..MARKER_LEN].iter().all(|&b| b == 0)
}

/// BFS the `MarkerNode` graph from the given seeds, following each node's
/// `children` slice pointers transitively. Returns the byte ranges to zero,
/// grouped by data segment: each `MarkerNode` contributes a 24-byte range
/// for its struct, plus a `len * 4`-byte range for its children slice
/// backing (if any, and if resolvable).
fn walk_marker_graph(
    module: &Module,
    seeds: &[(DataId, usize)],
) -> HashMap<DataId, Vec<(usize, usize)>> {
    let segs = build_segment_map(module);

    let mut visited_nodes: HashSet<(DataId, usize)> = HashSet::new();
    let mut visited_slices: HashSet<(DataId, usize)> = HashSet::new();
    let mut queue: Vec<(DataId, usize)> = seeds.to_vec();
    let mut ranges: HashMap<DataId, Vec<(usize, usize)>> = HashMap::new();

    while let Some((data_id, node_offset)) = queue.pop() {
        if !visited_nodes.insert((data_id, node_offset)) {
            continue;
        }

        let data_value = &module.data.get(data_id).value;
        if node_offset + NODE_SIZE > data_value.len() {
            continue;
        }
        let node_bytes = &data_value[node_offset..node_offset + NODE_SIZE];
        if !is_plausible_marker_node_prefix(node_bytes) {
            continue;
        }

        // Zero the 24-byte MarkerNode struct itself.
        ranges
            .entry(data_id)
            .or_default()
            .push((node_offset, node_offset + NODE_SIZE));

        // Read the children slice (ptr, len).
        let slice_ptr = u32::from_le_bytes(
            node_bytes[SLICE_PTR_OFFSET..SLICE_PTR_OFFSET + 4]
                .try_into()
                .unwrap(),
        );
        let slice_len = u32::from_le_bytes(
            node_bytes[SLICE_LEN_OFFSET..SLICE_LEN_OFFSET + 4]
                .try_into()
                .unwrap(),
        ) as usize;

        if slice_len == 0 || slice_ptr == 0 {
            continue;
        }

        // Resolve slice_ptr to a concrete (data_id, offset) in some active
        // segment. If the address doesn't land inside one, skip — we can't
        // safely walk or zero an unresolved region.
        let Some((slice_data_id, slice_offset)) = resolve_addr(&segs, slice_ptr) else {
            continue;
        };
        let slice_byte_len = slice_len * PTR_SIZE;
        let slice_data_len = module.data.get(slice_data_id).value.len();
        if slice_offset + slice_byte_len > slice_data_len {
            continue;
        }

        // Zero the children slice backing (dedup — wasm-ld may share a
        // backing between multiple identical slices).
        if visited_slices.insert((slice_data_id, slice_offset)) {
            ranges
                .entry(slice_data_id)
                .or_default()
                .push((slice_offset, slice_offset + slice_byte_len));
        }

        // Follow each child pointer to enqueue further nodes. Read the
        // backing bytes fresh each iteration since `module.data.get` returns
        // a borrow.
        for i in 0..slice_len {
            let entry_off = slice_offset + i * PTR_SIZE;
            let entry_bytes =
                &module.data.get(slice_data_id).value[entry_off..entry_off + PTR_SIZE];
            let child_ptr = u32::from_le_bytes(entry_bytes.try_into().unwrap());
            if child_ptr == 0 {
                continue;
            }
            if let Some((cd_id, co)) = resolve_addr(&segs, child_ptr) {
                queue.push((cd_id, co));
            }
        }
    }

    ranges
}

/// Split each active data segment around runs of zero bytes of at least
/// `threshold` length. The zeroed ranges inside stripped `MarkerNode` structs
/// are contiguous and typically ≥24 bytes, so splitting removes them from
/// the emitted wasm while preserving the memory layout of surrounding live
/// data (each resulting piece carries the absolute memory address it should
/// be placed at). Segments whose offset isn't a literal `i32.const` are
/// left alone — Rust-compiled contracts use literal offsets, so in practice
/// everything is compactable.
fn compact_zero_runs(module: &mut Module, threshold: usize) {
    let ids: Vec<DataId> = module.data.iter().map(|d| d.id()).collect();
    for id in ids {
        let data = module.data.get(id);
        let (memory, base) = match &data.kind {
            DataKind::Active { memory, offset } => match offset {
                ConstExpr::Value(Value::I32(v)) => (*memory, *v as u32),
                _ => continue,
            },
            DataKind::Passive => continue,
        };
        let value = data.value.clone();

        let chunks = non_zero_chunks(&value, threshold);
        // Nothing to do if the segment is one contiguous chunk (no zero runs
        // at or above the threshold).
        if chunks.len() == 1 && chunks[0] == (0, value.len()) {
            continue;
        }

        if let Some(&(first_start, first_end)) = chunks.first() {
            // Shrink the existing segment in place to the first surviving chunk.
            let d = module.data.get_mut(id);
            d.value = value[first_start..first_end].to_vec();
            if let DataKind::Active { offset, .. } = &mut d.kind {
                *offset = ConstExpr::Value(Value::I32((base + first_start as u32) as i32));
            }
            // Each remaining chunk becomes its own new segment, placed at the
            // original absolute memory address so live code still finds it.
            for &(start, end) in chunks.iter().skip(1) {
                module.data.add(
                    DataKind::Active {
                        memory,
                        offset: ConstExpr::Value(Value::I32((base + start as u32) as i32)),
                    },
                    value[start..end].to_vec(),
                );
            }
        } else {
            // Entire segment was zeroed — nothing to re-emit. walrus handles
            // the deletion via Tombstone; no other segments referred to it.
            module.data.delete(id);
        }
    }
}

/// Return `(start, end)` ranges of bytes that should survive segment
/// compaction — everything except runs of zero bytes of at least
/// `threshold` length. A shorter zero run is kept in place (splitting would
/// cost more bytes in segment headers than it'd save).
fn non_zero_chunks(bytes: &[u8], threshold: usize) -> Vec<(usize, usize)> {
    let mut chunks = Vec::new();
    let n = bytes.len();
    let mut i = 0;
    let mut chunk_start = 0;
    while i < n {
        if bytes[i] == 0 {
            let zero_start = i;
            while i < n && bytes[i] == 0 {
                i += 1;
            }
            if i - zero_start >= threshold {
                if chunk_start < zero_start {
                    chunks.push((chunk_start, zero_start));
                }
                chunk_start = i;
            }
        } else {
            i += 1;
        }
    }
    if chunk_start < n {
        chunks.push((chunk_start, n));
    }
    chunks
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
