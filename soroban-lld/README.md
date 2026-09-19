# soroban-lld

A linker shim that shakes unused entries out of a Soroban contract's spec as
part of linking it.

## Why this exists

A contract's spec is emitted by the SDK's macros as `#[link_section]` statics,
one per function, type and event. Those statics are retained unconditionally:
they are not subject to dead code elimination, so the spec entry for a type
reaches the wasm whether or not anything uses the type.

The macros cannot prune them either. Whether a type is used is not a property of
the crate that defines it — a type library defines many types and a contract
uses a few — so it is not knowable until every crate has been compiled and
linked together.

That makes the linker the first point in the build where the spec can be made
accurate, and this shim is the SDK's way of getting there. It runs the real
linker, then rewrites the wasm it produced.

The SDK helps by emitting a marker into the data section alongside each use of a
type or event. Unlike the spec entries, markers *are* dead code eliminated, so a
marker survives only if the thing it identifies is reachable. The shim reads the
markers that are left and keeps only the spec entries they name, plus every
function, which are the contract's API.

## Usage

Install it:

```console
cargo install soroban-lld
```

Then in the contract's `.cargo/config.toml`:

```toml
[target.wasm32v1-none]
rustflags = ["-Clinker=soroban-lld"]
```

That is the whole setup. The SDK refuses to build a contract unless the build
system shakes the spec, and it recognises this configuration by reading
`CARGO_ENCODED_RUSTFLAGS`, which cargo reports to build scripts. So the
requirement is satisfied by the configuration that carries it out, and there is
nothing to keep in sync alongside it.

Configuring the linker with `[target.<triple>] linker` instead also works, but
cargo does not report a config file's `linker` key to build scripts, so the SDK
cannot see it and the build fails the check. Use rustflags.

With that in place, `cargo build --release --target wasm32v1-none` produces a
contract whose spec names only what the contract uses — as does anything else
that drives cargo, including `cargo test --target wasm32v1-none`, a container
build, or another build system.

This is not a replacement for `stellar contract build`, which does more than
this (injecting build metadata, optimizing the wasm, and building each contract
in a workspace). Running both is fine: the CLI's own shaking pass finds nothing
left to do.

## Configuration

| Variable | Effect |
| --- | --- |
| `SOROBAN_LLD_LINKER` | The linker to run. Defaults to `rust-lld`, which rustc puts on `PATH` when it invokes a linker. |
| `SOROBAN_LLD_VERBOSE` | Report what was shaken. Off by default, because rustc surfaces linker output as a build warning. |

## What it does to the wasm

Only the `contractspecv0` custom section is rewritten. Every other section is
copied through byte for byte.

The spec is left alone unless the contract meta says the SDK emitted markers
(`rssdk_spec_shaking` is `2`). Without that there are no markers to read, and
shaking would drop every type and event. Wasm that is not a contract has no spec
section and is passed through untouched.

If the spec cannot be rewritten, the link fails rather than passing the wasm
through, so that a contract is never published with a spec claiming types it
does not have.
