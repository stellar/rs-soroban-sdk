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
//! - Removable graph records in the `contractspecv0.rssdk.graphv0` sidecar that describe exact
//!   spec-entry reachability by `SHA256(spec_entry_xdr)` ID.
//!
//! Function specs are roots because every exported `FunctionV0` entry defines
//! callable contract API. The macros emit a matching function graph record keyed
//! by that exact function spec entry. The graph-aware post-build filter keeps
//! the function entry itself as a root, but it discovers the function's
//! parameter and return UDTs only through that graph record. If a reachable
//! function references UDTs and its graph record is missing or incomplete, the
//! filter rejects the contract.
//!
//! Events, errors, and UDTs with public spec entries also emit graph records
//! when v2 is enabled. Types marked `export = false` still implement
//! [`SpecTypeId`] so other graph records can refer to them exactly, but they do
//! not export spec entries or graph records of their own. The sidecar is private
//! build metadata and is removed after `contractspecv0` is rewritten.

#[doc(hidden)]
/// Re-exported as function is referenced by generated code
pub use soroban_spec_markers::encode_graph_record;

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
///
/// This is also generated for `export = false` UDTs when spec shaking v2 is
/// enabled. Those types can be referenced by graph records without adding their
/// own entries to `contractspecv0`.
#[doc(hidden)]
pub trait SpecTypeId {
    const SPEC_TYPE_ID: [u8; 32];
}
