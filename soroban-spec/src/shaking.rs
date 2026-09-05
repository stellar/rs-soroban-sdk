//! Spec shaking: removing unused spec entries from contract WASMs.
//!
//! ## Meta
//!
//! The `contractmetav0` section of a WASM may contain an `ScMetaV0` entry
//! with key [`META_KEY`] (`rssdk_spec_shaking`). The value indicates the spec
//! shaking version:
//!
//! - Absent or unrecognised — version 1 (no markers, no shaking possible).
//! - `"2"` — version 2, every used entry has a marker in the data section, so
//!   [`filter`] shakes on markers alone.
//! - `"3"` — version 3, only events and panicked-with errors have markers, so
//!   [`filter_by_references`] shakes every other type by reachability.
//!
//! The version selects the rules, so a tool reads it before shaking and each
//! wasm is shaken the way it was built. A tool that only knows version 2 would
//! shake every type out of a version 3 wasm, which is why the version is
//! bumped rather than the meaning of `"2"` changed; reading an unrecognised
//! version as 1 is what makes such a tool leave a newer wasm alone.
//!
//! Use [`spec_shaking_version_for_meta`] to determine the version from the
//! contract's meta entries.
//!
//! ## Markers
//!
//! The marker is a byte array in the data section with a distinctive pattern:
//! - 6 bytes: "SpEcV1" prefix
//! - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
//!
//! Markers are embedded in usage functions with a volatile read. When the
//! entry is used, the function is called and the marker is included. When it
//! is unused, the function is DCE'd along with its marker.
//!
//! From version 3, only the entries a spec never references by name carry a
//! marker: events, which nothing in a spec names, and error enums, which a
//! contract may use solely by handing them to `panic_with_error!`. Every other
//! user-defined type is named by whatever references it, so
//! [`filter_by_references`] settles it by reachability and the wasm carries no
//! marker for it. A version 2 wasm carries a marker per type and is shaken by
//! [`filter`] on those markers alone.
//!
//! Post-processing tools (e.g. stellar-cli) can:
//! 1. Scan the WASM data section for "SpEcV1" patterns
//! 2. Extract the hash from each marker
//! 3. Match against specs in contractspecv0 section (by hashing each spec)
//! 4. Strip the specs the version's rules do not keep from contractspecv0
//!
//! Today markers are only used in contracts written in Rust, leveraging how Rust can eliminate
//! dead code to make the markers a good signal for if a type gets used. It's not known if the
//! same pattern could be used in other languages, and so it is not a general part of the SEP-48
//! Contract Interface Specification. Markers are just a mechanism used by the Rust soroban-sdk and
//! the stellar-cli to achieve accurately scoped contract specs.

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "std")]
use stellar_xdr::{
    Limits, ScMetaEntry, ScSpecEntry, ScSpecTypeDef, ScSpecUdtUnionCaseV0, WriteXdr,
};

mod sha256;
use sha256::sha256;

/// The contract meta key that indicates the spec shaking version.
///
/// Stored in the `contractmetav0` section as an [`ScMetaV0`] entry.
pub const META_KEY: &str = "rssdk_spec_shaking";

/// The meta value for spec shaking version 2.
pub const META_VALUE_V2: &str = "2";

/// The meta value for spec shaking version 3.
pub const META_VALUE_V3: &str = "3";

/// The spec shaking version a contract was built with, which selects the rules
/// [`filter`] shakes it by.
///
/// A tool reads the version the contract records rather than assuming the
/// newest it knows, because the version says what a missing marker means. An
/// unrecognised value reads as [`Version::V1`], so a tool that predates a
/// version leaves those contracts' specs alone instead of shaking them by
/// rules that do not apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    /// No markers, so nothing can be shaken.
    V1,
    /// Every used entry carries a marker, so markers alone say what is used.
    V2,
    /// Only events and panicked-with errors carry markers; every other type is
    /// settled by following the references to it.
    V3,
}

