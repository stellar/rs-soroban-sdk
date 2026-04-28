/// Spec shaking: removing unused spec entries from contract WASMs.
///
/// ## Meta
///
/// The `contractmetav0` section of a WASM may contain an [`ScMetaV0`] entry
/// with key [`META_KEY`] (`rssdk_spec_shaking`). The value indicates the spec
/// shaking version:
///
/// - Absent or `"1"` — version 1 (no markers, no shaking possible).
/// - `"2"` — version 2, event-root markers are embedded in the data section.
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
/// publish methods. Function input and output roots are discovered directly from the function
/// entries in `contractspecv0`, and UDT reachability is discovered from exact spec IDs in the
/// removable `contractspecv0.rssdk.graphv0` sidecar when present. If the sidecar is absent, the
/// filter falls back to graph-walking `ScSpecTypeDef` references from those roots by UDT name. In
/// that fallback, if multiple distinct UDT entries have the same name, all matching entries are
/// kept conservatively because `ScSpecTypeDef::Udt` stores only the name. Exact duplicate entries
/// are collapsed during filtering.
///
/// Post-processing tools (e.g. stellar-cli) can:
/// 1. Scan the WASM data section for "SpEcV1" patterns
/// 2. Extract the hash from each marker
/// 3. Keep marked events and all functions
/// 4. Read the removable sidecar graph when present
/// 5. Walk UDT references from those roots
/// 6. Strip unused specs from contractspecv0 and drop the sidecar graph
///
/// Today markers are only used in contracts written in Rust, leveraging how Rust can eliminate
/// dead code to make event markers a good signal for whether an event is published by reachable
/// contract code. It's not known if the same pattern could be used in other languages, and so it is
/// not a general part of the SEP-48 Contract Interface Specification. Markers are just a mechanism
/// used by the Rust soroban-sdk and the stellar-cli to achieve accurately scoped contract specs.
use std::collections::{HashMap, HashSet, VecDeque};

use sha2::{Digest, Sha256};
use stellar_xdr::curr::{
    Limits, ScMetaEntry, ScSpecEntry, ScSpecTypeDef, ScSpecUdtUnionCaseV0, WriteXdr,
};

/// The contract meta key that indicates the spec shaking version.
///
/// Stored in the `contractmetav0` section as an [`ScMetaV0`] entry.
pub const META_KEY: &str = "rssdk_spec_shaking";

/// The meta value for spec shaking version 2.
pub const META_VALUE_V2: &str = "2";

/// The custom section containing removable spec graph records.
pub const GRAPH_SECTION: &str = "contractspecv0.rssdk.graphv0";

/// Magic bytes at the start of each removable spec graph record.
const GRAPH_RECORD_MAGIC: &[u8; 8] = b"SpGrV0\0\0";

/// Length of a spec graph record header before referenced spec IDs.
const GRAPH_RECORD_HEADER_LEN: usize = 52;

/// A stable identity for a spec entry.
pub type SpecId = [u8; 32];

/// The kind of spec entry represented by a spec graph record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpecGraphEntryKind {
    Function,
    Event,
    Udt,
}

impl SpecGraphEntryKind {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Function),
            1 => Some(Self::Event),
            2 => Some(Self::Udt),
            _ => None,
        }
    }
}

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
    /// Inserts a graph entry, merging duplicate records for the same spec ID.
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
}

/// Returns the spec shaking version indicated by the contract meta entries.
///
/// Looks for an [`ScMetaV0`] entry with key [`META_KEY`]. Returns:
/// - `2` if the value is [`META_VALUE_V2`] (`"2"`).
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

/// Magic bytes that identify a spec marker: `SpEcV1`
const MAGIC: &[u8; 6] = b"SpEcV1";

/// Total length of a spec marker (6-byte prefix + 8-byte hash).
const LEN: usize = 14;

/// A spec marker that identifies a spec entry.
///
/// Format: "SpEcV1" prefix (6 bytes) + first 8 bytes of SHA256 hash = 14 bytes total.
pub type Marker = [u8; LEN];

