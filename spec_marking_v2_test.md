# Spec Marking V2 Test Cases

## 1. Marker Generation (`soroban-spec/src/marker.rs`)

### 1.1 `generate_for_xdr`
- [ ] Marker is 14 bytes total (6 prefix + 8 hash)
- [ ] Marker starts with `SpEcV1` magic prefix
- [ ] Same input XDR produces same marker (deterministic)
- [ ] Different input XDR produces different marker (no collision)
- [ ] Empty XDR bytes produce a valid marker

### 1.2 `generate_for_entry`
- [ ] Generates correct marker for a FunctionV0 entry
- [ ] Generates correct marker for a UdtStructV0 entry
- [ ] Generates correct marker for a UdtEnumV0 (int enum) entry
- [ ] Generates correct marker for a UdtUnionV0 (union enum) entry
- [ ] Generates correct marker for a UdtErrorEnumV0 entry
- [ ] Generates correct marker for an EventV0 entry
- [ ] Same entry produces same marker (deterministic)
- [ ] Different entries produce different markers

### 1.3 `find_all` (WASM scanning)
- [ ] Finds markers embedded in a WASM data section
- [ ] Finds multiple markers in the same data segment
- [ ] Finds markers with arbitrary padding/data between them
- [ ] Returns empty set when no markers exist
- [ ] Does not produce false positives from random data that partially matches the magic prefix
- [ ] Handles overlapping marker-like patterns correctly
- [ ] Handles data segments shorter than 14 bytes

### 1.4 `filter`
- [ ] Functions (FunctionV0) are always kept, regardless of markers
- [ ] Structs (UdtStructV0) without markers are removed
- [ ] Structs with markers are kept
- [ ] Int enums (UdtEnumV0) without markers are removed
- [ ] Int enums with markers are kept
- [ ] Union enums (UdtUnionV0) without markers are removed
- [ ] Union enums with markers are kept
- [ ] Error enums (UdtErrorEnumV0) without markers are removed
- [ ] Error enums with markers are kept
- [ ] Events (EventV0) without markers are removed
- [ ] Events with markers are kept
- [ ] Empty marker set removes all non-function entries
- [ ] All entries with markers present keeps everything

## 2. SpecShakingMarker Trait (`soroban-sdk/src/include_spec.rs`)

### 2.1 Primitive type impls (no-op, should compile)
- [ ] `()` implements SpecShakingMarker
- [ ] `bool` implements SpecShakingMarker
- [ ] `u32`, `i32`, `u64`, `i64`, `u128`, `i128` implement SpecShakingMarker

### 2.2 Reference impls (delegation)
- [ ] `&T` delegates to `T::spec_shaking_marker()`
- [ ] `&mut T` delegates to `T::spec_shaking_marker()`

### 2.3 Container impls (propagation)
- [ ] `Option<T>` calls `T::spec_shaking_marker()`
- [ ] `Result<T, E>` calls both `T::spec_shaking_marker()` and `E::spec_shaking_marker()`
- [ ] `Vec<T>` calls `T::spec_shaking_marker()`
- [ ] `Map<K, V>` calls both `K::spec_shaking_marker()` and `V::spec_shaking_marker()`

### 2.4 Tuple impls (propagation)
- [ ] Tuples of 2-13 elements call `spec_shaking_marker()` on each element

### 2.5 SDK type impls (no-op, should compile)
- [ ] `Address`, `Bytes`, `BytesN<N>`, `String`, `Symbol` implement SpecShakingMarker
- [ ] `U256`, `I256` implement SpecShakingMarker
- [ ] Auth types (`Context`, `ContractContext`, etc.) implement SpecShakingMarker
- [ ] Crypto types (`Hash<N>`, `Bls12_381` types, `Bn254` types) implement SpecShakingMarker

## 3. Macro-Generated SpecShakingMarker Impls

### 3.1 Structs (`#[contracttype] struct`)
- [ ] Named struct generates a SpecShakingMarker impl
- [ ] Impl calls `spec_shaking_marker()` on all field types
- [ ] On WASM target: static MARKER bytes are embedded and volatile-read
- [ ] On non-WASM target: marker is not embedded (no volatile read)

### 3.2 Tuple Structs (`#[contracttype] struct Foo(...)`)
- [ ] Tuple struct generates a SpecShakingMarker impl
- [ ] Impl calls `spec_shaking_marker()` on all field types

### 3.3 Union Enums (`#[contracttype] enum` with variants)
- [ ] Union enum generates a SpecShakingMarker impl
- [ ] Impl calls `spec_shaking_marker()` on all variant field types
- [ ] Duplicate field types across variants are deduplicated (e.g., two variants with `i64` fields only call `i64::spec_shaking_marker()` once)

### 3.4 Int Enums (`#[contracttype] enum` with `= N` discriminants)
- [ ] Int enum generates a SpecShakingMarker impl
- [ ] Impl has no field type calls (int enums have no fields)

### 3.5 Error Enums (`#[contracterror] enum`)
- [ ] Error enum generates a SpecShakingMarker impl
- [ ] Impl has no field type calls (error enums have no fields)

### 3.6 Events (`#[contractevent] struct`)
- [ ] Event generates a SpecShakingMarker impl
- [ ] Impl calls `spec_shaking_marker()` on all field types (both `#[topic]` and data fields)
- [ ] Calling `publish()` on an event invokes `spec_shaking_marker()` (marking the event as used)