/// Returns the spec shaking version indicated by the contract meta entries.
///
/// Looks for an [`ScMetaV0`] entry with key [`META_KEY`]. Returns:
/// - [`Version::V3`] if the value is [`META_VALUE_V3`] (`"3"`).
/// - [`Version::V2`] if the value is [`META_VALUE_V2`] (`"2"`).
/// - [`Version::V1`] otherwise (absent or any other value).
#[cfg(feature = "std")]
#[must_use]
pub fn spec_shaking_version_for_meta(meta: &[ScMetaEntry]) -> Version {
    for entry in meta {
        match entry {
            ScMetaEntry::ScMetaV0(v0) if v0.key.to_utf8_string_lossy() == META_KEY => {
                let val = v0.val.to_utf8_string_lossy();
                if val == META_VALUE_V3 {
                    return Version::V3;
                }
                if val == META_VALUE_V2 {
                    return Version::V2;
                }
            }
            _ => {}
        }
    }
    Version::V1
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
pub const fn generate_marker_for_xdr(spec_entry_xdr: &[u8]) -> Marker {
    let hash = sha256(spec_entry_xdr);
    [
        MAGIC[0], MAGIC[1], MAGIC[2], MAGIC[3], MAGIC[4], MAGIC[5], hash[0], hash[1], hash[2],
        hash[3], hash[4], hash[5], hash[6], hash[7],
    ]
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
#[cfg(feature = "std")]
pub fn generate_marker_for_entry(entry: &ScSpecEntry) -> Marker {
    let xdr_bytes = entry
        .to_xdr(Limits::none())
        .expect("XDR encoding should not fail");
    generate_marker_for_xdr(&xdr_bytes)
}

/// Finds all spec markers in a WASM binary's data section.
///
/// The SDK embeds markers in the data section for each event and error enum
/// that is actually used in the contract. These markers survive dead code
/// elimination only if the corresponding event or error is used.
///
/// Marker format:
/// - 6 bytes: `SpEcV1` magic
/// - 8 bytes: truncated SHA256 hash of the spec entry XDR bytes
#[cfg(feature = "std")]
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

/// Finds spec markers in a data segment.
#[cfg(feature = "std")]
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

/// Filters spec entries down to those the contract actually needs, by the
/// rules of the spec shaking version it was built with.
///
/// The version says which entries carry a marker, and so what a missing marker
/// means, which is why it has to be the contract's own version rather than the
/// newest one known:
///
/// - [`Version::V1`] — nothing carries a marker, so nothing can be shaken and
///   every entry is kept.
/// - [`Version::V2`] — every used entry carries a marker, so a marker is the
///   whole answer. Functions are always kept; every other entry is kept only
///   if the data section carries its marker.
/// - [`Version::V3`] — only the entries a spec never references by name carry
///   a marker, so most types are settled by following references:
///     - Functions are always kept: they define the contract's API.
///     - An event is kept only if the data section carries its marker, which
///       it does only where the contract publishes the event. Nothing
///       references an event, so a marker is the only evidence one is used.
///     - An error enum is kept if the data section carries its marker, which
///       it does where the error is handed to `panic_with_error!`, or if a
///       kept entry references it, as a function returning it in a `Result`
///       does.
///     - Every other user-defined type is kept only if a kept entry references
///       it, following references transitively: a type referenced by a
///       function, a kept event, or another kept type.
///
/// Under [`Version::V3`], references are matched to definitions by the name
/// the spec gives a type, so this holds names as they are, whether qualified
/// or simple.
///
/// # Arguments
///
/// * `entries` - The spec entries to filter
/// * `markers` - Markers extracted from the WASM data section
/// * `version` - The contract's spec shaking version, from
///   [`spec_shaking_version_for_meta`]
///
/// # Returns
///
/// Iterator of the kept entries, in the order they were given.
#[cfg(feature = "std")]
#[allow(clippy::implicit_hasher)]
pub fn filter<I: IntoIterator<Item = ScSpecEntry>>(
    entries: I,
    markers: &HashSet<Marker>,
    version: Version,
) -> impl Iterator<Item = ScSpecEntry> {
    let entries: Vec<ScSpecEntry> = entries.into_iter().collect();
    let keep = match version {
        Version::V1 => vec![true; entries.len()],
        Version::V2 => entries.iter().map(keep_by_marker(markers)).collect(),
        Version::V3 => keep_flags(&entries, markers),
    };
    entries
        .into_iter()
        .zip(keep)
        .filter_map(|(entry, keep)| keep.then_some(entry))
}

/// Whether an entry is kept under the version 2 rules: functions always, and
/// every other entry only if the data section carries its marker.
#[cfg(feature = "std")]
fn keep_by_marker(markers: &HashSet<Marker>) -> impl Fn(&ScSpecEntry) -> bool + '_ {
    move |entry| {
        // Always keep functions - they're the contract's API
        if matches!(entry, ScSpecEntry::FunctionV0(_)) {
            return true;
        }
        // For all other entries (types, events), check if marker exists
        markers.contains(&generate_marker_for_entry(entry))
    }
}

/// Whether each entry is kept, positionally, per the version 3 rules on
/// [`filter`].
#[cfg(feature = "std")]
fn keep_flags(entries: &[ScSpecEntry], markers: &HashSet<Marker>) -> Vec<bool> {
    // The entries that define each type name. A name is normally defined once,
    // but a spec can carry the same type twice, from a library linked in more
    // than one form, and then a reference reaches both.
    let mut defs: HashMap<&[u8], Vec<usize>> = HashMap::new();
    for (i, entry) in entries.iter().enumerate() {
        if let Some(name) = type_name(entry) {
            defs.entry(name).or_default().push(i);
        }
    }

    // Seed with the entries kept on their own account: functions, and the
    // events and errors the data section holds a marker for.
    let mut keep = vec![false; entries.len()];
    let mut pending = Vec::new();
    for (i, entry) in entries.iter().enumerate() {
        let seed = match entry {
            ScSpecEntry::FunctionV0(_) => true,
            ScSpecEntry::EventV0(_) | ScSpecEntry::UdtErrorEnumV0(_) => {
                markers.contains(&generate_marker_for_entry(entry))
            }
            ScSpecEntry::UdtStructV0(_)
            | ScSpecEntry::UdtUnionV0(_)
            | ScSpecEntry::UdtEnumV0(_) => false,
        };
        if seed {
            keep[i] = true;
            pending.push(i);
        }
    }

    // Follow references out of each kept entry, keeping what they name.
    while let Some(i) = pending.pop() {
        for name in referenced_type_names(&entries[i]) {
            for &j in defs.get(name).map_or(&[][..], Vec::as_slice) {
                if !keep[j] {
                    keep[j] = true;
                    pending.push(j);
                }
            }
        }
    }

    keep
}

/// The name of the user-defined type the entry defines, or `None` for an entry
/// that defines no type a reference can name (a function or an event).
#[cfg(feature = "std")]
fn type_name(entry: &ScSpecEntry) -> Option<&[u8]> {
    match entry {
        ScSpecEntry::UdtStructV0(s) => Some(s.name.as_slice()),
        ScSpecEntry::UdtUnionV0(u) => Some(u.name.as_slice()),
        ScSpecEntry::UdtEnumV0(e) => Some(e.name.as_slice()),
        ScSpecEntry::UdtErrorEnumV0(e) => Some(e.name.as_slice()),
        ScSpecEntry::FunctionV0(_) | ScSpecEntry::EventV0(_) => None,
    }
}

/// The names of every user-defined type the entry references.
#[cfg(feature = "std")]
fn referenced_type_names(entry: &ScSpecEntry) -> Vec<&[u8]> {
    let mut names = Vec::new();
    match entry {
        ScSpecEntry::FunctionV0(f) => {
            for input in f.inputs.iter() {
                collect_type_names(&input.type_, &mut names);
            }
            for output in f.outputs.iter() {
                collect_type_names(output, &mut names);
            }
        }
        ScSpecEntry::UdtStructV0(s) => {
            for field in s.fields.iter() {
                collect_type_names(&field.type_, &mut names);
            }
        }
        ScSpecEntry::UdtUnionV0(u) => {
            for case in u.cases.iter() {
                if let ScSpecUdtUnionCaseV0::TupleV0(t) = case {
                    for type_ in t.type_.iter() {
                        collect_type_names(type_, &mut names);
                    }
                }
            }
        }
        ScSpecEntry::EventV0(e) => {
            for param in e.params.iter() {
                collect_type_names(&param.type_, &mut names);
            }
        }
        ScSpecEntry::UdtEnumV0(_) | ScSpecEntry::UdtErrorEnumV0(_) => {}
    }
    names
}

/// Collects the name of every user-defined type the type def references,
/// descending through the containers that hold other types.
#[cfg(feature = "std")]
fn collect_type_names<'a>(type_: &'a ScSpecTypeDef, names: &mut Vec<&'a [u8]>) {
    match type_ {
        ScSpecTypeDef::Udt(u) => names.push(u.name.as_slice()),
        ScSpecTypeDef::Option(o) => collect_type_names(&o.value_type, names),
        ScSpecTypeDef::Result(r) => {
            collect_type_names(&r.ok_type, names);
            collect_type_names(&r.error_type, names);
        }
        ScSpecTypeDef::Vec(v) => collect_type_names(&v.element_type, names),
        ScSpecTypeDef::Map(m) => {
            collect_type_names(&m.key_type, names);
            collect_type_names(&m.value_type, names);
        }
        ScSpecTypeDef::Tuple(t) => {
            for value_type in t.value_types.iter() {
                collect_type_names(value_type, names);
            }
        }
        _ => {}
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use stellar_xdr::{
        ScMetaV0, ScSpecEntry, ScSpecEventDataFormat, ScSpecEventParamLocationV0,
        ScSpecEventParamV0, ScSpecEventV0, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
        ScSpecTypeOption, ScSpecTypeResult, ScSpecTypeUdt, ScSpecTypeVec, ScSpecUdtEnumCaseV0,
        ScSpecUdtEnumV0, ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0, ScSpecUdtStructFieldV0,
        ScSpecUdtStructV0, StringM, VecM,
    };

    fn make_function(name: &str, input_types: Vec<ScSpecTypeDef>) -> ScSpecEntry {
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
            outputs: VecM::default(),
        })
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

    fn make_function_with_output(name: &str, output: ScSpecTypeDef) -> ScSpecEntry {
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: name.try_into().unwrap(),
            inputs: VecM::default(),
            outputs: vec![output].try_into().unwrap(),
        })
    }

    fn make_error_enum(name: &str) -> ScSpecEntry {
        ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            cases: vec![ScSpecUdtErrorEnumCaseV0 {
                doc: StringM::default(),
                name: "Case".try_into().unwrap(),
                value: 1,
            }]
            .try_into()
            .unwrap(),
        })
    }

    fn udt(name: &str) -> ScSpecTypeDef {
        ScSpecTypeDef::Udt(ScSpecTypeUdt {
            name: name.try_into().unwrap(),
        })
    }

    fn struct_names(entries: &[ScSpecEntry]) -> Vec<String> {
        entries
            .iter()
            .filter_map(|e| match e {
                ScSpecEntry::UdtStructV0(s) => Some(s.name.to_utf8_string_lossy()),
                _ => None,
            })
            .collect()
    }

    fn error_enum_names(entries: &[ScSpecEntry]) -> Vec<String> {
        entries
            .iter()
            .filter_map(|e| match e {
                ScSpecEntry::UdtErrorEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
                _ => None,
            })
            .collect()
    }

    fn event_names(entries: &[ScSpecEntry]) -> Vec<String> {
        entries
            .iter()
            .filter_map(|e| match e {
                ScSpecEntry::EventV0(e) => Some(e.name.to_utf8_string_lossy()),
                _ => None,
            })
            .collect()
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
                    doc: StringM::default(),
                    name: format!("p{i}").try_into().unwrap(),
                    type_,
                    location: ScSpecEventParamLocationV0::Data,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            data_format: ScSpecEventDataFormat::SingleValue,
        })
    }

    fn make_event(name: &str) -> ScSpecEntry {
        ScSpecEntry::EventV0(ScSpecEventV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            prefix_topics: VecM::default(),
            params: VecM::default(),
            data_format: ScSpecEventDataFormat::SingleValue,
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
    fn test_filter_v1_keeps_everything() {
        // Version 1 contracts carry no markers, so nothing can be shaken and
        // an entry without a marker is not evidence of anything.
        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            make_struct("Unreferenced", vec![("field", ScSpecTypeDef::U32)]),
            make_event("Unpublished"),
        ];

        let filtered: Vec<_> = filter(entries.clone(), &HashSet::new(), Version::V1).collect();

        assert_eq!(filtered, entries);
    }

    #[test]
    fn test_filter_v2_keeps_entries_with_markers() {
        // Version 2 shaking: every entry carries a marker, so a marker is the
        // whole answer and no reference is followed.
        let used_struct = make_struct("UsedStruct", vec![("field", ScSpecTypeDef::U32)]);
        let used_enum = make_enum("UsedEnum");
        let used_event = make_event("UsedEvent");

        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            used_struct.clone(),
            make_struct("UnusedStruct", vec![("field", ScSpecTypeDef::U32)]),
            used_enum.clone(),
            make_enum("UnusedEnum"),
            used_event.clone(),
            make_event("UnusedEvent"),
        ];

        let markers = HashSet::from([
            generate_marker_for_entry(&used_struct),
            generate_marker_for_entry(&used_enum),
            generate_marker_for_entry(&used_event),
        ]);

        let filtered: Vec<_> = filter(entries, &markers, Version::V2).collect();

        assert_eq!(filtered.len(), 4);
        assert_eq!(struct_names(&filtered), ["UsedStruct"]);
        assert_eq!(event_names(&filtered), ["UsedEvent"]);
    }

    #[test]
    fn test_filter_v2_removes_everything_but_functions_without_markers() {
        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            make_struct("MyStruct", vec![("field", ScSpecTypeDef::U32)]),
            make_enum("MyEnum"),
            make_event("Unused"),
        ];

        let filtered: Vec<_> = filter(entries, &HashSet::new(), Version::V2).collect();

        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0], ScSpecEntry::FunctionV0(_)));
    }

    #[test]
    fn test_filter_v2_ignores_references_a_marker_does_not_back() {
        // The version 2 rules do not follow references: a type a function
        // names is still dropped without a marker of its own. This is what
        // makes the version, not the algorithm, the thing that has to be
        // right for a given wasm.
        let entries = vec![
            make_function("foo", vec![udt("Referenced")]),
            make_struct("Referenced", vec![("field", ScSpecTypeDef::U32)]),
        ];

        let filtered: Vec<_> = filter(entries, &HashSet::new(), Version::V2).collect();

        assert_eq!(struct_names(&filtered), Vec::<String>::new());
    }

    #[test]
    fn test_filter_v3_keeps_used_events() {
        let transfer_event = make_event("Transfer");
        let mint_event = make_event("Mint");

        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            transfer_event.clone(),
            mint_event.clone(),
            make_event("Unused"),
        ];

        let mut markers = HashSet::new();
        markers.insert(generate_marker_for_entry(&transfer_event));
        markers.insert(generate_marker_for_entry(&mint_event));

        let filtered: Vec<_> = filter(entries, &markers, Version::V3).collect();

        // Should have: 1 function + 2 used events
        assert_eq!(filtered.len(), 3);
        assert_eq!(event_names(&filtered), ["Transfer", "Mint"]);
    }

    #[test]
    fn test_filter_v3_removes_all_events_if_no_markers() {
        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            make_event("Transfer"),
            make_event("Mint"),
        ];

        let markers = HashSet::new();

        let filtered: Vec<_> = filter(entries, &markers, Version::V3).collect();

        // Should have: 1 function, 0 events
        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0], ScSpecEntry::FunctionV0(_)));
    }

    #[test]
    fn test_filter_v3_removes_types_no_entry_references() {
        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            make_struct("MyStruct", vec![("field", ScSpecTypeDef::U32)]),
            make_enum("MyEnum"),
            make_event("Unused"),
        ];

        let markers = HashSet::new();

        let filtered: Vec<_> = filter(entries, &markers, Version::V3).collect();

        // Should have: only the function. Nothing names the types, and the
        // event has no marker.
        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0], ScSpecEntry::FunctionV0(_)));
    }

    #[test]
    fn test_filter_v3_keeps_a_type_a_function_references_without_a_marker() {
        // A type is kept because a function names it, not because the data
        // section holds a marker for it: types carry no markers at all.
        let entries = vec![
            make_function("foo", vec![udt("UsedStruct")]),
            make_struct("UsedStruct", vec![("field", ScSpecTypeDef::U32)]),
            make_struct("UnusedStruct", vec![("field", ScSpecTypeDef::U32)]),
        ];

        let filtered: Vec<_> = filter(entries, &HashSet::new(), Version::V3).collect();

        assert_eq!(struct_names(&filtered), ["UsedStruct"]);
    }

    #[test]
    fn test_filter_v3_keeps_a_type_a_function_references_through_a_container() {
        // A reference nested in a container still names the type.
        let entries = vec![
            make_function(
                "foo",
                vec![ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
                    element_type: Box::new(ScSpecTypeDef::Option(Box::new(ScSpecTypeOption {
                        value_type: Box::new(udt("Nested")),
                    }))),
                }))],
            ),
            make_struct("Nested", vec![("field", ScSpecTypeDef::U32)]),
        ];

        let filtered: Vec<_> = filter(entries, &HashSet::new(), Version::V3).collect();

        assert_eq!(struct_names(&filtered), ["Nested"]);
    }

    #[test]
    fn test_filter_v3_follows_references_between_types() {
        // Reachability is transitive: a function names the outer type, which
        // names the middle type, which names the inner one.
        let entries = vec![
            make_function("foo", vec![udt("Outer")]),
            make_struct("Outer", vec![("field", udt("Middle"))]),
            make_struct("Middle", vec![("field", udt("Inner"))]),
            make_struct("Inner", vec![("field", ScSpecTypeDef::U32)]),
            make_struct("Orphan", vec![("field", udt("AlsoOrphan"))]),
            make_struct("AlsoOrphan", vec![("field", ScSpecTypeDef::U32)]),
        ];

        let filtered: Vec<_> = filter(entries, &HashSet::new(), Version::V3).collect();

        assert_eq!(struct_names(&filtered), ["Outer", "Middle", "Inner"]);
    }

    #[test]
    fn test_filter_v3_follows_a_reference_cycle_between_types() {
        // A recursive definition must not send the walk round forever.
        let entries = vec![
            make_function("foo", vec![udt("Root")]),
            make_struct("Root", vec![("field", udt("Node"))]),
            make_struct("Node", vec![("field", udt("Root"))]),
        ];

        let filtered: Vec<_> = filter(entries, &HashSet::new(), Version::V3).collect();

        assert_eq!(struct_names(&filtered), ["Root", "Node"]);
    }

    #[test]
    fn test_filter_v3_keeps_a_type_a_kept_event_references() {
        // A published event carries the types its params name along with it,
        // and an unpublished one takes them nowhere.
        let published = make_event_with_params("Published", vec![udt("InPublished")]);
        let entries = vec![
            published.clone(),
            make_event_with_params("Unpublished", vec![udt("InUnpublished")]),
            make_struct("InPublished", vec![("field", ScSpecTypeDef::U32)]),
            make_struct("InUnpublished", vec![("field", ScSpecTypeDef::U32)]),
        ];

        let markers = HashSet::from([generate_marker_for_entry(&published)]);

        let filtered: Vec<_> = filter(entries, &markers, Version::V3).collect();

        assert_eq!(event_names(&filtered), ["Published"]);
        assert_eq!(struct_names(&filtered), ["InPublished"]);
    }

    #[test]
    fn test_filter_v3_keeps_an_error_with_a_marker_nothing_references() {
        // An error handed to `panic_with_error!` is named by nothing in the
        // spec, so its marker is the only evidence it is used.
        let panicked = make_error_enum("Panicked");
        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            panicked.clone(),
            make_error_enum("Unused"),
        ];

        let markers = HashSet::from([generate_marker_for_entry(&panicked)]);

        let filtered: Vec<_> = filter(entries, &markers, Version::V3).collect();

        assert_eq!(error_enum_names(&filtered), ["Panicked"]);
    }

    #[test]
    fn test_filter_v3_keeps_an_error_a_function_references_without_a_marker() {
        // An error a function returns is named by the spec, so it is kept
        // whether or not the contract also panics with it.
        let entries = vec![
            make_function_with_output(
                "foo",
                ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U32),
                    error_type: Box::new(udt("ReturnedError")),
                })),
            ),
            make_error_enum("ReturnedError"),
            make_error_enum("UnusedError"),
        ];

        let filtered: Vec<_> = filter(entries, &HashSet::new(), Version::V3).collect();

        assert_eq!(error_enum_names(&filtered), ["ReturnedError"]);
    }

    #[test]
    fn test_filter_v3_keeps_every_definition_of_a_referenced_name() {
        // A spec can carry the same type twice, from a library linked in more
        // than one form. A reference to the name reaches both, and the caller
        // deduplicates identical entries afterwards.
        let entries = vec![
            make_function("foo", vec![udt("Twice")]),
            make_struct("Twice", vec![("a", ScSpecTypeDef::U32)]),
            make_struct("Twice", vec![("b", ScSpecTypeDef::U32)]),
        ];

        let filtered: Vec<_> = filter(entries, &HashSet::new(), Version::V3).collect();

        assert_eq!(struct_names(&filtered), ["Twice", "Twice"]);
    }

    #[test]
    fn test_filter_v3_matches_a_reference_by_its_qualified_name() {
        // Names are matched as they are: a qualified reference names the
        // qualified definition, and not a simple name that ends the same way.
        let entries = vec![
            make_function("foo", vec![udt("mycrate::mymod::MyType")]),
            make_struct("mycrate::mymod::MyType", vec![("a", ScSpecTypeDef::U32)]),
            make_struct("MyType", vec![("b", ScSpecTypeDef::U32)]),
        ];

        let filtered: Vec<_> = filter(entries, &HashSet::new(), Version::V3).collect();

        assert_eq!(struct_names(&filtered), ["mycrate::mymod::MyType"]);
    }

    #[test]
    fn test_spec_shaking_version_absent() {
        let meta = vec![];
        assert_eq!(spec_shaking_version_for_meta(&meta), Version::V1);
    }

    #[test]
    fn test_spec_shaking_version_other_keys() {
        let meta = vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: "rssdkver".try_into().unwrap(),
            val: "1.0.0".try_into().unwrap(),
        })];
        assert_eq!(spec_shaking_version_for_meta(&meta), Version::V1);
    }

    #[test]
    fn test_spec_shaking_version_v2() {
        let meta = vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: META_KEY.try_into().unwrap(),
            val: META_VALUE_V2.try_into().unwrap(),
        })];
        assert_eq!(spec_shaking_version_for_meta(&meta), Version::V2);
    }

    #[test]
    fn test_spec_shaking_version_v3() {
        let meta = vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: META_KEY.try_into().unwrap(),
            val: META_VALUE_V3.try_into().unwrap(),
        })];
        assert_eq!(spec_shaking_version_for_meta(&meta), Version::V3);
    }

    #[test]
    fn test_spec_shaking_version_unknown_value() {
        let meta = vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: META_KEY.try_into().unwrap(),
            val: "99".try_into().unwrap(),
        })];
        assert_eq!(spec_shaking_version_for_meta(&meta), Version::V1);
    }
}

#[cfg(all(test, feature = "std"))]
mod sha256_tests {
    use super::{generate_marker_for_xdr, sha256};

    /// The const SHA-256 must agree with `sha2` exactly, including across block
    /// boundaries where the padding lands in a different block to the message.
    #[test]
    fn sha256_matches_sha2() {
        use sha2::{Digest, Sha256};
        // Lengths either side of the 64-byte block size and the 55/56-byte
        // boundary where the length field no longer fits in the final block.
        for len in [
            0usize, 1, 54, 55, 56, 57, 63, 64, 65, 119, 120, 127, 128, 129, 1000,
        ] {
            let input: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
            let expected: [u8; 32] = Sha256::digest(&input).into();
            assert_eq!(sha256(&input), expected, "mismatch at len {len}");
        }
    }

    /// Evaluatable at compile time, which is what lets macro-generated code
    /// derive the marker from the same const-encoded spec bytes it embeds.
    #[test]
    fn generate_marker_for_xdr_is_const() {
        const M: [u8; 14] = generate_marker_for_xdr(b"abc");
        assert_eq!(&M[..6], b"SpEcV1");
        // SHA-256("abc") = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
        assert_eq!(&M[6..], &[0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea]);
    }
}