/// Generates a spec marker for spec entry XDR bytes.
pub fn generate_marker_for_xdr(spec_entry_xdr: &[u8]) -> Marker {
    let hash = generate_spec_id_for_xdr(spec_entry_xdr);
    [
        MAGIC[0], MAGIC[1], MAGIC[2], MAGIC[3], MAGIC[4], MAGIC[5], hash[0], hash[1], hash[2],
        hash[3], hash[4], hash[5], hash[6], hash[7],
    ]
}

/// Generates a stable identity for spec entry XDR bytes.
pub fn generate_spec_id_for_xdr(spec_entry_xdr: &[u8]) -> SpecId {
    Sha256::digest(spec_entry_xdr).into()
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
pub fn find_graph(wasm_bytes: &[u8]) -> SpecGraph {
    let mut graph = SpecGraph::default();

    for payload in wasmparser::Parser::new(0).parse_all(wasm_bytes) {
        let Ok(payload) = payload else { continue };

        if let wasmparser::Payload::CustomSection(section) = payload {
            if section.name() == GRAPH_SECTION {
                find_graph_in_data(section.data(), &mut graph);
            }
        }
    }

    graph
}

fn find_graph_in_data(data: &[u8], graph: &mut SpecGraph) {
    let mut offset = 0;
    while offset + GRAPH_RECORD_HEADER_LEN <= data.len() {
        let Some(pos) = data[offset..]
            .windows(GRAPH_RECORD_MAGIC.len())
            .position(|bytes| bytes == GRAPH_RECORD_MAGIC)
        else {
            break;
        };
        offset += pos;

        if offset + GRAPH_RECORD_HEADER_LEN > data.len() {
            break;
        }

        let kind = data[offset + 8];
        let Some(kind) = SpecGraphEntryKind::from_byte(kind) else {
            offset += GRAPH_RECORD_MAGIC.len();
            continue;
        };

        let mut spec_id = [0u8; 32];
        spec_id.copy_from_slice(&data[offset + 16..offset + 48]);

        let ref_count = u32::from_le_bytes([
            data[offset + 48],
            data[offset + 49],
            data[offset + 50],
            data[offset + 51],
        ]) as usize;
        let Some(record_len) = GRAPH_RECORD_HEADER_LEN.checked_add(ref_count.saturating_mul(32))
        else {
            break;
        };
        if offset + record_len > data.len() {
            break;
        }

        let mut refs = Vec::with_capacity(ref_count);
        for i in 0..ref_count {
            let ref_start = offset + GRAPH_RECORD_HEADER_LEN + i * 32;
            let mut ref_id = [0u8; 32];
            ref_id.copy_from_slice(&data[ref_start..ref_start + 32]);
            refs.push(ref_id);
        }

        graph.insert(spec_id, SpecGraphEntry { kind, refs });
        offset += record_len;
    }
}

/// Finds spec markers in a data segment.
fn find_all_in_data(data: &[u8], markers: &mut HashSet<Marker>) {
    // Marker size is exactly 14 bytes: 6 (magic) + 8 (hash)
    if data.len() < LEN {
        return;
    }

    for i in 0..=data.len() - LEN {
        // Look for magic bytes
        if data[i..].starts_with(MAGIC) {
            let marker_end = i + LEN;
            let mut marker_bytes = [0u8; LEN];
            marker_bytes.copy_from_slice(&data[i..marker_end]);
            markers.insert(marker_bytes);
        }
    }
}

/// Filters spec entries using function roots, event markers, and UDT graph reachability.
///
/// Functions are always kept as they define the contract's API. Function input and output
/// type definitions are used as roots for a conservative UDT graph walk. Events are kept only
/// when their marker is present in the WASM data section, and their param types become roots.
/// UDT references are resolved by name; if a name resolves to multiple distinct entries, all
/// entries with that name are kept. Exact duplicate entries are collapsed.
///
/// # Arguments
///
/// * `entries` - The spec entries to filter
/// * `markers` - Event markers extracted from the WASM data section
///
/// # Returns
///
/// Iterator of filtered entries with only used types/events remaining.
#[allow(clippy::implicit_hasher)]
pub fn filter<'a, I: IntoIterator<Item = ScSpecEntry> + 'a>(
    entries: I,
    markers: &'a HashSet<Marker>,
) -> impl Iterator<Item = ScSpecEntry> + 'a {
    let entries = entries.into_iter().collect::<Vec<_>>();
    let reachable = reachable_entry_indexes(&entries, markers);
    filter_reachable_entries(entries, reachable)
}

