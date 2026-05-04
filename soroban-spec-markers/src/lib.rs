//! Wire-format helpers for Soroban spec-shaking metadata.
//!
//! This crate owns the byte formats emitted by the Rust SDK and consumed by
//! post-build tooling:
//!
//! - `contractmetav0` keys that opt a contract into spec shaking v2.
//! - `SpEcV1` root markers embedded in reachable contract code.
//! - `SpGrV` graph records emitted into the removable
//!   `contractspecv0.rssdk.graphv0` sidecar custom section.
//!
//! It does not implement reachability analysis, Wasm scanning, or spec
//! filtering. Those live in `soroban-spec`.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "hash")]
use sha2::{Digest, Sha256};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// The contract meta key that indicates the spec shaking version.
pub const META_KEY: &str = "rssdk_spec_shaking";

/// The meta value for spec shaking version 2.
pub const META_VALUE_V2: &str = "2";

/// Length of a full SHA256-based spec ID.
pub const SPEC_ID_LEN: usize = 32;

/// A stable identity for a spec entry.
pub type SpecId = [u8; SPEC_ID_LEN];

/// Magic bytes that identify a root spec marker: `SpEcV1`.
pub const MARKER_MAGIC: [u8; 6] = *b"SpEcV1";

/// Number of spec ID bytes stored after [`MARKER_MAGIC`].
pub const MARKER_HASH_LEN: usize = 8;

/// Total length of a spec marker.
pub const MARKER_LEN: usize = 14;

/// A spec marker that identifies a spec entry by a truncated spec ID.
pub type Marker = [u8; MARKER_LEN];

/// Generates a stable identity for spec entry XDR bytes.
#[cfg(feature = "hash")]
pub fn generate_spec_id_for_xdr(spec_entry_xdr: &[u8]) -> SpecId {
    Sha256::digest(spec_entry_xdr).into()
}

/// Generates a spec marker for spec entry XDR bytes.
#[cfg(feature = "hash")]
pub fn generate_marker_for_xdr(spec_entry_xdr: &[u8]) -> Marker {
    let hash = generate_spec_id_for_xdr(spec_entry_xdr);
    [
        MARKER_MAGIC[0],
        MARKER_MAGIC[1],
        MARKER_MAGIC[2],
        MARKER_MAGIC[3],
        MARKER_MAGIC[4],
        MARKER_MAGIC[5],
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

/// A decoded root marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodedMarker {
    /// First 8 bytes of `SHA256(spec_entry_xdr)`.
    pub spec_id_prefix: [u8; MARKER_HASH_LEN],
}

/// Error returned when decoding a root marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeMarkerError {
    TruncatedMarker,
    InvalidMagic,
}

/// Decodes a root marker from the start of `data`.
pub fn decode_marker(data: &[u8]) -> Result<DecodedMarker, DecodeMarkerError> {
    if data.len() < MARKER_LEN {
        return Err(DecodeMarkerError::TruncatedMarker);
    }

    if !data.starts_with(&MARKER_MAGIC) {
        return Err(DecodeMarkerError::InvalidMagic);
    }

    let mut spec_id_prefix = [0; MARKER_HASH_LEN];
    spec_id_prefix.copy_from_slice(&data[MARKER_MAGIC.len()..MARKER_LEN]);
    Ok(DecodedMarker { spec_id_prefix })
}

/// The custom section containing removable spec graph records.
pub const GRAPH_SECTION: &str = "contractspecv0.rssdk.graphv0";

/// Magic bytes at the start of each removable spec graph record.
pub const GRAPH_RECORD_MAGIC: [u8; 5] = *b"SpGrV";

/// Version of the removable spec graph record format.
pub const GRAPH_RECORD_VERSION: u8 = 1;

/// Length of a spec graph record header before referenced spec IDs.
pub const GRAPH_RECORD_HEADER_LEN: usize = 42;

/// Graph record kind for a contract function spec entry.
pub const GRAPH_RECORD_KIND_FUNCTION: u16 = 0;

/// Graph record kind for a contract event spec entry.
pub const GRAPH_RECORD_KIND_EVENT: u16 = 1;

/// Graph record kind for a UDT spec entry.
pub const GRAPH_RECORD_KIND_UDT: u16 = 2;

/// The kind of spec entry represented by a spec graph record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpecGraphEntryKind {
    Function,
    Event,
    Udt,
}

