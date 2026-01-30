/// Spec marker generation for identifying used spec entries.
///
/// The marker is a byte array in the data section with a distinctive pattern:
/// - 4 bytes: "SpEc" prefix
/// - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
///
/// Markers are embedded in conversion/usage functions with a volatile read. When the type is used,
/// the function is called and the marker is included. When the type is unused, the function is
/// DCE'd along with its marker.
///
/// Post-processing tools (e.g. stellar-cli) can:
/// 1. Scan the WASM data section for "SpEc" patterns
/// 2. Extract the hash from each marker
/// 3. Match against specs in contractspecv0 section (by hashing each spec)
/// 4. Strip unused specs from contractspecv0
///
/// Today markers are only used in contracts written in Rust, leveraging how Rust can eliminate
/// dead code to make the markers are good signal for if a type gets used. It's not known if the
/// same pattern could be used in other languages, and so it is not a general part of the SEP-48
/// Contact Interface Specification. Markers are just a mechanism used by the Rust soroban-sdk and
/// the stellar-cli to achieve accurately scoped contract specs.
use std::collections::HashSet;

use sha2::{Digest, Sha256};
use stellar_xdr::curr::{Limits, ScSpecEntry, WriteXdr};

/// Magic bytes that identify a spec marker: `SpEc`
const SPEC_MARKER_MAGIC: &[u8; 4] = b"SpEc";

/// Total length of a spec marker (4-byte prefix + 8-byte hash).
const SPEC_MARKER_LEN: usize = 12;

/// A spec marker that identifies a spec entry.
///
/// Format: "SpEc" prefix (4 bytes) + first 8 bytes of SHA256 hash = 12 bytes total.
pub type SpecMarker = [u8; SPEC_MARKER_LEN];

/// Generates a spec marker for spec entry XDR bytes.
pub fn generate_for_xdr(spec_entry_xdr: &[u8]) -> SpecMarker {
    let hash: [u8; 32] = Sha256::digest(spec_entry_xdr).into();
    [
        SPEC_MARKER_MAGIC[0],
        SPEC_MARKER_MAGIC[1],
        SPEC_MARKER_MAGIC[2],
        SPEC_MARKER_MAGIC[3],
        hash[0],
        hash[1],
        hash[2],
        hash[3],
        hash[4],
        hash[5],
        hash[6],
        hash[7],
    ]
}

/// Generates a marker for a spec entry.
///
/// The marker is the magic prefix `SpEc` followed by a truncated SHA256
/// (first 8 bytes) of the spec entry's XDR bytes.
///
/// # Panics
///
/// Panics if the spec entry cannot be encoded to XDR, which should never happen
/// for valid `ScSpecEntry` values.
pub fn generate_for_entry(entry: &ScSpecEntry) -> SpecMarker {
    let xdr_bytes = entry
        .to_xdr(Limits::none())
        .expect("XDR encoding should not fail");
    generate_for_xdr(&xdr_bytes)
}