### 3.7 Functions (`#[contractimpl]`)
- [ ] Functions do NOT generate spec markers (they are always kept by `filter`)
- [ ] Function parameter types trigger `spec_shaking_marker()` via `TryFromValForContractFn`
- [ ] Function return types trigger `spec_shaking_marker()` via `IntoValForContractFn`

## 4. Boundary Marking via Conversion Traits

### 4.1 `TryFromValForContractFn`
- [ ] When feature enabled: calls `spec_shaking_marker()` before conversion
- [ ] When feature disabled: does not call `spec_shaking_marker()`
- [ ] Requires `T: SpecShakingMarker` bound when feature enabled

### 4.2 `IntoValForContractFn`
- [ ] When feature enabled: calls `spec_shaking_marker()` before conversion
- [ ] When feature disabled: does not call `spec_shaking_marker()`
- [ ] Requires `T: SpecShakingMarker` bound when feature enabled

## 5. Transitive Type Marking

- [ ] A struct used as a function parameter marks that struct's spec
- [ ] A struct used as a function return value marks that struct's spec
- [ ] A struct containing another struct transitively marks the inner struct
- [ ] An enum variant containing a struct transitively marks that struct
- [ ] An event published at runtime marks the event's spec AND its field types
- [ ] A `Result<T, E>` return type marks both `T` and `E`
- [ ] An `Option<T>` parameter marks `T`
- [ ] A `Vec<T>` parameter marks `T`
- [ ] A `Map<K, V>` parameter marks both `K` and `V`
- [ ] Deeply nested types (e.g., `Vec<Option<StructA>>`) mark all transitive types

## 6. Feature Gating

- [ ] All spec marking code is behind `#[cfg(feature = "experimental_spec_shaking_v2")]`
- [ ] Without the feature: no SpecShakingMarker trait exists
- [ ] Without the feature: `TryFromValForContractFn` and `IntoValForContractFn` do not require SpecShakingMarker bound
- [ ] Without the feature: macros do not generate SpecShakingMarker impls
- [ ] With the feature: everything compiles and works on native (non-WASM) targets
- [ ] With the feature: everything compiles and works on WASM targets

## 7. Build Script (`soroban-sdk/build.rs`)

- [ ] On WASM target with feature enabled: panics if `SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2` env var is not set
- [ ] On WASM target with feature enabled: succeeds if env var is set
- [ ] On non-WASM target: does not check the env var
- [ ] Without feature: does not check the env var

## 8. Contract Import (`contractimport!`)

### 8.1 Lib import (via Rust dependency)
- [ ] Imported types used at the contract boundary get markers
- [ ] Imported types NOT used at the boundary do not get markers
- [ ] Imported types generate SpecShakingMarker impls when feature enabled

### 8.2 WASM import (`contractimport!(file = "...")`)
- [ ] Types generated from WASM spec get SpecShakingMarker impls when feature enabled
- [ ] Types re-exported at the wrapping contract's boundary get markers
- [ ] Types not used at the boundary do not get markers
- [ ] `soroban-spec-rust` generates types with `export = true` when feature enabled (so markers are emitted)
- [ ] `soroban-spec-rust` generates types with `export = false` when feature disabled (backward compat)

## 9. End-to-End Spec Shaking

### 9.1 Direct contract (types defined in a lib, contract uses a subset)
- [ ] Types used as function params are kept (e.g., StructA, StructB)
- [ ] Types used as function returns are kept
- [ ] Types used only as event fields are kept when event is published
- [ ] Error types used in `Result<_, Error>` returns are kept
- [ ] Types NOT used at any boundary are stripped (e.g., StructC, EnumC, etc.)
- [ ] Functions are always kept

### 9.2 WASM-imported contract (contract wraps another via WASM import)
- [ ] Types used at the wrapping contract's boundary are kept
- [ ] Types from the imported contract that are NOT re-exported are stripped
- [ ] The wrapping contract's spec only contains the types it actually exposes

### 9.3 Type categories preserved/stripped correctly
- [ ] Named structs: used ones kept, unused stripped
- [ ] Tuple structs: used ones kept, unused stripped
- [ ] Union enums: used ones kept, unused stripped
- [ ] Int enums: used ones kept, unused stripped
- [ ] Error enums: used ones kept, unused stripped
- [ ] Events: published ones kept, unpublished stripped

## 10. Edge Cases

- [ ] A type defined but never used in any function or event is stripped
- [ ] A type used only internally (not at contract boundary) is stripped
- [ ] A type used in a private (non-`pub`) function is NOT at the boundary (should be stripped)
- [ ] Multiple contracts in the same workspace with different type usage get correct independent markers
- [ ] Hash collision resistance: two different spec entries don't produce the same 8-byte truncated hash (probabilistic, but worth testing with known entries)
- [ ] A contract with no types at all (only functions with primitives) produces no markers and all functions are kept
- [ ] A contract with the feature enabled but building natively (tests) compiles correctly with no markers in binary
- [ ] An event type that is defined but never `publish()`ed is stripped
- [ ] An error type that is defined but never returned from a function is stripped
- [ ] A struct used only as a field of another struct that IS used is kept (transitive)
- [ ] A struct used only as a field of another struct that is NOT used is stripped (transitive unused)