impl SpecGraphEntryKind {
    /// Converts a graph record kind value into an enum variant.
    pub const fn from_u16(value: u16) -> Option<Self> {
        match value {
            GRAPH_RECORD_KIND_FUNCTION => Some(Self::Function),
            GRAPH_RECORD_KIND_EVENT => Some(Self::Event),
            GRAPH_RECORD_KIND_UDT => Some(Self::Udt),
            _ => None,
        }
    }

    /// Converts an enum variant into a graph record kind value.
    pub const fn to_u16(self) -> u16 {
        match self {
            Self::Function => GRAPH_RECORD_KIND_FUNCTION,
            Self::Event => GRAPH_RECORD_KIND_EVENT,
            Self::Udt => GRAPH_RECORD_KIND_UDT,
        }
    }
}

/// Returns the encoded byte length for a spec graph record with `ref_count` references.
pub const fn graph_record_len(ref_count: usize) -> usize {
    GRAPH_RECORD_HEADER_LEN + ref_count * SPEC_ID_LEN
}

/// Generates an encoded removable spec graph record in const contexts.
///
/// This is a `const fn` rather than a fully baked byte literal because SDK macros emit graph
/// records before every referenced UDT's exact spec ID is known. When `derive_*` expands for type
/// `Foo`, it knows `Foo`'s own spec XDR and can hash it, but referenced UDTs such as `Bar` or
/// `Baz` may be defined elsewhere, possibly in another crate. The macro therefore emits each ref
/// as a trait-associated constant expression like `<Bar as SpecTypeId>::SPEC_TYPE_ID` and lets
/// const-eval resolve the final 32-byte IDs after all impls are in scope.
pub const fn encode_graph_record<const LEN: usize, const N: usize>(
    kind: u16,
    spec_id: SpecId,
    refs: [SpecId; N],
) -> [u8; LEN] {
    assert!(N <= u16::MAX as usize);
    assert!(LEN == graph_record_len(N));

    let mut record = [0; LEN];

    record[0] = GRAPH_RECORD_MAGIC[0];
    record[1] = GRAPH_RECORD_MAGIC[1];
    record[2] = GRAPH_RECORD_MAGIC[2];
    record[3] = GRAPH_RECORD_MAGIC[3];
    record[4] = GRAPH_RECORD_MAGIC[4];
    record[5] = GRAPH_RECORD_VERSION;
    record[6] = (kind >> 8) as u8;
    record[7] = kind as u8;

    let mut i = 0;
    while i < SPEC_ID_LEN {
        record[8 + i] = spec_id[i];
        i += 1;
    }

    let ref_count = N as u16;
    record[40] = (ref_count >> 8) as u8;
    record[41] = ref_count as u8;

    let mut ref_index = 0;
    while ref_index < N {
        let mut byte_index = 0;
        while byte_index < SPEC_ID_LEN {
            record[GRAPH_RECORD_HEADER_LEN + ref_index * SPEC_ID_LEN + byte_index] =
                refs[ref_index][byte_index];
            byte_index += 1;
        }
        ref_index += 1;
    }

    record
}

/// Generates an encoded removable spec graph record.
///
/// # Panics
///
/// Panics if `refs` contains more than `u16::MAX` entries.
#[cfg(feature = "alloc")]
pub fn generate_graph_record(
    kind: SpecGraphEntryKind,
    spec_id: SpecId,
    refs: &[SpecId],
) -> Vec<u8> {
    assert!(
        refs.len() <= u16::MAX as usize,
        "spec graph record cannot encode more than u16::MAX refs"
    );

    let mut bytes = Vec::with_capacity(graph_record_len(refs.len()));
    bytes.extend_from_slice(&GRAPH_RECORD_MAGIC);
    bytes.push(GRAPH_RECORD_VERSION);
    bytes.extend_from_slice(&kind.to_u16().to_be_bytes());
    bytes.extend_from_slice(&spec_id);
    bytes.extend_from_slice(&(refs.len() as u16).to_be_bytes());
    for ref_id in refs {
        bytes.extend_from_slice(ref_id);
    }
    bytes
}

/// A decoded removable spec graph record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodedGraphRecord<'a> {
    pub kind: SpecGraphEntryKind,
    pub spec_id: SpecId,
    refs: &'a [u8],
    encoded_len: usize,
}

impl<'a> DecodedGraphRecord<'a> {
    /// Returns the byte length consumed by this record.
    pub const fn encoded_len(&self) -> usize {
        self.encoded_len
    }

    /// Returns the number of referenced spec IDs in this record.
    pub const fn ref_count(&self) -> usize {
        self.refs.len() / SPEC_ID_LEN
    }

