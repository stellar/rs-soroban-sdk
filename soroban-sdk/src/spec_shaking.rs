//! Hidden support types for experimental spec shaking.
//!
//! Spec shaking v2 lets post-build tools remove unused entries from a contract's
//! public `contractspecv0` custom section. Rust code cannot directly tell the
//! linker which spec bytes are semantically reachable, so the SDK emits two
//! pieces of build metadata:
//!
//! - `SpEcV1` markers for extra roots that cannot be inferred from function
//!   specs, currently published events and errors thrown through
//!   `panic_with_error!` or `assert_with_error!`.
//! - Removable graph records in [`GRAPH_SECTION`] that describe exact
//!   spec-entry reachability by `SHA256(spec_entry_xdr)` ID.
//!
//! Function specs are roots because every exported `FunctionV0` entry defines
//! callable contract API. In v2, every exported function spec must also have a
//! matching [`GRAPH_RECORD_KIND_FUNCTION`] record keyed by that exact function
//! spec entry. The graph-aware post-build filter keeps the function entry
//! itself either way, but it discovers the function's parameter and return UDTs
//! only through that graph record. If the record is missing, UDTs reachable only
//! through that function can be stripped from the final spec.
//!
//! Events, errors, and UDTs also emit graph records when v2 is enabled. The
//! sidecar is private build metadata and is removed after `contractspecv0` is
//! rewritten.

/// The custom section containing removable spec graph records.
#[doc(hidden)]
pub const GRAPH_SECTION: &str = "contractspecv0.rssdk.graphv0";

/// Magic bytes at the start of each removable spec graph record.
#[doc(hidden)]
pub const GRAPH_RECORD_MAGIC: [u8; 5] = *b"SpGrV";

/// Version of the removable spec graph record format.
#[doc(hidden)]
pub const GRAPH_RECORD_VERSION: u8 = 1;

/// Length of a spec graph record header before referenced spec IDs.
#[doc(hidden)]
pub const GRAPH_RECORD_HEADER_LEN: usize = 42;

/// A graph record for a contract function spec entry.
#[doc(hidden)]
pub const GRAPH_RECORD_KIND_FUNCTION: u16 = 0;

/// A graph record for a contract event spec entry.
#[doc(hidden)]
pub const GRAPH_RECORD_KIND_EVENT: u16 = 1;

/// A graph record for a UDT spec entry.
#[doc(hidden)]
pub const GRAPH_RECORD_KIND_UDT: u16 = 2;

/// Call-site hook for error specs thrown through `panic_with_error!`.
///
/// This roots only the error's own spec entry. Types referenced by that entry
/// are retained by walking the removable spec graph.
#[doc(hidden)]
pub trait SpecShakingMarker {
    fn spec_shaking_marker();
}

impl SpecShakingMarker for crate::Error {
    #[inline(always)]
    fn spec_shaking_marker() {}
}

impl<T: SpecShakingMarker> SpecShakingMarker for &T {
    #[inline(always)]
    fn spec_shaking_marker() {
        T::spec_shaking_marker();
    }
}

/// Implemented by generated UDTs so sidecar graph records can refer to exact type specs.
#[doc(hidden)]
pub trait SpecTypeId {
    const SPEC_TYPE_ID: [u8; 32];
}

/// Serializes a removable spec graph record emitted into [`GRAPH_SECTION`].
#[doc(hidden)]
pub const fn spec_graph_record<const LEN: usize, const N: usize>(
    kind: u16,
    spec_id: [u8; 32],
    refs: [[u8; 32]; N],
) -> [u8; LEN] {
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
    while i < 32 {
        record[8 + i] = spec_id[i];
        i += 1;
    }

    let ref_count = N as u16;
    record[40] = (ref_count >> 8) as u8;
    record[41] = ref_count as u8;

    let mut ref_index = 0;
    while ref_index < N {
        let mut byte_index = 0;
        while byte_index < 32 {
            record[GRAPH_RECORD_HEADER_LEN + ref_index * 32 + byte_index] =
                refs[ref_index][byte_index];
            byte_index += 1;
        }
        ref_index += 1;
    }

    record
}
