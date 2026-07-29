# SemVer classification rules

How to classify a change to the crates published from this repository, based on
the [Cargo SemVer Compatibility reference](https://doc.rust-lang.org/cargo/reference/semver.html).

A change is classified by the highest category any part of it falls into. If any
single change is major, the whole change is major.

## Major (vX._._) — breaking change to the public API

- [Removing, renaming, or moving public items](https://doc.rust-lang.org/cargo/reference/semver.html#item-remove).
- [Changing a trait's item signatures](https://doc.rust-lang.org/cargo/reference/semver.html#trait-item-signature), or [adding a non-defaulted trait item](https://doc.rust-lang.org/cargo/reference/semver.html#trait-new-item-no-default).
- [Adding a public struct field](https://doc.rust-lang.org/cargo/reference/semver.html#struct-add-public-field-when-no-private) or [enum variant](https://doc.rust-lang.org/cargo/reference/semver.html#enum-variant-new) to a type that isn't `non_exhaustive`.
- [Adding `non_exhaustive`](https://doc.rust-lang.org/cargo/reference/semver.html#attr-adding-non-exhaustive).
- [Tightening generic bounds](https://doc.rust-lang.org/cargo/reference/semver.html#generic-bounds-tighten).
- [Changing function arity](https://doc.rust-lang.org/cargo/reference/semver.html#fn-change-arity).
- [Changing a type's layout, size, alignment, or `repr`](https://doc.rust-lang.org/cargo/reference/semver.html#type-layout).
- [Requiring `std` where `no_std` worked](https://doc.rust-lang.org/cargo/reference/semver.html#attr-no-std-to-std).
- [Removing a Cargo feature](https://doc.rust-lang.org/cargo/reference/semver.html#cargo-feature-remove).

## Minor (v_.Y._) — additive change to the public API

- [Adding new public items](https://doc.rust-lang.org/cargo/reference/semver.html#item-new).
- [Adding defaulted type parameters](https://doc.rust-lang.org/cargo/reference/semver.html#generic-new-default) or [defaulted trait parameters](https://doc.rust-lang.org/cargo/reference/semver.html#trait-new-parameter-default).
- [Loosening generic bounds](https://doc.rust-lang.org/cargo/reference/semver.html#generic-bounds-loosen) or [generalizing to more generic types](https://doc.rust-lang.org/cargo/reference/semver.html#generic-more-generic).
- [Making an `unsafe` function safe](https://doc.rust-lang.org/cargo/reference/semver.html#fn-unsafe-safe).
- [Adding private struct fields when one already exists](https://doc.rust-lang.org/cargo/reference/semver.html#struct-private-fields-with-private).
- [Adding a Cargo feature](https://doc.rust-lang.org/cargo/reference/semver.html#cargo-feature-add) or [dependency](https://doc.rust-lang.org/cargo/reference/semver.html#cargo-dep-add).
- The [possibly-breaking changes](https://doc.rust-lang.org/cargo/reference/semver.html#possibly-breaking-changes), such as [raising the minimum supported Rust version](https://doc.rust-lang.org/cargo/reference/semver.html#env-new-rust), [adding a defaulted trait item](https://doc.rust-lang.org/cargo/reference/semver.html#trait-new-default-item), or [adding inherent items](https://doc.rust-lang.org/cargo/reference/semver.html#impl-item-new).

## Patch (v_._.Z) — no change to the public API

- Bug fixes that preserve documented behavior.
- Performance improvements.
- Internal refactors and private item changes.
- Documentation, example, test, or CI changes.
- Dependency patch bumps.

## Applying the rules to this repository

The crates in the top level directories are published and are the ones being
classified. The crates under `tests/` are test vectors and are not published.

The public API of the SDK includes the code that the macros in
`soroban-sdk-macros` generate. A change to what `#[contract]`,
`#[contractimpl]`, `#[contracttype]`, or the other macros emit can break
contracts that build against the SDK, or change the contract spec and events a
built contract exposes, even when no Rust item in the SDK itself changes.
`tests-expanded/` holds the generated code for the test vectors, so a diff there
is evidence that the generated API changed, and it is classified by what the
change does to a contract that uses it.

Changes confined to `tests/`, `.github/`, the `Makefile`, or the repository's
documentation are patch.
