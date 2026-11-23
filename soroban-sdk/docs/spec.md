# Contract Spec Generation Internals

This document describes how the macros build the contract spec using const evaluation. It is intended for SDK developers who need to understand or modify the macro implementation.

## Overview

Every contract embeds its interface, the contract spec, in a custom section of its Wasm named `contractspecv0`. The spec is a sequence of XDR encoded `ScSpecEntry` values, one for each function, user-defined type, error enum and event. Tools such as the stellar-cli, the JavaScript SDK, and block explorers read the section to learn how to call the contract's functions, encode the arguments they take, and decode the values they return, the errors they fail with, and the events the contract publishes. The spec only describes the contract's public interface: it doesn't describe the data a contract keeps in storage, except where that data uses types that also appear in the interface.

The macros that generate spec entries are `#[contractimpl]` (via `#[contractspecfn]`) for functions, `#[contracttype]` for structs and enums, `#[contracterror]` for error enums, and `#[contractevent]` for events.

The macros don't build or encode spec entries themselves. They emit const expressions that describe each entry, referring to constants on the types involved, and the compiler evaluates them during the build: first the entry, then its XDR encoding. The compiler evaluates consts after it has resolved names and types, which macros run too early to see, so the entries are built from what each type actually is rather than from the name it was written with.

## How it works

Given this event:

```rust
use soroban_sdk::Address;
type Destination = Address;

#[contractevent]
pub struct Transfer {
    #[topic] from: Address,
    #[topic] to: Destination,
    amount: i128,
}
```

`#[contractevent]` generates, simplified:

```rust
impl Transfer {
    const __SPEC_XDR_ENTRY: ScSpecEntry = ScSpecEntry::EventV0(ScSpecEventV0 {
        name: StringM::try_from_str_or_panic(<Transfer as SpecName>::SPEC_NAME), // "::token::Transfer"
        params: VecM::try_from_slice_or_panic(&[
            ScSpecEventParamV0 { name: ..."from", type_: <Address as SpecTypeDef>::SPEC_TYPE_DEF, ... },
            ScSpecEventParamV0 { name: ..."to", type_: <Destination as SpecTypeDef>::SPEC_TYPE_DEF, ... },
            ScSpecEventParamV0 { name: ..."amount", type_: <i128 as SpecTypeDef>::SPEC_TYPE_DEF, ... },
        ]),
        ...
    });

    pub const fn spec_xdr() -> [u8; Transfer::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Transfer::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}

#[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
static __SPEC_XDR_EVENT_TRANSFER: [u8; Transfer::spec_xdr().len()] = Transfer::spec_xdr();
```

The entry is a const built from the `r#const` XDR types. `spec_xdr()` encodes it into a byte array whose length is itself computed at compile time, and the `static` places those bytes in the `contractspecv0` section. Everything above is evaluated at compile time. None of it runs on chain.

The values in the entry come from three places: the spec type of each Rust type, from `SpecTypeDef`; the name of each user-defined type, from `SpecName`; and literals the macro can see directly, such as field names and docs.

### `SpecTypeDef`: the spec type of a Rust type

**Source:** `soroban-sdk/src/spec_type_def.rs`

```rust
pub trait SpecTypeDef {
    const SPEC_TYPE_DEF: ScSpecTypeDef;
}
```

Every type that can appear in a spec implements `SpecTypeDef`. The SDK implements it for the built-in types (`Address => ScSpecTypeDef::Address`, `u32 => ScSpecTypeDef::U32`, ...) and generically for containers, whose spec type is built from their parameters' constants (`Option<T>`, `Result<T, E>`, `Vec<T>`, `Map<K, V>`, `BytesN<N>`, tuples, and references). User-defined types get an impl from their macro (see below).

The macros render every type in a spec entry as `<T as SpecTypeDef>::SPEC_TYPE_DEF` (see `const_view_type_def` in `soroban-sdk-macros/src/spec.rs`). The compiler resolves `T` before looking up the constant, so a type alias or a type imported under another name gets the spec type of the type it refers to. In the example, `Destination` gets `Address`'s spec type.

