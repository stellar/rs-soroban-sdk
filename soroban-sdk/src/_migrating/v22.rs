//! # Migrating from v21 to v22
//!
//! - Add `Env` functions `register`` and `register_at` to replace `register_contract`, `register_contract_wasm`.
//!
//! - Deprecate `fuzz_catch_panic`. Use `Env` `try_invoke_contract` and the `try_` client invoke functions instead.
//!
//! - Events in test snapshots are now reduced to only contract events and system events. Diagnostic events will no longer appear in test snapshots.
