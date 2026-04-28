//! Hidden support types for experimental spec shaking.

/// The custom section containing removable spec graph records.
#[doc(hidden)]
pub const GRAPH_SECTION: &str = "contractspecv0.rssdk.graphv0";

/// Magic bytes at the start of each removable spec graph record.
#[doc(hidden)]
pub const GRAPH_RECORD_MAGIC: [u8; 8] = *b"SpGrV0\0\0";

/// A graph record for a contract function spec entry.
#[doc(hidden)]
pub const GRAPH_RECORD_KIND_FUNCTION: u8 = 0;

/// A graph record for a contract event spec entry.
#[doc(hidden)]
pub const GRAPH_RECORD_KIND_EVENT: u8 = 1;

/// A graph record for a UDT spec entry.
#[doc(hidden)]
pub const GRAPH_RECORD_KIND_UDT: u8 = 2;

/// Implemented by generated UDTs so sidecar graph records can refer to exact type specs.
#[doc(hidden)]
pub trait SpecTypeId {
    const SPEC_TYPE_ID: [u8; 32];
}

/// A removable spec graph record emitted into [`GRAPH_SECTION`].
#[doc(hidden)]
#[repr(C)]
pub struct SpecGraphRecord<const N: usize> {
    pub magic: [u8; 8],
    pub kind: u8,
    pub reserved: [u8; 7],
    pub spec_id: [u8; 32],
    pub ref_count_le: [u8; 4],
    pub refs: [[u8; 32]; N],
}

impl<const N: usize> SpecGraphRecord<N> {
    #[doc(hidden)]
    pub const fn new(kind: u8, spec_id: [u8; 32], refs: [[u8; 32]; N]) -> Self {
        Self {
            magic: GRAPH_RECORD_MAGIC,
            kind,
            reserved: [0; 7],
            spec_id,
            ref_count_le: (N as u32).to_le_bytes(),
            refs,
        }
    }
}
