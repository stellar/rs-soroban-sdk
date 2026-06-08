# rs-soroban-sdk
Rust SDK for writing contracts for [Soroban].

Soroban: https://soroban.stellar.org

Docs: https://docs.rs/soroban-sdk

[Soroban]: https://soroban.stellar.org

## Support

The two most recent soroban-sdk major releases are supported with critical security fixes.
Critical security issues may be backported to earlier versions if practical, but not guaranteed.
General bugs are only fixed on, and new features are only added to, the latest major release.

## Build Target

Contracts must be built for the `wasm32v1-none` target, available with Rust 1.84+. It is the only
wasm target supported by the Soroban runtime on Stellar.

The `wasm32-unknown-unknown` target is not supported when building with Rust 1.82 or newer, because
on those versions the target enables wasm features (reference-types, multi-value) that the Soroban
environment does not support and that cannot be easily disabled. Building for
`wasm32-unknown-unknown` on Rust 1.82+ produces a build error.

Build contracts with `stellar contract build` (stellar-cli), or with
`cargo build --target wasm32v1-none`.

## Contributing

Contributing to the SDK? Read [CONTRIBUTING.md](CONTRIBUTING.md).
