/// Spec shaking: removing unused spec entries from contract WASMs.
///
/// ## Meta
///
/// The `contractmetav0` section of a WASM may contain an [`ScMetaV0`] entry
/// with key [`META_KEY`] (`rssdk_spec_shaking`). The value indicates the spec
/// shaking version:
///
/// - Absent or `"1"` — version 1 (no markers, no shaking possible).
/// - `"2"` — version 2, markers are embedded in the data section.
///
/// Use [`spec_shaking_version_for_meta`] to determine the version from the
/// contract's meta entries.
///
/// ## Markers (version 2)
///
/// The marker is a byte array in the data section with a distinctive pattern:
/// - 6 bytes: "SpEcV1" prefix
/// - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
///
/// Markers are embedded in conversion/usage functions with a volatile read. When the type is used,
/// the function is called and the marker is included. When the type is unused, the function is
/// DCE'd along with its marker.
///
/// Post-processing tools (e.g. stellar-cli) can:
/// 1. Scan the WASM data section for "SpEcV1" patterns
/// 2. Extract the hash from each marker
/// 3. Match against specs in contractspecv0 section (by hashing each spec)
/// 4. Strip unused specs from contractspecv0
///
/// Today markers are only used in contracts written in Rust, leveraging how Rust can eliminate
/// dead code to make the markers a good signal for if a type gets used. It's not known if the
/// same pattern could be used in other languages, and so it is not a general part of the SEP-48
/// Contract Interface Specification. Markers are just a mechanism used by the Rust soroban-sdk and
/// the stellar-cli to achieve accurately scoped contract specs.

#[cfg(feature = "std")]
use std::collections::HashSet;

#[cfg(feature = "std")]
use stellar_xdr::{Limits, ScMetaEntry, ScSpecEntry, WriteXdr};

mod sha256;
use sha256::sha256;

/// The contract meta key that indicates the spec shaking version.
///
/// Stored in the `contractmetav0` section as an [`ScMetaV0`] entry.
pub const META_KEY: &str = "rssdk_spec_shaking";

/// The meta value for spec shaking version 2.
pub const META_VALUE_V2: &str = "2";

