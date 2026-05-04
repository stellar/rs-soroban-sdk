/// Spec shaking: removing unused spec entries from contract WASMs.
///
/// ## Meta
///
/// The `contractmetav0` section of a WASM may contain an [`ScMetaV0`] entry
/// with key [`soroban_spec_markers::META_KEY`] (`rssdk_spec_shaking`). The value indicates the
/// spec shaking version:
///
/// - Absent or `"1"` — version 1 (no markers, no shaking possible).
/// - `"2"` — version 2, extra-root markers are embedded in the data section.
///
/// Use [`spec_shaking_version_for_meta`] to determine the version from the
/// contract's meta entries.
///
/// ## Markers and spec graph pruning (version 2)
///
/// The marker is a byte array in the data section with a distinctive pattern:
/// - 6 bytes: "SpEcV1" prefix
/// - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
///
/// Markers are embedded for roots that cannot be discovered from the spec alone, currently event
/// publish methods and errors thrown by `panic_with_error!` or `assert_with_error!`. Function input
/// and output roots are discovered directly from the function entries in `contractspecv0`, and UDT
/// reachability is discovered from exact spec IDs in the removable
/// `contractspecv0.rssdk.graphv0` sidecar. When a reachable function, event, or UDT entry
/// references UDTs, its graph record must be present and must resolve those referenced UDTs to
/// exported spec entries. The filter keeps function entries themselves as API roots, but their
/// parameter and return UDTs are only reached through graph records. Exact duplicate spec entries
/// are collapsed during filtering.
///
/// Post-processing tools (e.g. stellar-cli) can:
/// 1. Scan the WASM data section for "SpEcV1" patterns
/// 2. Extract the hash from each marker
/// 3. Read the removable sidecar graph
/// 4. Keep marked entries and all functions
/// 5. Walk exact spec-ID references from those roots
/// 6. Strip unused specs from contractspecv0 and drop the sidecar graph
///
/// Today markers are only used in contracts written in Rust, leveraging how Rust can eliminate
/// dead code to make markers a good signal for whether an event is published or an error is thrown
/// by reachable contract code. It's not known if the same pattern could be used in other languages,
/// and so it is not a general part of the SEP-48 Contract Interface Specification. Markers are just
/// a mechanism used by the Rust soroban-sdk and the stellar-cli to achieve accurately scoped
/// contract specs.
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Write as _,
};

use soroban_spec_markers::{
    decode_graph_record, generate_marker_for_xdr, generate_spec_id_for_xdr, DecodeGraphRecordError,
    GRAPH_SECTION, MARKER_LEN, MARKER_MAGIC, META_KEY, META_VALUE_V2,
};
pub use soroban_spec_markers::{Marker, SpecGraphEntryKind, SpecId};
use stellar_xdr::curr::{
    Limits, ScMetaEntry, ScSpecEntry, ScSpecTypeDef, ScSpecUdtUnionCaseV0, WriteXdr,
};
use wasmparser::BinaryReaderError;

/// A removable sidecar graph record for one spec entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpecGraphEntry {
    pub kind: SpecGraphEntryKind,
    pub refs: Vec<SpecId>,
}

/// Removable sidecar graph data keyed by spec entry ID.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SpecGraph {
    pub entries: HashMap<SpecId, SpecGraphEntry>,
}

impl SpecGraph {
    /// Inserts a graph entry, merging refs for duplicate records with the same spec ID.
    ///
    /// If a duplicate record has a different kind, the first kind is retained; the mismatch is
    /// reported later when the graph is validated against the reachable spec entry.
    pub fn insert(&mut self, spec_id: SpecId, entry: SpecGraphEntry) {
        let existing = self.entries.entry(spec_id).or_insert(SpecGraphEntry {
            kind: entry.kind,
            refs: Vec::new(),
        });
        for ref_id in entry.refs {
            if !existing.refs.contains(&ref_id) {
                existing.refs.push(ref_id);
            }
        }
    }