/// Finds all spec markers in a WASM binary's data section.
///
/// The SDK embeds markers in the data section for each spec entry that is
/// actually used in the contract. These markers survive dead code elimination
/// only if the corresponding type/event is used.
///
/// Marker format:
/// - 4 bytes: `SpEc` magic
/// - 8 bytes: truncated SHA256 hash of the spec entry XDR bytes
pub fn find_all(wasm_bytes: &[u8]) -> HashSet<SpecMarker> {
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
fn find_all_in_data(data: &[u8], markers: &mut HashSet<SpecMarker>) {
    // Marker size is exactly 12 bytes: 4 (magic) + 8 (hash)
    if data.len() < SPEC_MARKER_LEN {
        return;
    }

    for i in 0..=data.len() - SPEC_MARKER_LEN {
        // Look for magic bytes
        if data[i..].starts_with(SPEC_MARKER_MAGIC) {
            let marker_end = i + SPEC_MARKER_LEN;
            let mut marker_bytes = [0u8; SPEC_MARKER_LEN];
            marker_bytes.copy_from_slice(&data[i..marker_end]);
            markers.insert(marker_bytes);
        }
    }
}

/// Filters spec entries based on markers found in the WASM data section.
///
/// This removes any spec entries (types, events) that don't have corresponding
/// markers in the data section. The SDK embeds markers for types/events that
/// are actually used, and these markers survive dead code elimination.
///
/// Functions are always kept as they define the contract's API.
///
/// # Arguments
///
/// * `entries` - The spec entries to filter
/// * `markers` - Markers extracted from the WASM data section
///
/// # Returns
///
/// Filtered entries with only used types/events remaining.
#[allow(clippy::implicit_hasher)]
pub fn filter(entries: Vec<ScSpecEntry>, markers: &HashSet<SpecMarker>) -> Vec<ScSpecEntry> {
    entries
        .into_iter()
        .filter(|entry| {
            // Always keep functions - they're the contract's API
            if matches!(entry, ScSpecEntry::FunctionV0(_)) {
                return true;
            }
            // For all other entries (types, events), check if marker exists
            let marker = generate_for_entry(entry);
            markers.contains(&marker)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use stellar_xdr::curr::{
        ScSpecEntry, ScSpecEventDataFormat, ScSpecEventV0, ScSpecFunctionInputV0, ScSpecFunctionV0,
        ScSpecTypeDef, ScSpecUdtEnumCaseV0, ScSpecUdtEnumV0, ScSpecUdtStructFieldV0,
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
    fn test_generate_for_xdr() {
        let spec_xdr = b"some spec xdr bytes";
        let marker: SpecMarker = generate_for_xdr(spec_xdr);

        // Check prefix
        assert_eq!(&marker[..4], SPEC_MARKER_MAGIC);

        // Check total length
        assert_eq!(marker.len(), SPEC_MARKER_LEN);
        assert_eq!(marker.len(), 12);

        // Same input produces same marker
        let marker2 = generate_for_xdr(spec_xdr);
        assert_eq!(marker, marker2);

        // Different input produces different marker
        let different_xdr = b"different spec xdr bytes";
        let different_marker = generate_for_xdr(different_xdr);
        assert_eq!(&different_marker[..4], SPEC_MARKER_MAGIC);
        assert_ne!(marker, different_marker);
    }

    #[test]
    fn test_generate_for_entry() {
        let entry = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: "test".try_into().unwrap(),
            inputs: VecM::default(),
            outputs: VecM::default(),
        });

        let marker = generate_for_entry(&entry);

        // Marker should be 12 bytes
        assert_eq!(marker.len(), SPEC_MARKER_LEN);

        // First 4 bytes should be magic
        assert_eq!(&marker[..4], SPEC_MARKER_MAGIC);

        // Same entry produces same marker
        let marker2 = generate_for_entry(&entry);
        assert_eq!(marker, marker2);

        // Different entry produces different marker
        let entry2 = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: "other".try_into().unwrap(),
            inputs: VecM::default(),
            outputs: VecM::default(),
        });
        let marker3 = generate_for_entry(&entry2);
        assert_ne!(marker, marker3);
    }

    #[test]
    fn test_generate_for_entry_struct() {
        let entry = make_struct("MyStruct", vec![("field", ScSpecTypeDef::U32)]);
        let marker = generate_for_entry(&entry);

        // Marker should be 12 bytes
        assert_eq!(marker.len(), SPEC_MARKER_LEN);

        // First 4 bytes should be magic
        assert_eq!(&marker[..4], SPEC_MARKER_MAGIC);

        // Same entry produces same marker
        let marker2 = generate_for_entry(&entry);
        assert_eq!(marker, marker2);

        // Different entry produces different marker
        let entry2 = make_struct("DifferentStruct", vec![("field", ScSpecTypeDef::U32)]);
        let marker3 = generate_for_entry(&entry2);
        assert_ne!(marker, marker3);
    }

    #[test]
    fn test_find_all_in_data() {
        let entry1 = make_event("Transfer");
        let entry2 = make_struct("MyStruct", vec![("field", ScSpecTypeDef::U32)]);

        let encoded1 = generate_for_entry(&entry1);
        let encoded2 = generate_for_entry(&entry2);

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
        assert!(found.contains(&generate_for_entry(&entry1)));
        assert!(found.contains(&generate_for_entry(&entry2)));
    }

    #[test]
    fn test_filter_keeps_used_events() {
        let transfer_event = make_event("Transfer");
        let mint_event = make_event("Mint");

        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            transfer_event.clone(),
            mint_event.clone(),
            make_event("Unused"),
        ];

        let mut markers = HashSet::new();
        markers.insert(generate_for_entry(&transfer_event));
        markers.insert(generate_for_entry(&mint_event));

        let filtered = filter(entries, &markers);

        // Should have: 1 function + 2 used events
        assert_eq!(filtered.len(), 3);

        let event_names: Vec<_> = filtered
            .iter()
            .filter_map(|e| {
                if let ScSpecEntry::EventV0(event) = e {
                    Some(event.name.to_utf8_string_lossy())
                } else {
                    None
                }
            })
            .collect();

        assert!(event_names.contains(&"Transfer".to_string()));
        assert!(event_names.contains(&"Mint".to_string()));
        assert!(!event_names.contains(&"Unused".to_string()));
    }

    #[test]
    fn test_filter_removes_all_events_if_no_markers() {
        let entries = vec![
            make_function("foo", vec![ScSpecTypeDef::U32]),
            make_event("Transfer"),
            make_event("Mint"),
        ];

        let markers = HashSet::new();

        let filtered = filter(entries, &markers);

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

        let filtered = filter(entries, &markers);

        // Should have: only functions (always kept), no types or events
        assert_eq!(filtered.len(), 1);
        assert!(filtered
            .iter()
            .all(|e| matches!(e, ScSpecEntry::FunctionV0(_))));
    }

    #[test]
    fn test_filter_keeps_types_with_markers() {
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

        let mut markers = HashSet::new();
        markers.insert(generate_for_entry(&used_struct));
        markers.insert(generate_for_entry(&used_enum));
        markers.insert(generate_for_entry(&used_event));

        let filtered = filter(entries, &markers);

        // Should have: 1 function + 1 struct + 1 enum + 1 event
        assert_eq!(filtered.len(), 4);

        // Check specific entries
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
        assert_eq!(struct_names, vec!["UsedStruct"]);

        let enum_names: Vec<_> = filtered
            .iter()
            .filter_map(|e| {
                if let ScSpecEntry::UdtEnumV0(s) = e {
                    Some(s.name.to_utf8_string_lossy())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(enum_names, vec!["UsedEnum"]);

        let event_names: Vec<_> = filtered
            .iter()
            .filter_map(|e| {
                if let ScSpecEntry::EventV0(s) = e {
                    Some(s.name.to_utf8_string_lossy())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(event_names, vec!["UsedEvent"]);
    }
}
