//! Building contracts requires the stellar-cli.
//!
//! ## Breaking Change: Wasm Builds Fail Without stellar-cli
//!
//! Building a contract for a wasm target without using the stellar-cli now
//! results in a compile error. The soroban-sdk relies on the stellar-cli's
//! `stellar contract build` to perform post-build steps such as spec
//! optimization. Builds that bypass the stellar-cli produce incomplete or
//! incorrect contract artifacts.
//!
//! ## Upgrade
//!
//! Install [stellar-cli] v25.2.0 or newer, and build contracts with:
//!
//! ```sh
//! stellar contract build
//! ```
//!
//! No further changes to the contract crate or its `Cargo.toml` are required.
//!
//! [stellar-cli]: https://github.com/stellar/stellar-cli
//!
//! ## Temporary Escape Hatch
//!
//! For environments that cannot yet use the stellar-cli, the error can be
//! suppressed by setting:
//!
//! ```sh
//! SOROBAN_SDK_ALLOW_BUILD_WITHOUT_STELLAR_CLI=1
//! ```
//!
//! This escape hatch is provided for the transition and will be removed in a
//! near future version of soroban-sdk once support for building without the
//! stellar-cli is completely removed. Contracts built with the escape hatch may
//! be missing post-build steps and should not be deployed to mainnet.
//!
//! ## Non-Wasm Builds
//!
//! The check only applies when building for a wasm target. Host builds (for
//! example, running tests with `cargo test`) are unaffected.
