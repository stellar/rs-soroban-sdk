### What

[TODO: Short statement about what is changing.]

### Why

[TODO: Why this change is being made. Include any context required to understand the why.]

### Known limitations

[TODO or N/A]

### SemVer Change

[TODO: Check one. Merge only when confirmed that the main branch is accepting changes for the matching type of release. See the [Cargo SemVer Compatibility reference](https://doc.rust-lang.org/cargo/reference/semver.html).]

- [ ] Major (vX._._) - Breaking change to the public API: [removing, renaming, or moving public items](https://doc.rust-lang.org/cargo/reference/semver.html#item-remove); [changing a trait's item signatures](https://doc.rust-lang.org/cargo/reference/semver.html#trait-item-signature) or [adding a non-defaulted trait item](https://doc.rust-lang.org/cargo/reference/semver.html#trait-new-item-no-default); [adding a public struct field](https://doc.rust-lang.org/cargo/reference/semver.html#struct-add-public-field-when-no-private) or [enum variant](https://doc.rust-lang.org/cargo/reference/semver.html#enum-variant-new) to a type that isn't `non_exhaustive`; [adding `non_exhaustive`](https://doc.rust-lang.org/cargo/reference/semver.html#attr-adding-non-exhaustive); [tightening generic bounds](https://doc.rust-lang.org/cargo/reference/semver.html#generic-bounds-tighten); [changing function arity](https://doc.rust-lang.org/cargo/reference/semver.html#fn-change-arity); [changing a type's layout, size, alignment, or `repr`](https://doc.rust-lang.org/cargo/reference/semver.html#type-layout); [requiring `std` where `no_std` worked](https://doc.rust-lang.org/cargo/reference/semver.html#attr-no-std-to-std); [removing a Cargo feature](https://doc.rust-lang.org/cargo/reference/semver.html#cargo-feature-remove).
- [ ] Minor (v_.Y._) - Additive change to the public API: [adding new public items](https://doc.rust-lang.org/cargo/reference/semver.html#item-new); [adding defaulted type parameters](https://doc.rust-lang.org/cargo/reference/semver.html#generic-new-default) or [defaulted trait parameters](https://doc.rust-lang.org/cargo/reference/semver.html#trait-new-parameter-default); [loosening generic bounds](https://doc.rust-lang.org/cargo/reference/semver.html#generic-bounds-loosen) or [generalizing to more generic types](https://doc.rust-lang.org/cargo/reference/semver.html#generic-more-generic); [making an `unsafe` function safe](https://doc.rust-lang.org/cargo/reference/semver.html#fn-unsafe-safe); [adding private struct fields when one already exists](https://doc.rust-lang.org/cargo/reference/semver.html#struct-private-fields-with-private); [adding a Cargo feature](https://doc.rust-lang.org/cargo/reference/semver.html#cargo-feature-add) or [dependency](https://doc.rust-lang.org/cargo/reference/semver.html#cargo-dep-add). Also use minor for the [possibly-breaking changes](https://doc.rust-lang.org/cargo/reference/semver.html#possibly-breaking-changes), such as [raising the minimum supported Rust version](https://doc.rust-lang.org/cargo/reference/semver.html#env-new-rust), [adding a defaulted trait item](https://doc.rust-lang.org/cargo/reference/semver.html#trait-new-default-item), or [adding inherent items](https://doc.rust-lang.org/cargo/reference/semver.html#impl-item-new).
- [ ] Patch (v_._.Z) - No change to the public API: bug fixes that preserve documented behaviour; performance improvements; internal refactors and private item changes; documentation, example, test, or CI changes; dependency patch bumps.
