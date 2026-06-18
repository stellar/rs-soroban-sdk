# CLAUDE.md

## Migration guide (`_migrating.rs`)

`soroban-sdk/src/_migrating.rs` (and `soroban-token-sdk/src/_migrating.rs`)
documents how to migrate across major versions — the breaking changes a
developer needs to act on when upgrading the SDK from one major version to the
next. It is the doc a developer should go to to understand what they need to
change when upgrading.

It is **not** a changelog: not every small change belongs here. Only capture
breaking changes and the steps required to migrate; leave incremental,
non-breaking changes out.