/// Returns the spec shaking version indicated by the contract meta entries.
///
/// Looks for an [`ScMetaV0`] entry with key [`META_KEY`]. Returns:
/// - `2` if the value is [`META_VALUE_V2`] (`"2"`).
/// - `1` otherwise (absent or any other value).
#[cfg(feature = "std")]
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
/// The SDK embeds markers in the data section for each spec entry that is
/// actually used in the contract. These markers survive dead code elimination
/// only if the corresponding type/event is used.
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
/// Iterator of filtered entries with only used types/events remaining.
#[cfg(feature = "std")]
#[allow(clippy::implicit_hasher)]
pub fn filter<'a, I: IntoIterator<Item = ScSpecEntry> + 'a>(
    entries: I,
    markers: &'a HashSet<Marker>,
) -> impl Iterator<Item = ScSpecEntry> + 'a {
    entries.into_iter().filter(move |entry| {
        // Always keep functions - they're the contract's API
        if matches!(entry, ScSpecEntry::FunctionV0(_)) {
            return true;
        }
        // For all other entries (types, events), check if marker exists
        let marker = generate_marker_for_entry(entry);
        markers.contains(&marker)
    })
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use stellar_xdr::{
        ScMetaV0, ScSpecEntry, ScSpecEventDataFormat, ScSpecEventV0, ScSpecFunctionInputV0,
        ScSpecFunctionV0, ScSpecTypeDef, ScSpecUdtEnumCaseV0, ScSpecUdtEnumV0,
        ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM, VecM,
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
        markers.insert(generate_marker_for_entry(&transfer_event));
        markers.insert(generate_marker_for_entry(&mint_event));

        let filtered: Vec<_> = filter(entries, &markers).collect();

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
        markers.insert(generate_marker_for_entry(&used_struct));
        markers.insert(generate_marker_for_entry(&used_enum));
        markers.insert(generate_marker_for_entry(&used_event));

        let filtered: Vec<_> = filter(entries, &markers).collect();

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

#[cfg(test)]
mod sha256_tests {
    use super::{generate_marker_for_xdr, sha256};
    use std::vec::Vec;

    /// The const SHA-256 must produce known digests, including across block
    /// boundaries where the padding lands in a different block to the message.
    /// Each input is `len` bytes of `i % 251`, and each expected digest was
    /// computed independently of this crate.
    #[test]
    fn sha256_matches_known_digests() {
        // Lengths either side of the 64-byte block size and the 55/56-byte
        // boundary where the length field no longer fits in the final block.
        let cases: &[(usize, &[u8; 32])] = &[
            (
                0,
                b"\xe3\xb0\xc4\x42\x98\xfc\x1c\x14\x9a\xfb\xf4\xc8\x99\x6f\xb9\x24\
                  \x27\xae\x41\xe4\x64\x9b\x93\x4c\xa4\x95\x99\x1b\x78\x52\xb8\x55",
            ),
            (
                1,
                b"\x6e\x34\x0b\x9c\xff\xb3\x7a\x98\x9c\xa5\x44\xe6\xbb\x78\x0a\x2c\
                  \x78\x90\x1d\x3f\xb3\x37\x38\x76\x85\x11\xa3\x06\x17\xaf\xa0\x1d",
            ),
            (
                54,
                b"\x67\x5f\x28\xac\xc0\xb9\x0a\x72\xd1\xc3\xa5\x70\xfe\x83\xac\x56\
                  \x55\x55\xdb\x35\x8c\xf0\x18\x26\xdc\x8e\xef\xb2\xbf\x7c\xa0\xf3",
            ),
            (
                55,
                b"\x46\x3e\xb2\x8e\x72\xf8\x2e\x0a\x96\xc0\xa4\xcc\x53\x69\x0c\x57\
                  \x12\x81\x13\x1f\x67\x2a\xa2\x29\xe0\xd4\x5a\xe5\x9b\x59\x8b\x59",
            ),
            (
                56,
                b"\xda\x2a\xe4\xd6\xb3\x67\x48\xf2\xa3\x18\xf2\x3e\x7a\xb1\xdf\xdf\
                  \x45\xac\xdc\x9d\x04\x9b\xd8\x0e\x59\xde\x82\xa6\x08\x95\xf5\x62",
            ),
            (
                57,
                b"\x2f\xe7\x41\xaf\x80\x1c\xc2\x38\x60\x2a\xc0\xec\x6a\x7b\x0c\x3a\
                  \x8a\x87\xc7\xfc\x7d\x7f\x02\xa3\xfe\x03\xd1\xc1\x2e\xac\x4d\x8f",
            ),
            (
                63,
                b"\x29\xaf\x26\x86\xfd\x53\x37\x4a\x36\xb0\x84\x66\x94\xcc\x34\x21\
                  \x77\xe4\x28\xd1\x64\x75\x15\xf0\x78\x78\x4d\x69\xcd\xb9\xe4\x88",
            ),
            (
                64,
                b"\xfd\xea\xb9\xac\xf3\x71\x03\x62\xbd\x26\x58\xcd\xc9\xa2\x9e\x8f\
                  \x9c\x75\x7f\xcf\x98\x11\x60\x3a\x8c\x44\x7c\xd1\xd9\x15\x11\x08",
            ),
            (
                65,
                b"\x4b\xfd\x2c\x8b\x6f\x1e\xec\x7a\x2a\xfe\xb4\x8b\x93\x4e\xe4\xb2\
                  \x69\x41\x82\x02\x7e\x6d\x0f\xc0\x75\x07\x4f\x2f\xab\xb3\x17\x81",
            ),
            (
                119,
                b"\xda\x18\x79\x7e\xd7\xc3\xa7\x77\xf0\x84\x7f\x42\x97\x24\xa2\xd8\
                  \xcd\x51\x38\xe6\xed\x28\x95\xc3\xfa\x1a\x6d\x39\xd1\x8f\x7e\xc6",
            ),
            (
                120,
                b"\xf5\x2b\x23\xdb\x1f\xbb\x6d\xed\x89\xef\x42\xa2\x3c\xe0\xc8\x92\
                  \x2c\x45\xf2\x5c\x50\xb5\x68\xa9\x3b\xf1\xc0\x75\x42\x0b\xbb\x7c",
            ),
            (
                127,
                b"\x92\xca\x0f\xa6\x65\x1e\xe2\xf9\x7b\x88\x4b\x72\x46\xa5\x62\xfa\
                  \x71\x25\x0f\xed\xef\xe5\xeb\xf2\x70\xd3\x1c\x54\x6b\xfe\xa9\x76",
            ),
            (
                128,
                b"\x47\x1f\xb9\x43\xaa\x23\xc5\x11\xf6\xf7\x2f\x8d\x16\x52\xd9\xc8\
                  \x80\xcf\xa3\x92\xad\x80\x50\x31\x20\x54\x77\x03\xe5\x6a\x2b\xe5",
            ),
            (
                129,
                b"\x50\x99\xc6\xa5\x62\x03\xf9\x68\x7f\x7d\x33\xf4\xbf\xdf\x57\x6d\
                  \x31\xdc\x91\xf6\xb6\x95\xec\xea\x38\xb2\x77\x0c\x87\x63\x11\x35",
            ),
            (
                1000,
                b"\x4e\x4c\x29\x4b\x33\x1f\x7a\x20\x99\xa3\x79\xbe\xc3\x4b\x9f\x9f\
                  \xc0\x3d\xc4\x6a\xb4\x65\xd9\x98\xf4\xd6\x83\xda\x53\x48\x7e\x6d",
            ),
        ];
        for &(len, expected) in cases {
            let input: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
            assert_eq!(&sha256(&input), expected, "mismatch at len {len}");
        }
    }

    /// The two samples from NIST's SHA-256 example document, a message that
    /// fits in one block and a message that spans two.
    /// https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/SHA256.pdf
    #[test]
    fn sha256_matches_nist_examples() {
        // One Block Message Sample.
        assert_eq!(
            &sha256(b"abc"),
            b"\xba\x78\x16\xbf\x8f\x01\xcf\xea\x41\x41\x40\xde\x5d\xae\x22\x23\
              \xb0\x03\x61\xa3\x96\x17\x7a\x9c\xb4\x10\xff\x61\xf2\x00\x15\xad",
        );
        // Two Block Message Sample.
        assert_eq!(
            &sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            b"\x24\x8d\x6a\x61\xd2\x06\x38\xb8\xe5\xc0\x26\x93\x0c\x3e\x60\x39\
              \xa3\x3c\xe4\x59\x64\xff\x21\x67\xf6\xec\xed\xd4\x19\xdb\x06\xc1",
        );
    }

    /// Evaluatable at compile time, which is what lets macro-generated code
    /// derive the marker from the same const-encoded spec bytes it embeds.
    #[test]
    fn generate_marker_for_xdr_is_const() {
        const M: [u8; 14] = generate_marker_for_xdr(b"abc");
        assert_eq!(&M[..6], b"SpEcV1");
        // SHA-256("abc") = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
        assert_eq!(&M[6..], b"\xba\x78\x16\xbf\x8f\x01\xcf\xea");
    }
}
