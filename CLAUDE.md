# CLAUDE.md

## Migration guides (`_migrating.rs`)

`soroban-sdk/src/_migrating.rs` (and `soroban-token-sdk/src/_migrating.rs`) is
the migration guide for major version upgrades. It documents the breaking
changes a developer needs to act on when upgrading the SDK from one major
version to the next — it is the doc a developer should go to to understand
what they need to change when upgrading.

It is **not** a changelog. Do not capture every small change here. Only
breaking changes, and changes significant enough to affect how a developer
migrates between major versions, belong in this file.