    /// Builds a [`SpecGraph`] from a sequence of `(entry, kind, refs)` tuples.
    ///
    /// Each tuple becomes one record keyed by the entry's spec ID, with `refs` resolved to spec
    /// IDs by hashing each referenced entry. Useful for tests and for synthesizing a graph that
    /// mirrors what the SDK macros emit at compile time.
    pub fn from_records<'a, I>(records: I) -> Self
    where
        I: IntoIterator<Item = (&'a ScSpecEntry, SpecGraphEntryKind, Vec<&'a ScSpecEntry>)>,
    {
        let mut graph = Self::default();
        for (entry, kind, refs) in records {
            let spec_id = generate_spec_id_for_entry(entry);
            let refs = refs.into_iter().map(generate_spec_id_for_entry).collect();
            graph.insert(spec_id, SpecGraphEntry { kind, refs });
        }
        graph
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SpecShakingError {
    #[error("reading wasm")]
    ReadWasm(BinaryReaderError),
    #[error("invalid spec graph record magic at offset {offset}")]
    InvalidMagic { offset: usize },
    #[error("unsupported spec graph record version {version} at offset {offset}")]
    UnsupportedVersion { offset: usize, version: u8 },
    #[error("invalid spec graph record kind {kind} at offset {offset}")]
    InvalidKind { offset: usize, kind: u16 },
    #[error("truncated spec graph record at offset {offset}")]
    TruncatedRecord { offset: usize },
    #[error("missing spec graph entry for {entry}")]
    MissingGraphEntry { spec_id: SpecId, entry: String },
    #[error("missing graph reference from {entry} to UDT {type_name}")]
    MissingGraphReference {
        spec_id: SpecId,
        entry: String,
        type_name: String,
    },
    #[error("graph reference from {entry} to UDT {type_name} has unknown spec id {ref_spec_id} and does not match any spec entry")]
    MissingReferencedSpecEntry {
        spec_id: SpecId,
        entry: String,
        type_name: String,
        ref_id: SpecId,
        ref_spec_id: String,
    },
    #[error("graph reference from {entry} to UDT {type_name} resolved to {ref_entry}, which is not a UDT spec entry")]
    ReferencedSpecEntryNotUdt {
        spec_id: SpecId,
        entry: String,
        type_name: String,
        ref_id: SpecId,
        ref_entry: String,
    },
    #[error("spec graph entry for {entry} has kind {actual:?}, expected {expected:?}")]
    GraphEntryKindMismatch {
        spec_id: SpecId,
        entry: String,
        expected: SpecGraphEntryKind,
        actual: SpecGraphEntryKind,
    },
}

/// Returns the spec shaking version indicated by the contract meta entries.
///
/// Looks for an [`ScMetaV0`] entry with key [`META_KEY`]. Returns:
/// - `2` if the value is [`soroban_spec_markers::META_VALUE_V2`] (`"2"`).
/// - `1` otherwise (absent or any other value).
pub fn spec_shaking_version_for_meta(meta: &[ScMetaEntry]) -> u32 {
    for entry in meta {
        match entry {
            ScMetaEntry::ScMetaV0(v0) if v0.key.to_utf8_string_lossy() == META_KEY => {
                if v0.val.to_utf8_string_lossy() == META_VALUE_V2 {
                    return 2;
                }
            }
            _ => {}
        }
    }
    1
}

/// Generates a stable identity for a spec entry.
///
/// # Panics
///
/// Panics if the spec entry cannot be encoded to XDR, which should never happen
/// for valid `ScSpecEntry` values.
pub fn generate_spec_id_for_entry(entry: &ScSpecEntry) -> SpecId {
    let xdr_bytes = entry
        .to_xdr(Limits::none())
        .expect("XDR encoding should not fail");
    generate_spec_id_for_xdr(&xdr_bytes)
}

/// Generates a marker for a spec entry.
///
/// The marker is the magic prefix `SpEcV1` followed by a truncated SHA256
/// (first 8 bytes) of the spec entry's XDR bytes.
///
/// # Panics
///
/// Panics if the spec entry cannot be encoded to XDR, which should never happen
/// for valid `ScSpecEntry` values.
pub fn generate_marker_for_entry(entry: &ScSpecEntry) -> Marker {
    let xdr_bytes = entry
        .to_xdr(Limits::none())
        .expect("XDR encoding should not fail");
    generate_marker_for_xdr(&xdr_bytes)
}

/// Finds all spec markers in a WASM binary's data section.
///
/// The SDK embeds markers in the data section for each event entry that is
/// actually published by reachable contract code. Function roots are derived
/// from `contractspecv0` and do not need markers.
///
/// Marker format:
/// - 6 bytes: `SpEcV1` magic
/// - 8 bytes: truncated SHA256 hash of the spec entry XDR bytes
pub fn find_all(wasm_bytes: &[u8]) -> HashSet<Marker> {
    let mut markers = HashSet::new();

    for payload in wasmparser::Parser::new(0).parse_all(wasm_bytes) {
        let Ok(payload) = payload else { continue };

        if let wasmparser::Payload::DataSection(reader) = payload {
            for data in reader.into_iter().flatten() {
                find_all_in_data(data.data, &mut markers);
            }
        }
    }

    markers
}

/// Finds removable spec graph records in a WASM binary.
pub fn find_graph(wasm_bytes: &[u8]) -> Result<SpecGraph, SpecShakingError> {
    let mut graph = SpecGraph::default();

    for payload in wasmparser::Parser::new(0).parse_all(wasm_bytes) {
        let payload = payload.map_err(SpecShakingError::ReadWasm)?;

        if let wasmparser::Payload::CustomSection(section) = payload {
            if section.name() == GRAPH_SECTION {
                find_graph_in_data(section.data(), &mut graph)?;
            }
        }
    }

    Ok(graph)
}

fn find_graph_in_data(data: &[u8], graph: &mut SpecGraph) -> Result<(), SpecShakingError> {
    let mut offset = 0;
    while offset < data.len() {
        let record = decode_graph_record(&data[offset..])
            .map_err(|err| spec_graph_decode_error(offset, err))?;
        graph.insert(
            record.spec_id,
            SpecGraphEntry {
                kind: record.kind,
                refs: record.refs().collect(),
            },
        );
        offset += record.encoded_len();
    }
    Ok(())
}

fn spec_graph_decode_error(offset: usize, err: DecodeGraphRecordError) -> SpecShakingError {
    match err {
        DecodeGraphRecordError::TruncatedHeader | DecodeGraphRecordError::TruncatedRefs => {
            SpecShakingError::TruncatedRecord { offset }
        }
        DecodeGraphRecordError::InvalidMagic => SpecShakingError::InvalidMagic { offset },
        DecodeGraphRecordError::UnsupportedVersion { version } => {
            SpecShakingError::UnsupportedVersion { offset, version }
        }
        DecodeGraphRecordError::InvalidKind { kind } => {
            SpecShakingError::InvalidKind { offset, kind }
        }
    }
}

/// Finds spec markers in a data segment.
fn find_all_in_data(data: &[u8], markers: &mut HashSet<Marker>) {
    // Marker size is exactly 14 bytes: 6 (magic) + 8 (hash)
    if data.len() < MARKER_LEN {
        return;
    }

    for i in 0..=data.len() - MARKER_LEN {
        // Look for magic bytes
        if data[i..].starts_with(&MARKER_MAGIC) {
            let marker_end = i + MARKER_LEN;
            let mut marker_bytes = [0u8; MARKER_LEN];
            marker_bytes.copy_from_slice(&data[i..marker_end]);
            markers.insert(marker_bytes);
        }
    }
}

/// Filters spec entries using function roots, extra-root markers, and UDT graph reachability.
///
/// Functions are always kept as they define the contract's API. Function input and output UDTs
/// are queued for reachability through the matching function graph record. Non-function entries
/// are kept when their marker is present in the WASM data section, and their referenced UDTs are
/// queued through their graph records. Exact duplicate spec entries are collapsed.
///
/// # Arguments
///
/// * `entries` - The spec entries to filter
/// * `markers` - Extra-root markers extracted from the WASM data section
/// * `graph` - The removable sidecar graph
///
/// # Errors
///
/// Returns an error when a reachable entry references a UDT that cannot be resolved through the
/// graph to an exported spec entry. Empty graphs are accepted for contracts whose reachable
/// entries do not reference any UDTs.
#[allow(clippy::implicit_hasher)]
pub fn filter<'a, I: IntoIterator<Item = ScSpecEntry> + 'a>(
    entries: I,
    markers: &'a HashSet<Marker>,
    graph: &'a SpecGraph,
) -> Result<impl Iterator<Item = ScSpecEntry> + 'a, SpecShakingError> {
    let entries = entries.into_iter().collect::<Vec<_>>();
    let reachable = reachable_entry_indexes_with_graph(&entries, markers, graph)?;
    Ok(filter_reachable_entries(entries, reachable))
}

fn filter_reachable_entries(
    entries: Vec<ScSpecEntry>,
    reachable: HashSet<usize>,
) -> impl Iterator<Item = ScSpecEntry> {
    let mut seen_entries = HashSet::<Vec<u8>>::new();
    entries
        .into_iter()
        .enumerate()
        .filter_map(move |(i, entry)| {
            if !reachable.contains(&i) {
                return None;
            }
            let xdr = entry
                .to_xdr(Limits::none())
                .expect("XDR encoding should not fail");
            seen_entries.insert(xdr).then_some(entry)
        })
}

fn reachable_entry_indexes_with_graph(
    entries: &[ScSpecEntry],
    markers: &HashSet<Marker>,
    graph: &SpecGraph,
) -> Result<HashSet<usize>, SpecShakingError> {
    // The sidecar graph is authoritative for UDT reachability when present. Function entries are
    // always API roots, but their argument and return UDTs are only queued through the matching
    // function graph record.
    let mut entry_indexes_by_id = HashMap::<SpecId, Vec<usize>>::new();
    let mut udt_names_by_id = HashMap::<SpecId, Vec<String>>::new();
    let mut entry_descriptions_by_id = HashMap::<SpecId, String>::new();
    for (i, entry) in entries.iter().enumerate() {
        let spec_id = generate_spec_id_for_entry(entry);
        entry_indexes_by_id.entry(spec_id).or_default().push(i);
        entry_descriptions_by_id
            .entry(spec_id)
            .or_insert_with(|| spec_entry_description(entry));
        if let Some(name) = udt_entry_name(entry) {
            udt_names_by_id.entry(spec_id).or_default().push(name);
        }
    }

    let mut reachable = HashSet::<usize>::new();
    let mut pending_spec_ids = VecDeque::<SpecId>::new();

    for (i, entry) in entries.iter().enumerate() {
        match entry {
            ScSpecEntry::FunctionV0(_) => {
                reachable.insert(i);
                add_graph_refs(
                    entry,
                    graph,
                    &entry_indexes_by_id,
                    &udt_names_by_id,
                    &entry_descriptions_by_id,
                    &mut pending_spec_ids,
                )?;
            }
            _ if markers.contains(&generate_marker_for_entry(entry)) => {
                reachable.insert(i);
                add_graph_refs(
                    entry,
                    graph,
                    &entry_indexes_by_id,
                    &udt_names_by_id,
                    &entry_descriptions_by_id,
                    &mut pending_spec_ids,
                )?;
            }
            _ => {}
        }
    }

    let mut visited_spec_ids = HashSet::<SpecId>::new();
    while let Some(spec_id) = pending_spec_ids.pop_front() {
        if !visited_spec_ids.insert(spec_id) {
            continue;
        }

        let Some(indices) = entry_indexes_by_id.get(&spec_id) else {
            continue;
        };
        for &i in indices {
            if reachable.insert(i) {
                add_graph_refs(
                    &entries[i],
                    graph,
                    &entry_indexes_by_id,
                    &udt_names_by_id,
                    &entry_descriptions_by_id,
                    &mut pending_spec_ids,
                )?;
            }
        }
    }

    Ok(reachable)
}

fn add_graph_refs(
    entry: &ScSpecEntry,
    graph: &SpecGraph,
    entry_indexes_by_id: &HashMap<SpecId, Vec<usize>>,
    udt_names_by_id: &HashMap<SpecId, Vec<String>>,
    entry_descriptions_by_id: &HashMap<SpecId, String>,
    pending_spec_ids: &mut VecDeque<SpecId>,
) -> Result<(), SpecShakingError> {
    let spec_id = generate_spec_id_for_entry(entry);
    let entry_description = spec_entry_description(entry);
    let expected_names = referenced_udt_names(entry);
    if expected_names.is_empty() {
        return Ok(());
    }

    let Some(record) = graph.entries.get(&spec_id) else {
        return Err(SpecShakingError::MissingGraphEntry {
            spec_id,
            entry: entry_description,
        });
    };

    let expected_kind = graph_entry_kind_for_spec(entry);
    if record.kind != expected_kind {
        return Err(SpecShakingError::GraphEntryKindMismatch {
            spec_id,
            entry: entry_description,
            expected: expected_kind,
            actual: record.kind,
        });
    }

    let mut matched_ref_ids = HashSet::<SpecId>::new();
    for expected_name in expected_names {
        let mut matched = false;
        let mut missing_ref_id = None;
        let mut non_udt_ref = None;
        for ref_id in record.refs.iter().copied() {
            if let Some(names) = udt_names_by_id.get(&ref_id) {
                if names.contains(&expected_name) {
                    matched_ref_ids.insert(ref_id);
                    matched = true;
                    break;
                }
            } else if !entry_indexes_by_id.contains_key(&ref_id) {
                missing_ref_id.get_or_insert(ref_id);
            } else if non_udt_ref.is_none() {
                let ref_entry = entry_descriptions_by_id
                    .get(&ref_id)
                    .cloned()
                    .expect("referenced spec entry should have a description");
                non_udt_ref = Some((ref_id, ref_entry));
            }
        }
        if matched {
            continue;
        }
        if let Some(ref_id) = missing_ref_id {
            return Err(SpecShakingError::MissingReferencedSpecEntry {
                spec_id,
                entry: entry_description,
                type_name: expected_name,
                ref_id,
                ref_spec_id: spec_id_hex(ref_id),
            });
        }
        if let Some((ref_id, ref_entry)) = non_udt_ref {
            return Err(SpecShakingError::ReferencedSpecEntryNotUdt {
                spec_id,
                entry: entry_description,
                type_name: expected_name,
                ref_id,
                ref_entry,
            });
        }
        return Err(SpecShakingError::MissingGraphReference {
            spec_id,
            entry: entry_description,
            type_name: expected_name,
        });
    }

    // Only traverse refs that satisfied an expected UDT name. Extra graph refs are tolerated and
    // intentionally not rooted because reachability is defined by public spec coverage.
    pending_spec_ids.extend(matched_ref_ids);
    Ok(())
}

fn graph_entry_kind_for_spec(entry: &ScSpecEntry) -> SpecGraphEntryKind {
    match entry {
        ScSpecEntry::FunctionV0(_) => SpecGraphEntryKind::Function,
        ScSpecEntry::EventV0(_) => SpecGraphEntryKind::Event,
        ScSpecEntry::UdtStructV0(_)
        | ScSpecEntry::UdtUnionV0(_)
        | ScSpecEntry::UdtEnumV0(_)
        | ScSpecEntry::UdtErrorEnumV0(_) => SpecGraphEntryKind::Udt,
    }
}

fn udt_entry_name(entry: &ScSpecEntry) -> Option<String> {
    match entry {
        ScSpecEntry::UdtStructV0(s) => Some(s.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtUnionV0(u) => Some(u.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtErrorEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
        _ => None,
    }
}

fn spec_entry_description(entry: &ScSpecEntry) -> String {
    match entry {
        ScSpecEntry::FunctionV0(f) => format!("function {}", f.name.to_utf8_string_lossy()),
        ScSpecEntry::EventV0(e) => format!("event {}", e.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtStructV0(s) => format!("UDT {}", s.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtUnionV0(u) => format!("UDT {}", u.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtEnumV0(e) => format!("UDT {}", e.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtErrorEnumV0(e) => format!("UDT {}", e.name.to_utf8_string_lossy()),
    }
}

fn spec_id_hex(spec_id: SpecId) -> String {
    let mut hex = String::with_capacity(64);
    for byte in spec_id {
        write!(&mut hex, "{byte:02x}").expect("writing to String should not fail");
    }
    hex
}

fn referenced_udt_names(entry: &ScSpecEntry) -> Vec<String> {
    let mut names = Vec::new();
    match entry {
        ScSpecEntry::FunctionV0(f) => {
            for input in f.inputs.iter() {
                add_type_def_udt_names(&input.type_, &mut names);
            }
            for output in f.outputs.iter() {
                add_type_def_udt_names(output, &mut names);
            }
        }
        ScSpecEntry::EventV0(e) => {
            for param in e.params.iter() {
                add_type_def_udt_names(&param.type_, &mut names);
            }
        }
        ScSpecEntry::UdtStructV0(s) => {
            for field in s.fields.iter() {
                add_type_def_udt_names(&field.type_, &mut names);
            }
        }
        ScSpecEntry::UdtUnionV0(u) => {
            for case in u.cases.iter() {
                if let ScSpecUdtUnionCaseV0::TupleV0(tuple) = case {
                    for type_ in tuple.type_.iter() {
                        add_type_def_udt_names(type_, &mut names);
                    }
                }
            }
        }
        ScSpecEntry::UdtEnumV0(_) | ScSpecEntry::UdtErrorEnumV0(_) => {}
    }
    names
}

fn add_type_def_udt_names(type_: &ScSpecTypeDef, names: &mut Vec<String>) {
    // Keep this traversal in sync with `soroban-sdk-macros/src/shaking.rs::type_id_refs`.
    // This validator mirrors the macro-emitted graph refs for every spec container.
    match type_ {
        ScSpecTypeDef::Udt(udt) => names.push(udt.name.to_utf8_string_lossy()),
        ScSpecTypeDef::Option(option) => add_type_def_udt_names(&option.value_type, names),
        ScSpecTypeDef::Result(result) => {
            add_type_def_udt_names(&result.ok_type, names);
            add_type_def_udt_names(&result.error_type, names);
        }
        ScSpecTypeDef::Vec(vec) => add_type_def_udt_names(&vec.element_type, names),
        ScSpecTypeDef::Map(map) => {
            add_type_def_udt_names(&map.key_type, names);
            add_type_def_udt_names(&map.value_type, names);
        }
        ScSpecTypeDef::Tuple(tuple) => {
            for type_ in tuple.value_types.iter() {
                add_type_def_udt_names(type_, names);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_spec_markers::generate_graph_record;
    use stellar_xdr::curr::{
        ScMetaV0, ScSpecEntry, ScSpecEventDataFormat, ScSpecEventParamLocationV0,
        ScSpecEventParamV0, ScSpecEventV0, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
        ScSpecTypeUdt, ScSpecTypeVec, ScSpecUdtEnumCaseV0, ScSpecUdtEnumV0, ScSpecUdtStructFieldV0,
        ScSpecUdtStructV0, ScSpecUdtUnionCaseTupleV0, ScSpecUdtUnionCaseV0,
        ScSpecUdtUnionCaseVoidV0, ScSpecUdtUnionV0, StringM, VecM,
    };

    fn make_function(name: &str, input_types: Vec<ScSpecTypeDef>) -> ScSpecEntry {
        make_function_with_outputs(name, input_types, vec![])
    }

    fn make_function_with_outputs(
        name: &str,
        input_types: Vec<ScSpecTypeDef>,
        output_types: Vec<ScSpecTypeDef>,
    ) -> ScSpecEntry {
        let inputs = input_types
            .into_iter()
            .enumerate()
            .map(|(i, type_)| ScSpecFunctionInputV0 {
                doc: StringM::default(),
                name: format!("arg{i}").try_into().unwrap(),
                type_,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: name.try_into().unwrap(),
            inputs,
            outputs: output_types.try_into().unwrap(),
        })
    }

    fn udt(name: &str) -> ScSpecTypeDef {
        ScSpecTypeDef::Udt(ScSpecTypeUdt {
            name: name.try_into().unwrap(),
        })
    }

    fn vec_of(type_: ScSpecTypeDef) -> ScSpecTypeDef {
        ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
            element_type: Box::new(type_),
        }))
    }

    fn make_struct(name: &str, field_types: Vec<(&str, ScSpecTypeDef)>) -> ScSpecEntry {
        let fields = field_types
            .into_iter()
            .map(|(field_name, type_)| ScSpecUdtStructFieldV0 {
                doc: StringM::default(),
                name: field_name.try_into().unwrap(),
                type_,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            fields,
        })
    }

    fn make_enum(name: &str) -> ScSpecEntry {
        ScSpecEntry::UdtEnumV0(ScSpecUdtEnumV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            cases: vec![ScSpecUdtEnumCaseV0 {
                doc: StringM::default(),
                name: "Variant".try_into().unwrap(),
                value: 0,
            }]
            .try_into()
            .unwrap(),
        })
    }

    fn make_event(name: &str) -> ScSpecEntry {
        make_event_with_params(name, vec![])
    }

    fn make_event_with_params(name: &str, param_types: Vec<ScSpecTypeDef>) -> ScSpecEntry {
        ScSpecEntry::EventV0(ScSpecEventV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            prefix_topics: VecM::default(),
            params: param_types
                .into_iter()
                .enumerate()
                .map(|(i, type_)| ScSpecEventParamV0 {
                    location: ScSpecEventParamLocationV0::Data,
                    doc: StringM::default(),
                    name: format!("param{i}").try_into().unwrap(),
                    type_,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            data_format: ScSpecEventDataFormat::SingleValue,
        })
    }

    fn make_union(name: &str, case_types: Vec<ScSpecTypeDef>) -> ScSpecEntry {
        ScSpecEntry::UdtUnionV0(ScSpecUdtUnionV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            cases: vec![
                ScSpecUdtUnionCaseV0::VoidV0(ScSpecUdtUnionCaseVoidV0 {
                    doc: StringM::default(),
                    name: "Empty".try_into().unwrap(),
                }),
                ScSpecUdtUnionCaseV0::TupleV0(ScSpecUdtUnionCaseTupleV0 {
                    doc: StringM::default(),
                    name: "Value".try_into().unwrap(),
                    type_: case_types.try_into().unwrap(),
                }),
            ]
            .try_into()
            .unwrap(),
        })
    }

    #[test]
    fn test_generate_marker_for_xdr() {
        let spec_xdr = b"some spec xdr bytes";
        let marker: Marker = generate_marker_for_xdr(spec_xdr);

        // Assert exact marker bytes so that any change to the marker
        // format (prefix, hash algorithm, truncation length) is caught.
        assert_eq!(marker, *b"SpEcV1\xf5\xbe\x3f\x49\x6f\x7b\xbc\xb6");

        // Same input produces same marker
        let marker2 = generate_marker_for_xdr(spec_xdr);
        assert_eq!(marker, marker2);

        // Different input produces different marker
        let different_xdr = b"different spec xdr bytes";
        let different_marker = generate_marker_for_xdr(different_xdr);
        assert_eq!(&different_marker[..6], MARKER_MAGIC.as_slice());
        assert_ne!(marker, different_marker);
    }

    #[test]
    fn test_generate_marker_for_entry() {
        let entry = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: "test".try_into().unwrap(),
            inputs: VecM::default(),
            outputs: VecM::default(),
        });

        let marker = generate_marker_for_entry(&entry);

        // Marker should be 14 bytes (6-byte prefix + 8-byte hash)
        assert_eq!(marker.len(), MARKER_LEN);

        // First 6 bytes should be magic
        assert_eq!(&marker[..6], MARKER_MAGIC.as_slice());

        // Same entry produces same marker
        let marker2 = generate_marker_for_entry(&entry);
        assert_eq!(marker, marker2);

        // Different entry produces different marker
        let entry2 = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: "other".try_into().unwrap(),
            inputs: VecM::default(),
            outputs: VecM::default(),
        });
        let marker3 = generate_marker_for_entry(&entry2);
        assert_ne!(marker, marker3);
    }

    #[test]
    fn test_find_all_in_data() {
        let entry1 = make_event("Transfer");
        let entry2 = make_struct("MyStruct", vec![("field", ScSpecTypeDef::U32)]);

        let encoded1 = generate_marker_for_entry(&entry1);
        let encoded2 = generate_marker_for_entry(&entry2);

        // Concatenate markers with some padding
        let mut data = Vec::new();
        data.extend_from_slice(&[0u8; 16]); // Some leading bytes
        data.extend_from_slice(&encoded1);
        data.extend_from_slice(&[0u8; 8]); // Some padding
        data.extend_from_slice(&encoded2);
        data.extend_from_slice(&[0u8; 16]); // Some trailing bytes

        let mut found = HashSet::new();
        find_all_in_data(&data, &mut found);

        // Both markers should be found
        assert!(found.contains(&generate_marker_for_entry(&entry1)));
        assert!(found.contains(&generate_marker_for_entry(&entry2)));
    }

    #[test]
    fn test_find_graph_in_data() {
        let spec_id = [1u8; 32];
        let ref_id = [2u8; 32];
        let data = generate_graph_record(SpecGraphEntryKind::Function, spec_id, &[ref_id]);

        let mut graph = SpecGraph::default();
        find_graph_in_data(&data, &mut graph).unwrap();

        assert_eq!(
            graph.entries.get(&spec_id),
            Some(&SpecGraphEntry {
                kind: SpecGraphEntryKind::Function,
                refs: vec![ref_id],
            })
        );
    }

    #[test]
    fn test_generate_graph_record_roundtrip() {
        let spec_id = [4u8; 32];
        let refs = [[5u8; 32], [6u8; 32], [7u8; 32]];
        let data = generate_graph_record(SpecGraphEntryKind::Udt, spec_id, &refs);

        let mut graph = SpecGraph::default();
        find_graph_in_data(&data, &mut graph).unwrap();

        assert_eq!(
            graph.entries.get(&spec_id),
            Some(&SpecGraphEntry {
                kind: SpecGraphEntryKind::Udt,
                refs: refs.to_vec(),
            })
        );
    }

    #[test]
    fn test_filter_keeps_marked_events_and_event_param_types() {
        let foo = make_function("foo", vec![ScSpecTypeDef::U32]);
        let transfer_payload = make_struct("TransferPayload", vec![("amount", ScSpecTypeDef::U32)]);
        let transfer_event = make_event_with_params("Transfer", vec![udt("TransferPayload")]);
        let unused_payload = make_struct("UnusedPayload", vec![("amount", ScSpecTypeDef::U32)]);
        let unused_event = make_event("Unused");

        let entries = vec![
            foo.clone(),
            transfer_payload.clone(),
            transfer_event.clone(),
            unused_payload.clone(),
            unused_event.clone(),
        ];
        let graph = SpecGraph::from_records([
            (&foo, SpecGraphEntryKind::Function, vec![]),
            (&transfer_payload, SpecGraphEntryKind::Udt, vec![]),
            (
                &transfer_event,
                SpecGraphEntryKind::Event,
                vec![&transfer_payload],
            ),
            (&unused_payload, SpecGraphEntryKind::Udt, vec![]),
            (&unused_event, SpecGraphEntryKind::Event, vec![]),
        ]);

        let mut markers = HashSet::new();
        markers.insert(generate_marker_for_entry(&transfer_event));

        let filtered: Vec<_> = filter(entries, &markers, &graph).unwrap().collect();

        // Should have: 1 function + 1 used event + 1 event param type.
        assert_eq!(filtered.len(), 3);

        assert!(filtered.iter().any(|e| matches!(e, ScSpecEntry::EventV0(event) if event.name.to_utf8_string_lossy() == "Transfer")));
        assert!(filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "TransferPayload")));
        assert!(!filtered.iter().any(|e| matches!(e, ScSpecEntry::EventV0(event) if event.name.to_utf8_string_lossy() == "Unused")));
        assert!(!filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "UnusedPayload")));
    }

    #[test]
    fn test_filter_keeps_marked_udt_and_graph_refs() {
        let foo = make_function("foo", vec![ScSpecTypeDef::U32]);
        let marked = make_struct("Marked", vec![("child", udt("Child"))]);
        let child = make_struct("Child", vec![("field", ScSpecTypeDef::U32)]);
        let unused = make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]);

        let entries = vec![foo.clone(), marked.clone(), child.clone(), unused.clone()];
        let graph = SpecGraph::from_records([
            (&foo, SpecGraphEntryKind::Function, vec![]),
            (&marked, SpecGraphEntryKind::Udt, vec![&child]),
            (&child, SpecGraphEntryKind::Udt, vec![]),
            (&unused, SpecGraphEntryKind::Udt, vec![]),
        ]);

        let mut markers = HashSet::new();
        markers.insert(generate_marker_for_entry(&marked));

        let filtered: Vec<_> = filter(entries, &markers, &graph).unwrap().collect();

        assert!(filtered
            .iter()
            .any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Marked")));
        assert!(filtered.iter().any(
            |e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Child")
        ));
        assert!(!filtered
            .iter()
            .any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Unused")));
    }

    #[test]
    fn test_filter_accepts_empty_graph_without_udt_refs() {
        let entries = vec![make_function("foo", vec![ScSpecTypeDef::U32])];
        let markers = HashSet::new();
        let graph = SpecGraph::default();

        let filtered: Vec<_> = filter(entries.clone(), &markers, &graph).unwrap().collect();

        assert_eq!(filtered, entries);
    }

    #[test]
    fn test_filter_rejects_missing_graph_entry_for_udt_refs() {
        let entries = vec![
            make_function("foo", vec![udt("Input")]),
            make_struct("Input", vec![("field", ScSpecTypeDef::U32)]),
        ];
        let markers = HashSet::new();
        let graph = SpecGraph::default();

        let Err(err) = filter(entries, &markers, &graph) else {
            panic!("missing graph entry should be rejected");
        };

        assert_eq!(err.to_string(), "missing spec graph entry for function foo");
        assert!(
            matches!(&err, SpecShakingError::MissingGraphEntry { entry, .. } if entry == "function foo")
        );
    }

    #[test]
    fn test_filter_rejects_missing_transitive_graph_entry() {
        let foo = make_function("foo", vec![udt("Root")]);
        let root = make_struct("Root", vec![("leaf", udt("Leaf"))]);
        let leaf = make_struct("Leaf", vec![("field", ScSpecTypeDef::U32)]);
        let entries = vec![foo.clone(), root.clone(), leaf.clone()];
        let graph = SpecGraph::from_records([(&foo, SpecGraphEntryKind::Function, vec![&root])]);
        let markers = HashSet::new();

        let Err(err) = filter(entries, &markers, &graph) else {
            panic!("missing transitive graph entry should be rejected");
        };

        assert!(matches!(err, SpecShakingError::MissingGraphEntry { .. }));
    }

    #[test]
    fn test_filter_rejects_missing_graph_reference() {
        let foo = make_function("foo", vec![udt("Input")]);
        let input = make_struct("Input", vec![("field", ScSpecTypeDef::U32)]);
        let entries = vec![foo.clone(), input];
        let graph = SpecGraph::from_records([(&foo, SpecGraphEntryKind::Function, vec![])]);
        let markers = HashSet::new();

        let Err(err) = filter(entries, &markers, &graph) else {
            panic!("missing graph ref should be rejected");
        };

        assert!(matches!(
            err,
            SpecShakingError::MissingGraphReference { type_name, .. } if type_name == "Input"
        ));
    }

    #[test]
    fn test_filter_rejects_missing_referenced_spec_entry() {
        let foo = make_function("foo", vec![udt("Hidden")]);
        // This mirrors a malformed graph record that refers to an exact type ID without a
        // matching exported UDT entry in contractspecv0.
        let hidden = make_struct("Hidden", vec![("field", ScSpecTypeDef::U32)]);
        let hidden_id = generate_spec_id_for_entry(&hidden);
        let graph = SpecGraph::from_records([(&foo, SpecGraphEntryKind::Function, vec![&hidden])]);
        let markers = HashSet::new();

        let Err(err) = filter(vec![foo], &markers, &graph) else {
            panic!("missing referenced spec entry should be rejected");
        };

        assert!(err
            .to_string()
            .starts_with("graph reference from function foo to UDT Hidden has unknown spec id"));
        assert!(
            matches!(&err, SpecShakingError::MissingReferencedSpecEntry { entry, type_name, ref_id, .. } if entry == "function foo" && type_name == "Hidden" && ref_id == &hidden_id)
        );
    }

    #[test]
    fn test_filter_rejects_referenced_spec_entry_not_udt() {
        let foo = make_function("foo", vec![udt("Input")]);
        let bar = make_function("bar", vec![ScSpecTypeDef::U32]);
        let entries = vec![foo.clone(), bar.clone()];
        let graph = SpecGraph::from_records([(&foo, SpecGraphEntryKind::Function, vec![&bar])]);
        let markers = HashSet::new();

        let Err(err) = filter(entries, &markers, &graph) else {
            panic!("non-UDT graph refs should be rejected");
        };

        assert_eq!(
            err.to_string(),
            "graph reference from function foo to UDT Input resolved to function bar, which is not a UDT spec entry"
        );
        assert!(
            matches!(&err, SpecShakingError::ReferencedSpecEntryNotUdt { entry, type_name, ref_entry, .. } if entry == "function foo" && type_name == "Input" && ref_entry == "function bar")
        );
    }

    #[test]
    fn test_filter_rejects_graph_entry_kind_mismatch() {
        let foo = make_function("foo", vec![udt("Input")]);
        let input = make_struct("Input", vec![("field", ScSpecTypeDef::U32)]);
        let graph = SpecGraph::from_records([(&foo, SpecGraphEntryKind::Event, vec![&input])]);
        let markers = HashSet::new();

        let Err(err) = filter(vec![foo, input], &markers, &graph) else {
            panic!("graph entry kind mismatch should be rejected");
        };

        assert_eq!(
            err.to_string(),
            "spec graph entry for function foo has kind Event, expected Function"
        );
        assert!(
            matches!(&err, SpecShakingError::GraphEntryKindMismatch { entry, expected, actual, .. } if entry == "function foo" && expected == &SpecGraphEntryKind::Function && actual == &SpecGraphEntryKind::Event)
        );
    }

    #[test]
    fn test_filter_removes_all_non_function_entries_without_markers() {
        let foo = make_function("foo", vec![ScSpecTypeDef::U32]);
        let my_struct = make_struct("MyStruct", vec![("field", ScSpecTypeDef::U32)]);
        let my_enum = make_enum("MyEnum");
        let unused_event = make_event("Unused");

        let entries = vec![
            foo.clone(),
            my_struct.clone(),
            my_enum.clone(),
            unused_event.clone(),
        ];
        let graph = SpecGraph::from_records([
            (&foo, SpecGraphEntryKind::Function, vec![]),
            (&my_struct, SpecGraphEntryKind::Udt, vec![]),
            (&my_enum, SpecGraphEntryKind::Udt, vec![]),
            (&unused_event, SpecGraphEntryKind::Event, vec![]),
        ]);

        let markers = HashSet::new(); // No markers

        let filtered: Vec<_> = filter(entries, &markers, &graph).unwrap().collect();

        // Should have: only functions (always kept), no types or events
        assert_eq!(filtered.len(), 1);
        assert!(filtered
            .iter()
            .all(|e| matches!(e, ScSpecEntry::FunctionV0(_))));
    }

    #[test]
    fn test_filter_keeps_function_referenced_types_without_markers() {
        let foo = make_function_with_outputs("foo", vec![udt("Input")], vec![udt("Output")]);
        let input = make_struct("Input", vec![("field", ScSpecTypeDef::U32)]);
        let output = make_struct("Output", vec![("field", ScSpecTypeDef::U32)]);
        let unused_struct = make_struct("UnusedStruct", vec![("field", ScSpecTypeDef::U32)]);
        let unused_event = make_event("UnusedEvent");

        let entries = vec![
            foo.clone(),
            input.clone(),
            output.clone(),
            unused_struct.clone(),
            unused_event.clone(),
        ];
        let graph = SpecGraph::from_records([
            (&foo, SpecGraphEntryKind::Function, vec![&input, &output]),
            (&input, SpecGraphEntryKind::Udt, vec![]),
            (&output, SpecGraphEntryKind::Udt, vec![]),
            (&unused_struct, SpecGraphEntryKind::Udt, vec![]),
            (&unused_event, SpecGraphEntryKind::Event, vec![]),
        ]);

        let markers = HashSet::new();

        let filtered: Vec<_> = filter(entries, &markers, &graph).unwrap().collect();

        // Should have: 1 function + input and output types.
        assert_eq!(filtered.len(), 3);

        let struct_names: Vec<_> = filtered
            .iter()
            .filter_map(|e| {
                if let ScSpecEntry::UdtStructV0(s) = e {
                    Some(s.name.to_utf8_string_lossy())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(struct_names, vec!["Input", "Output"]);
    }

    #[test]
    fn test_filter_keeps_transitive_type_refs() {
        let foo = make_function("foo", vec![udt("Root")]);
        let root = make_struct("Root", vec![("items", vec_of(udt("Leaf")))]);
        let leaf = make_struct("Leaf", vec![("field", ScSpecTypeDef::U32)]);
        let unused = make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]);

        let entries = vec![foo.clone(), root.clone(), leaf.clone(), unused.clone()];
        let graph = SpecGraph::from_records([
            (&foo, SpecGraphEntryKind::Function, vec![&root]),
            (&root, SpecGraphEntryKind::Udt, vec![&leaf]),
            (&leaf, SpecGraphEntryKind::Udt, vec![]),
            (&unused, SpecGraphEntryKind::Udt, vec![]),
        ]);

        let markers = HashSet::new();
        let filtered: Vec<_> = filter(entries, &markers, &graph).unwrap().collect();
        let struct_names: Vec<_> = filtered
            .iter()
            .filter_map(|e| {
                if let ScSpecEntry::UdtStructV0(s) = e {
                    Some(s.name.to_utf8_string_lossy())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(struct_names, vec!["Root", "Leaf"]);
    }

    #[test]
    fn test_filter_keeps_union_case_refs() {
        let foo = make_function("foo", vec![udt("RootUnion")]);
        let root_union = make_union("RootUnion", vec![udt("Leaf")]);
        let leaf = make_struct("Leaf", vec![("field", ScSpecTypeDef::U32)]);
        let unused = make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]);

        let entries = vec![
            foo.clone(),
            root_union.clone(),
            leaf.clone(),
            unused.clone(),
        ];
        let graph = SpecGraph::from_records([
            (&foo, SpecGraphEntryKind::Function, vec![&root_union]),
            (&root_union, SpecGraphEntryKind::Udt, vec![&leaf]),
            (&leaf, SpecGraphEntryKind::Udt, vec![]),
            (&unused, SpecGraphEntryKind::Udt, vec![]),
        ]);

        let markers = HashSet::new();
        let filtered: Vec<_> = filter(entries, &markers, &graph).unwrap().collect();

        assert!(filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtUnionV0(u) if u.name.to_utf8_string_lossy() == "RootUnion")));
        assert!(filtered.iter().any(
            |e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Leaf")
        ));
        assert!(!filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Unused")));
    }

    #[test]
    fn test_filter_removes_exact_duplicate_entries() {
        let foo = make_function("foo", vec![udt("Duplicate")]);
        let duplicate = make_struct("Duplicate", vec![("field", udt("Child"))]);
        let child = make_struct("Child", vec![("field", ScSpecTypeDef::U32)]);
        let unused = make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]);

        let entries = vec![
            foo.clone(),
            duplicate.clone(),
            duplicate.clone(),
            child.clone(),
            unused.clone(),
        ];
        let graph = SpecGraph::from_records([
            (&foo, SpecGraphEntryKind::Function, vec![&duplicate]),
            (&duplicate, SpecGraphEntryKind::Udt, vec![&child]),
            (&child, SpecGraphEntryKind::Udt, vec![]),
            (&unused, SpecGraphEntryKind::Udt, vec![]),
        ]);

        let markers = HashSet::new();
        let filtered: Vec<_> = filter(entries, &markers, &graph).unwrap().collect();

        let duplicate_count = filtered
            .iter()
            .filter(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Duplicate"))
            .count();

        assert_eq!(duplicate_count, 1);
        assert!(filtered.iter().any(
            |e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Child")
        ));
        assert!(!filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Unused")));
    }

    #[test]
    fn test_filter_disambiguates_duplicate_udt_names() {
        let foo = make_function("foo", vec![udt("Duplicate")]);
        let duplicate_struct = make_struct("Duplicate", vec![("field", udt("Child"))]);
        let duplicate_enum = make_enum("Duplicate");
        let child = make_struct("Child", vec![("field", ScSpecTypeDef::U32)]);
        let unused = make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]);

        let entries = vec![
            foo.clone(),
            duplicate_struct.clone(),
            duplicate_enum.clone(),
            child.clone(),
            unused.clone(),
        ];
        let graph = SpecGraph::from_records([
            (&foo, SpecGraphEntryKind::Function, vec![&duplicate_struct]),
            (&duplicate_struct, SpecGraphEntryKind::Udt, vec![&child]),
            (&duplicate_enum, SpecGraphEntryKind::Udt, vec![]),
            (&child, SpecGraphEntryKind::Udt, vec![]),
            (&unused, SpecGraphEntryKind::Udt, vec![]),
        ]);

        let markers = HashSet::new();
        let filtered: Vec<_> = filter(entries, &markers, &graph).unwrap().collect();

        assert!(filtered.iter().any(
            |e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Duplicate")
        ));
        assert!(!filtered.iter().any(
            |e| matches!(e, ScSpecEntry::UdtEnumV0(s) if s.name.to_utf8_string_lossy() == "Duplicate")
        ));
        assert!(filtered.iter().any(
            |e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Child")
        ));
        assert!(!filtered.iter().any(
            |e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Unused")
        ));
    }

    #[test]
    fn test_spec_shaking_version_absent() {
        let meta = vec![];
        assert_eq!(spec_shaking_version_for_meta(&meta), 1);
    }

    #[test]
    fn test_spec_shaking_version_v2() {
        let meta = vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: META_KEY.try_into().unwrap(),
            val: META_VALUE_V2.try_into().unwrap(),
        })];
        assert_eq!(spec_shaking_version_for_meta(&meta), 2);
    }

    #[test]
    fn test_spec_shaking_version_unknown_value() {
        let meta = vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: META_KEY.try_into().unwrap(),
            val: "99".try_into().unwrap(),
        })];
        assert_eq!(spec_shaking_version_for_meta(&meta), 1);
    }
}