    /// Iterates over referenced spec IDs.
    pub const fn refs(&self) -> GraphRecordRefs<'a> {
        GraphRecordRefs { bytes: self.refs }
    }
}

/// Iterator over graph record references.
#[derive(Clone, Debug)]
pub struct GraphRecordRefs<'a> {
    bytes: &'a [u8],
}

impl Iterator for GraphRecordRefs<'_> {
    type Item = SpecId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.len() < SPEC_ID_LEN {
            return None;
        }

        let (id_bytes, rest) = self.bytes.split_at(SPEC_ID_LEN);
        self.bytes = rest;

        let mut id = [0; SPEC_ID_LEN];
        id.copy_from_slice(id_bytes);
        Some(id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for GraphRecordRefs<'_> {
    fn len(&self) -> usize {
        self.bytes.len() / SPEC_ID_LEN
    }
}

/// Error returned when decoding a graph record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeGraphRecordError {
    TruncatedHeader,
    InvalidMagic,
    UnsupportedVersion { version: u8 },
    InvalidKind { kind: u16 },
    TruncatedRefs,
}

/// Decodes a removable spec graph record from the start of `data`.
pub fn decode_graph_record(data: &[u8]) -> Result<DecodedGraphRecord<'_>, DecodeGraphRecordError> {
    if data.len() < GRAPH_RECORD_HEADER_LEN {
        return Err(DecodeGraphRecordError::TruncatedHeader);
    }

    if !data.starts_with(&GRAPH_RECORD_MAGIC) {
        return Err(DecodeGraphRecordError::InvalidMagic);
    }

    if data[5] != GRAPH_RECORD_VERSION {
        return Err(DecodeGraphRecordError::UnsupportedVersion { version: data[5] });
    }

    let kind = u16::from_be_bytes([data[6], data[7]]);
    let Some(kind) = SpecGraphEntryKind::from_u16(kind) else {
        return Err(DecodeGraphRecordError::InvalidKind { kind });
    };

    let mut spec_id = [0; SPEC_ID_LEN];
    spec_id.copy_from_slice(&data[8..40]);

    let ref_count = u16::from_be_bytes([data[40], data[41]]) as usize;
    let Some(refs_len) = ref_count.checked_mul(SPEC_ID_LEN) else {
        return Err(DecodeGraphRecordError::TruncatedRefs);
    };
    let Some(record_len) = GRAPH_RECORD_HEADER_LEN.checked_add(refs_len) else {
        return Err(DecodeGraphRecordError::TruncatedRefs);
    };
    if data.len() < record_len {
        return Err(DecodeGraphRecordError::TruncatedRefs);
    }

    Ok(DecodedGraphRecord {
        kind,
        spec_id,
        refs: &data[GRAPH_RECORD_HEADER_LEN..record_len],
        encoded_len: record_len,
    })
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "hash")]
    #[test]
    fn marker_generation_has_stable_bytes() {
        let marker = generate_marker_for_xdr(b"hello world");
        assert_eq!(marker, *b"SpEcV1\xb9\x4d\x27\xb9\x93\x4d\x3e\x08");
    }

    #[test]
    fn marker_decode_reads_prefix() {
        let marker = *b"SpEcV1\x01\x02\x03\x04\x05\x06\x07\x08";
        let decoded = decode_marker(&marker).unwrap();

        assert_eq!(decoded.spec_id_prefix, marker[6..14]);
    }

    #[test]
    fn marker_decode_rejects_truncated_marker() {
        assert_eq!(
            decode_marker(b"SpEcV1\x01\x02\x03").unwrap_err(),
            DecodeMarkerError::TruncatedMarker
        );
    }

    #[test]
    fn marker_decode_rejects_invalid_magic() {
        let marker = *b"NoEcV1\x01\x02\x03\x04\x05\x06\x07\x08";

        assert_eq!(
            decode_marker(&marker).unwrap_err(),
            DecodeMarkerError::InvalidMagic
        );
    }

    #[test]
    fn graph_record_kind_rejects_out_of_range_value() {
        assert_eq!(SpecGraphEntryKind::from_u16(3), None);
    }

    #[test]
    fn graph_record_encode_has_stable_bytes() {
        const LEN: usize = graph_record_len(2);
        let spec_id = [1u8; SPEC_ID_LEN];
        let refs = [[2u8; SPEC_ID_LEN], [3u8; SPEC_ID_LEN]];
        let record =
            encode_graph_record::<LEN, 2>(SpecGraphEntryKind::Event.to_u16(), spec_id, refs);

        assert_eq!(record.len(), graph_record_len(refs.len()));
        assert_eq!(&record[0..5], GRAPH_RECORD_MAGIC.as_slice());
        assert_eq!(record[5], GRAPH_RECORD_VERSION);
        assert_eq!(
            u16::from_be_bytes([record[6], record[7]]),
            GRAPH_RECORD_KIND_EVENT
        );
        assert_eq!(&record[8..40], spec_id.as_slice());
        assert_eq!(
            u16::from_be_bytes([record[40], record[41]]),
            refs.len() as u16
        );
        assert_eq!(&record[42..74], refs[0].as_slice());
        assert_eq!(&record[74..106], refs[1].as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn graph_record_const_encoder_matches_vec_encoder() {
        const LEN: usize = graph_record_len(2);
        let spec_id = [4u8; SPEC_ID_LEN];
        let refs = [[5u8; SPEC_ID_LEN], [6u8; SPEC_ID_LEN]];

        let const_record =
            encode_graph_record::<LEN, 2>(SpecGraphEntryKind::Udt.to_u16(), spec_id, refs);
        let vec_record = generate_graph_record(SpecGraphEntryKind::Udt, spec_id, &refs);

        assert_eq!(const_record.as_slice(), vec_record.as_slice());
    }

    #[test]
    fn graph_record_decode_roundtrips() {
        const LEN: usize = graph_record_len(2);
        let spec_id = [7u8; SPEC_ID_LEN];
        let refs = [[8u8; SPEC_ID_LEN], [9u8; SPEC_ID_LEN]];
        let record =
            encode_graph_record::<LEN, 2>(SpecGraphEntryKind::Function.to_u16(), spec_id, refs);

        let decoded = decode_graph_record(&record).unwrap();

        assert_eq!(decoded.kind, SpecGraphEntryKind::Function);
        assert_eq!(decoded.spec_id, spec_id);
        assert_eq!(decoded.encoded_len(), record.len());
        let mut decoded_refs = decoded.refs();
        assert_eq!(decoded_refs.next(), Some(refs[0]));
        assert_eq!(decoded_refs.next(), Some(refs[1]));
        assert_eq!(decoded_refs.next(), None);
    }

    #[test]
    fn graph_record_decode_rejects_truncated_header() {
        assert_eq!(
            decode_graph_record(&[0u8; GRAPH_RECORD_HEADER_LEN - 1]).unwrap_err(),
            DecodeGraphRecordError::TruncatedHeader
        );
    }

    #[test]
    fn graph_record_decode_rejects_invalid_magic() {
        let record = [0u8; GRAPH_RECORD_HEADER_LEN];

        assert_eq!(
            decode_graph_record(&record).unwrap_err(),
            DecodeGraphRecordError::InvalidMagic
        );
    }

    #[test]
    fn graph_record_decode_rejects_unsupported_version() {
        const LEN: usize = graph_record_len(0);
        let mut record = encode_graph_record::<LEN, 0>(
            SpecGraphEntryKind::Function.to_u16(),
            [1u8; SPEC_ID_LEN],
            [],
        );
        record[5] = GRAPH_RECORD_VERSION + 1;

        assert_eq!(
            decode_graph_record(&record).unwrap_err(),
            DecodeGraphRecordError::UnsupportedVersion { version: record[5] }
        );
    }

    #[test]
    fn graph_record_decode_rejects_invalid_kind() {
        const LEN: usize = graph_record_len(0);
        let mut record = encode_graph_record::<LEN, 0>(
            SpecGraphEntryKind::Function.to_u16(),
            [1u8; SPEC_ID_LEN],
            [],
        );
        record[6] = 0xff;
        record[7] = 0xff;

        assert_eq!(
            decode_graph_record(&record).unwrap_err(),
            DecodeGraphRecordError::InvalidKind { kind: u16::MAX }
        );
    }

    #[test]
    fn graph_record_decode_rejects_truncated_refs() {
        const LEN: usize = graph_record_len(1);
        let record = encode_graph_record::<LEN, 1>(
            SpecGraphEntryKind::Function.to_u16(),
            [1u8; SPEC_ID_LEN],
            [[2u8; SPEC_ID_LEN]],
        );

        assert_eq!(
            decode_graph_record(&record[..record.len() - 1]).unwrap_err(),
            DecodeGraphRecordError::TruncatedRefs
        );
    }
}
