//! Spec markers are only included for events and for errors triggered by panics.
//!
//! This change is largely an internal detail, included for visibility for tool builders who may be
//! inspecting Wasm and contract spec internal data.
//!
//! In v26 and v27 under a feature flag, and in v28+ the SDK embedded a marker in the Wasm data
//! section for every type, event, and error used by the contract (see [v28_spec_shaking]), and the
//! stellar-cli kept the spec entries that had a marker and removed the others. The SDK recorded
//! this in the `rssdk_spec_shaking` contract meta entry with the value `2`.
//!
//! In v30 the SDK names every type in the spec by its full Rust path (see [v30_duplicate_names]),
//! so a reference to a type in the spec identifies it exactly. The stellar-cli uses this to shake
//! the spec by following references: it keeps every function, every type a kept function, event,
//! or type refers to, and removes the rest. Only the entries that nothing in the spec refers to
//! need markers:
//!
//! - **Events**, as no spec entry refers to an event. The marker is triggered inside the
//!   `publish()` call.
//! - **Errors triggered by panics**, as a contract can use an error solely by passing it to
//!   [`panic_with_error!`] or [`Env::panic_with_error`] without any function returning it. The
//!   marker is triggered by the panic. An error returned in a function's `Result` is referred to by
//!   the function, and so needs no marker.
//!
//! As a result, contracts carry fewer markers in their Wasm.
//!
//! The SDK no longer records the `rssdk_spec_shaking` meta entry. Tools choose how to shake a
//! contract's spec from the SDK version recorded in the `rssdkver` meta entry: a contract built
//! with soroban-sdk v30 or newer is shaken by following references. A tool that predates v30 finds
//! no `rssdk_spec_shaking` entry and leaves the spec unchanged, rather than removing every type
//! from it for lack of markers.
//!
//! ## Migrating
//!
//! No code changes are required for contracts. Tools that inspect spec markers in the Wasm, or the
//! `rssdk_spec_shaking` meta entry, need to read the `rssdkver` meta entry and, for contracts built
//! with soroban-sdk v30 or newer, follow references between spec entries instead of expecting a
//! marker for every entry kept.
//!
//! [v28_spec_shaking]: super::v28_spec_shaking
//! [v30_duplicate_names]: super::v30_duplicate_names
//! [`panic_with_error!`]: crate::panic_with_error
//! [`Env::panic_with_error`]: crate::Env::panic_with_error