`SPEC_TYPE_DEF` is an associated const rather than a fn so that it can be used to build the const spec entry. The trait has a `#[diagnostic::on_unimplemented]` message so that using a type that isn't supported in a contract spec gives a readable error.

### `SpecName`: the fully qualified name of a user-defined type

```rust
pub trait SpecName {
    const SPEC_NAME: &'static str;
}
```

`#[contracttype]`, `#[contracterror]` and `#[contractevent]` implement `SpecName` and `SpecTypeDef` for the type they are on (see `spec_type_def_gen` in `soroban-sdk-macros/src/spec.rs`):

```rust
impl SpecName for Transfer {
    const SPEC_NAME: &'static str = concat!("::", module_path!(), "::", "Transfer");
}
impl SpecTypeDef for Transfer {
    const SPEC_TYPE_DEF: ScSpecTypeDef = ScSpecTypeDef::Udt(ScSpecTypeUdt {
        name: StringM::try_from_str_or_panic(<Self as SpecName>::SPEC_NAME),
    });
}
```

The name is the type's full Rust path, so that same-named types in different modules or crates have distinct names in the spec. The macro emits `module_path!()` next to the type and the compiler expands it to the module the type is defined in. The name is rooted with a leading `::` so it is an absolute path. Every spec entry that refers to the type, in the same crate or another, does so through the type's `SPEC_TYPE_DEF`, and so uses the same name.

The spec limits the length of type names. The length is only known once `module_path!()` has expanded, so the generated `SPEC_NAME` contains a const `assert!` that fails the build with an error pointing at the type if its full name is too long.

### Const XDR types

**Source:** the `const` module of `stellar-xdr`, re-exported as `soroban_sdk::xdr::r#const`

Spec entries are built from the `r#const` XDR types rather than the standard ones, because the standard types own heap data (`Vec`, `String`, `Box`) and can't be built in const context. The `r#const` module has a type for every XDR type: types that own heap data have a borrowing equivalent holding `&'static` data, and every other type is an alias to the standard type. Every type in the module has:

- `const_xdr_len()`: the length of the type's XDR encoding, used as the length of the array the encoding is written into.
- `const_to_xdr()`: the XDR encoding as a fixed-size byte array.

Length-limited types have const constructors, such as `StringM::try_from_str_or_panic` and `VecM::try_from_slice_or_panic`, that check the limit and fail the build if it is exceeded.

The macros render literal values, such as docs and names, as calls to these constructors with helpers such as `const_view_string` and `const_view_symbol` in `soroban-sdk-macros/src/spec.rs`.

## Things to know

- **Errors happen at compile time.** A panic while evaluating a const, such as a name that is too long, fails the build. Prefer checks that point at the user's code, as the `SpecName` length check does, over letting a length check deep inside the XDR encoding fail.
- **Outer references are stripped.** `const_view_type_def` removes outer `&` from a type before rendering it, because the lifetimes may be declared by a function signature and the spec entry is not rendered in that function's scope.
- **Only Wasm builds get the section.** Spec entries are generated for native builds too, but the `link_section` attribute is only applied on Wasm targets.

## Source Files

| File | Purpose |
|------|---------|
| `soroban-sdk/src/spec_type_def.rs` | `SpecTypeDef` and `SpecName` traits and SDK type impls |
| `soroban-sdk-macros/src/spec.rs` | Renders types and values as const expressions, and generates `SpecName`/`SpecTypeDef` impls for user-defined types |
| `soroban-sdk-macros/src/derive_spec_fn.rs` | Function spec entries |
| `soroban-sdk-macros/src/derive_struct.rs`, `derive_struct_tuple.rs`, `derive_enum.rs`, `derive_enum_int.rs` | Type spec entries |
| `soroban-sdk-macros/src/derive_error_enum_int.rs` | Error enum spec entries |
| `soroban-sdk-macros/src/derive_event.rs` | Event spec entries |