/// Filters spec entries using a removable sidecar graph when present.
#[allow(clippy::implicit_hasher)]
pub fn filter_with_graph<'a, I: IntoIterator<Item = ScSpecEntry> + 'a>(
    entries: I,
    markers: &'a HashSet<Marker>,
    graph: &'a SpecGraph,
) -> impl Iterator<Item = ScSpecEntry> + 'a {
    let entries = entries.into_iter().collect::<Vec<_>>();
    let reachable = if graph.entries.is_empty() {
        reachable_entry_indexes(&entries, markers)
    } else {
        reachable_entry_indexes_with_graph(&entries, markers, graph)
    };
    filter_reachable_entries(entries, reachable)
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
    event_markers: &HashSet<Marker>,
    graph: &SpecGraph,
) -> HashSet<usize> {
    let mut entry_indexes_by_id = HashMap::<SpecId, Vec<usize>>::new();
    for (i, entry) in entries.iter().enumerate() {
        entry_indexes_by_id
            .entry(generate_spec_id_for_entry(entry))
            .or_default()
            .push(i);
    }

    let mut reachable = HashSet::<usize>::new();
    let mut pending_spec_ids = VecDeque::<SpecId>::new();

    for (i, entry) in entries.iter().enumerate() {
        match entry {
            ScSpecEntry::FunctionV0(_) => {
                reachable.insert(i);
                add_graph_refs(entry, graph, &mut pending_spec_ids);
            }
            ScSpecEntry::EventV0(_)
                if event_markers.contains(&generate_marker_for_entry(entry)) =>
            {
                reachable.insert(i);
                add_graph_refs(entry, graph, &mut pending_spec_ids);
            }
            _ => {}
        }
    }

    let mut visited_spec_ids = HashSet::<SpecId>::new();
    while let Some(spec_id) = pending_spec_ids.pop_front() {
        if !visited_spec_ids.insert(spec_id) {
            continue;
        }

        if let Some(indices) = entry_indexes_by_id.get(&spec_id) {
            for &i in indices {
                reachable.insert(i);
            }
        }

        if let Some(record) = graph.entries.get(&spec_id) {
            pending_spec_ids.extend(record.refs.iter().copied());
        }
    }

    reachable
}

fn add_graph_refs(entry: &ScSpecEntry, graph: &SpecGraph, pending_spec_ids: &mut VecDeque<SpecId>) {
    let spec_id = generate_spec_id_for_entry(entry);
    if let Some(record) = graph.entries.get(&spec_id) {
        pending_spec_ids.extend(record.refs.iter().copied());
    }
}

fn reachable_entry_indexes(
    entries: &[ScSpecEntry],
    event_markers: &HashSet<Marker>,
) -> HashSet<usize> {
    let mut udt_entries_by_name = HashMap::<String, Vec<usize>>::new();
    for (i, entry) in entries.iter().enumerate() {
        if let Some(name) = udt_entry_name(entry) {
            udt_entries_by_name.entry(name).or_default().push(i);
        }
    }

    let mut reachable = HashSet::<usize>::new();
    let mut pending_udt_names = VecDeque::<String>::new();

    for (i, entry) in entries.iter().enumerate() {
        match entry {
            ScSpecEntry::FunctionV0(_) => {
                reachable.insert(i);
                add_entry_type_refs(entry, &mut pending_udt_names);
            }
            ScSpecEntry::EventV0(_)
                if event_markers.contains(&generate_marker_for_entry(entry)) =>
            {
                reachable.insert(i);
                add_entry_type_refs(entry, &mut pending_udt_names);
            }
            _ => {}
        }
    }

    let mut visited_udt_names = HashSet::<String>::new();
    while let Some(name) = pending_udt_names.pop_front() {
        if !visited_udt_names.insert(name.clone()) {
            continue;
        }
        if let Some(indices) = udt_entries_by_name.get(&name) {
            for &i in indices {
                if reachable.insert(i) {
                    add_entry_type_refs(&entries[i], &mut pending_udt_names);
                }
            }
        }
    }

    reachable
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

fn add_entry_type_refs(entry: &ScSpecEntry, pending_udt_names: &mut VecDeque<String>) {
    match entry {
        ScSpecEntry::FunctionV0(f) => {
            for input in f.inputs.iter() {
                add_type_def_refs(&input.type_, pending_udt_names);
            }
            for output in f.outputs.iter() {
                add_type_def_refs(output, pending_udt_names);
            }
        }
        ScSpecEntry::EventV0(e) => {
            for param in e.params.iter() {
                add_type_def_refs(&param.type_, pending_udt_names);
            }
        }
        ScSpecEntry::UdtStructV0(s) => {
            for field in s.fields.iter() {
                add_type_def_refs(&field.type_, pending_udt_names);
            }
        }
        ScSpecEntry::UdtUnionV0(u) => {
            for case in u.cases.iter() {
                if let ScSpecUdtUnionCaseV0::TupleV0(tuple) = case {
                    for type_ in tuple.type_.iter() {
                        add_type_def_refs(type_, pending_udt_names);
                    }
                }
            }
        }
        ScSpecEntry::UdtEnumV0(_) | ScSpecEntry::UdtErrorEnumV0(_) => {}
    }
}

fn add_type_def_refs(type_: &ScSpecTypeDef, pending_udt_names: &mut VecDeque<String>) {
    match type_ {
        ScSpecTypeDef::Udt(udt) => pending_udt_names.push_back(udt.name.to_utf8_string_lossy()),
        ScSpecTypeDef::Option(option) => add_type_def_refs(&option.value_type, pending_udt_names),
        ScSpecTypeDef::Result(result) => {
            add_type_def_refs(&result.ok_type, pending_udt_names);
            add_type_def_refs(&result.error_type, pending_udt_names);
        }
        ScSpecTypeDef::Vec(vec) => add_type_def_refs(&vec.element_type, pending_udt_names),
        ScSpecTypeDef::Map(map) => {
            add_type_def_refs(&map.key_type, pending_udt_names);
            add_type_def_refs(&map.value_type, pending_udt_names);
        }
        ScSpecTypeDef::Tuple(tuple) => {
            for type_ in tuple.value_types.iter() {
                add_type_def_refs(type_, pending_udt_names);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn graph_record_bytes(kind: u8, spec_id: SpecId, refs: &[SpecId]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(GRAPH_RECORD_MAGIC);
        bytes.push(kind);
        bytes.extend_from_slice(&[0; 7]);
        bytes.extend_from_slice(&spec_id);
        bytes.extend_from_slice(&(refs.len() as u32).to_le_bytes());
        for ref_id in refs {
            bytes.extend_from_slice(ref_id);
        }
        bytes
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
        assert_eq!(&different_marker[..6], MAGIC.as_slice());
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
        assert_eq!(marker.len(), LEN);

        // First 6 bytes should be magic
        assert_eq!(&marker[..6], MAGIC.as_slice());

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
    fn test_generate_marker_for_entry_struct() {
        let entry = make_struct("MyStruct", vec![("field", ScSpecTypeDef::U32)]);
        let marker = generate_marker_for_entry(&entry);

        // Marker should be 14 bytes (6-byte prefix + 8-byte hash)
        assert_eq!(marker.len(), LEN);

        // First 6 bytes should be magic
        assert_eq!(&marker[..6], MAGIC.as_slice());

        // Same entry produces same marker
        let marker2 = generate_marker_for_entry(&entry);
        assert_eq!(marker, marker2);

        // Different entry produces different marker
        let entry2 = make_struct("DifferentStruct", vec![("field", ScSpecTypeDef::U32)]);
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
        let data = graph_record_bytes(0, spec_id, &[ref_id]);

        let mut graph = SpecGraph::default();
        find_graph_in_data(&data, &mut graph);

        assert_eq!(
            graph.entries.get(&spec_id),
            Some(&SpecGraphEntry {
                kind: SpecGraphEntryKind::Function,
                refs: vec![ref_id],
            })
        );
    }

    #[test]
    fn test_filter_keeps_marked_events_and_event_param_types() {
        let transfer_payload = make_struct("TransferPayload", vec![("amount", ScSpecTypeDef::U32)]);
        let transfer_event = make_event_with_params("Transfer", vec![udt("TransferPayload")]);

        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            transfer_payload,
            transfer_event.clone(),
            make_struct("UnusedPayload", vec![("amount", ScSpecTypeDef::U32)]),
            make_event("Unused"),
        ];

        let mut markers = HashSet::new();
        markers.insert(generate_marker_for_entry(&transfer_event));

        let filtered: Vec<_> = filter(entries, &markers).collect();

        // Should have: 1 function + 1 used event + 1 event param type.
        assert_eq!(filtered.len(), 3);

        assert!(filtered.iter().any(|e| matches!(e, ScSpecEntry::EventV0(event) if event.name.to_utf8_string_lossy() == "Transfer")));
        assert!(filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "TransferPayload")));
        assert!(!filtered.iter().any(|e| matches!(e, ScSpecEntry::EventV0(event) if event.name.to_utf8_string_lossy() == "Unused")));
        assert!(!filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "UnusedPayload")));
    }

    #[test]
    fn test_filter_removes_all_events_if_no_markers() {
        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            make_event("Transfer"),
            make_event("Mint"),
        ];

        let markers = HashSet::new();

        let filtered: Vec<_> = filter(entries, &markers).collect();

        // Should have: 1 function, 0 events
        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0], ScSpecEntry::FunctionV0(_)));
    }

    #[test]
    fn test_filter_removes_all_types_if_no_markers() {
        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            make_struct("MyStruct", vec![("field", ScSpecTypeDef::U32)]),
            make_enum("MyEnum"),
            make_event("Unused"),
        ];

        let markers = HashSet::new(); // No markers

        let filtered: Vec<_> = filter(entries, &markers).collect();

        // Should have: only functions (always kept), no types or events
        assert_eq!(filtered.len(), 1);
        assert!(filtered
            .iter()
            .all(|e| matches!(e, ScSpecEntry::FunctionV0(_))));
    }

    #[test]
    fn test_filter_keeps_function_referenced_types_without_markers() {
        let entries = vec![
            make_function_with_outputs("foo", vec![udt("Input")], vec![udt("Output")]),
            make_struct("Input", vec![("field", ScSpecTypeDef::U32)]),
            make_struct("Output", vec![("field", ScSpecTypeDef::U32)]),
            make_struct("UnusedStruct", vec![("field", ScSpecTypeDef::U32)]),
            make_event("UnusedEvent"),
        ];

        let markers = HashSet::new();

        let filtered: Vec<_> = filter(entries, &markers).collect();

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
        let entries = vec![
            make_function("foo", vec![udt("Root")]),
            make_struct("Root", vec![("items", vec_of(udt("Leaf")))]),
            make_struct("Leaf", vec![("field", ScSpecTypeDef::U32)]),
            make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]),
        ];

        let markers = HashSet::new();
        let filtered: Vec<_> = filter(entries, &markers).collect();
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
        let entries = vec![
            make_function("foo", vec![udt("RootUnion")]),
            make_union("RootUnion", vec![udt("Leaf")]),
            make_struct("Leaf", vec![("field", ScSpecTypeDef::U32)]),
            make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]),
        ];

        let markers = HashSet::new();
        let filtered: Vec<_> = filter(entries, &markers).collect();

        assert!(filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtUnionV0(u) if u.name.to_utf8_string_lossy() == "RootUnion")));
        assert!(filtered.iter().any(
            |e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Leaf")
        ));
        assert!(!filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Unused")));
    }

    #[test]
    fn test_filter_keeps_duplicate_udt_names_conservatively() {
        let entries = vec![
            make_function("foo", vec![udt("Duplicate")]),
            make_struct("Duplicate", vec![("field", udt("Child"))]),
            make_enum("Duplicate"),
            make_struct("Child", vec![("field", ScSpecTypeDef::U32)]),
            make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]),
        ];

        let markers = HashSet::new();
        let filtered: Vec<_> = filter(entries, &markers).collect();

        let duplicate_count = filtered
            .iter()
            .filter(|e| match e {
                ScSpecEntry::UdtStructV0(s) => s.name.to_utf8_string_lossy() == "Duplicate",
                ScSpecEntry::UdtEnumV0(e) => e.name.to_utf8_string_lossy() == "Duplicate",
                _ => false,
            })
            .count();

        assert_eq!(duplicate_count, 2);
        assert!(filtered.iter().any(
            |e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Child")
        ));
        assert!(!filtered.iter().any(|e| matches!(e, ScSpecEntry::UdtStructV0(s) if s.name.to_utf8_string_lossy() == "Unused")));
    }

    #[test]
    fn test_filter_removes_exact_duplicate_entries() {
        let duplicate = make_struct("Duplicate", vec![("field", udt("Child"))]);
        let entries = vec![
            make_function("foo", vec![udt("Duplicate")]),
            duplicate.clone(),
            duplicate,
            make_struct("Child", vec![("field", ScSpecTypeDef::U32)]),
            make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]),
        ];

        let markers = HashSet::new();
        let filtered: Vec<_> = filter(entries, &markers).collect();

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
    fn test_filter_with_graph_disambiguates_duplicate_udt_names() {
        let function = make_function("foo", vec![udt("Duplicate")]);
        let duplicate_struct = make_struct("Duplicate", vec![("field", udt("Child"))]);
        let duplicate_enum = make_enum("Duplicate");
        let child = make_struct("Child", vec![("field", ScSpecTypeDef::U32)]);
        let unused = make_struct("Unused", vec![("field", ScSpecTypeDef::U32)]);

        let mut graph = SpecGraph::default();
        graph.insert(
            generate_spec_id_for_entry(&function),
            SpecGraphEntry {
                kind: SpecGraphEntryKind::Function,
                refs: vec![generate_spec_id_for_entry(&duplicate_struct)],
            },
        );
        graph.insert(
            generate_spec_id_for_entry(&duplicate_struct),
            SpecGraphEntry {
                kind: SpecGraphEntryKind::Udt,
                refs: vec![generate_spec_id_for_entry(&child)],
            },
        );
        graph.insert(
            generate_spec_id_for_entry(&duplicate_enum),
            SpecGraphEntry {
                kind: SpecGraphEntryKind::Udt,
                refs: vec![],
            },
        );
        graph.insert(
            generate_spec_id_for_entry(&child),
            SpecGraphEntry {
                kind: SpecGraphEntryKind::Udt,
                refs: vec![],
            },
        );

        let entries = vec![function, duplicate_struct, duplicate_enum, child, unused];
        let markers = HashSet::new();
        let filtered: Vec<_> = filter_with_graph(entries, &markers, &graph).collect();

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
    fn test_spec_shaking_version_other_keys() {
        let meta = vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: "rssdkver".try_into().unwrap(),
            val: "1.0.0".try_into().unwrap(),
        })];
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
