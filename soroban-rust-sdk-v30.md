# Soroban Rust SDK v30

Version 30 of the Soroban Rust SDK supports Protocol 30, also called
[Bell](https://stellar.org/blog/developers/introducing-bell-protocol-30-on-stellar).
The first release candidate,
[v30.0.0-rc.1](https://github.com/stellar/rs-soroban-sdk/releases/tag/v30.0.0-rc.1),
is out. Most of the changes are in how a contract's spec is built. Those changes
alter the specs of some existing contracts and the build requirements for all
of them.

Two Protocol 30 features reach contracts through the SDK. A `MuxedAddress` can
now be a muxed contract address
([CAP-84](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0084.md)).
Contracts can verify ML-DSA post-quantum signatures
([CAP-87](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0087.md)).
On the spec side, types are named by their full Rust path while building, so
two types with the same name no longer collide. The build follows references
between spec entries to decide what to keep, and spec markers are left only
where nothing else can do the job.

## The SDK and the CLI move in lockstep

A contract's spec is produced by two tools. The SDK writes an entry for every
type, event, and error. The `stellar-cli` removes the entries the contract does
not use and, as of v30, shortens their names. A spec is only correct once the
CLI has done both. How it does them depends on the SDK version.

v28 enforced this through a feature flag, an environment variable the CLI set
to say that its build supported spec shaking. The plan was one flag for each
feature that needed the two tools to cooperate. Several more such features are
coming, and almost every one just needs a CLI at least as new as the SDK.

In v30 the SDK compares major versions. A contract using soroban-sdk v30 must be
built with `stellar contract build` from `stellar-cli` v30.0.0 or newer. More
generally, the CLI's major version has to be the same as the SDK's or greater.
Building for Wasm with an older CLI, or with any other build system, fails:

```text
error: soroban-sdk requires stellar-cli v30.0.0+ to build a contract
```

The check only applies to Wasm targets. Native builds and unit tests are
unaffected.

## Types with the same name no longer collide in the spec

Before v30 a spec named each type, event, and error by its Rust identifier and
nothing else. Two types called `State` in different modules, or in different
crates, produced two spec entries called `State`. A function that took one of
them pointed at the name `State`, and a client generator reading the spec had no
way to tell which one it meant. This used to be rare. It is getting more common
as contracts get bigger and depend on more library crates.

In v30 the SDK names each type in the spec by its full path, such as
`::my_contract::a::State`. Every reference to the type uses that path too.
`stellar contract build` then reduces each name back to its last segment. When
two names would reduce to the same one, the path that sorts first keeps the
short name. The others get a number added to the end. Every reference is
rewritten to match.

```rust
mod a {
    #[contracttype]
    pub struct State { pub count: u32 }
}

mod b {
    #[contracttype]
    pub struct State { pub enabled: bool }
}

#[contractimpl]
impl Contract {
    pub fn get(_env: Env, a: a::State, b: b::State) -> (a::State, b::State) {
        (a, b)
    }
}
```

The built spec contains a struct `State` for `a::State` and a struct `State2`
for `b::State`, and `get` refers to each by its new name. A name that would clash
with a soroban-sdk type, or with an item that client generators add such as
`Client`, is numbered the same way. `contractimport!` and the CLI's generated
bindings reduce the names of an imported spec in the same way.

The full paths exist only during the build. The spec in the built Wasm contains
short names only.

Contracts that have no duplicate names keep the same spec. Contracts that do
have them get a different spec after upgrading, because all but one of the
entries that share a name are renamed. Clients generated from that spec will
see the new names. To keep the old names, give each type, event, and error a
unique name.

## The spec is shaken by following references

v28 made spec shaking the only behaviour. The SDK put a small marker in the Wasm
data section for every type, event, and error, inside code that runs only when
that item is used at the contract boundary. The compiler removed markers that
nothing reached, and the CLI kept only the spec entries whose markers survived.
The markers were needed because the spec could not say which type a reference
pointed to. A bare name like `State` was not enough to follow.

Full paths make the references exact, so the CLI can now follow them. It keeps
every function, every type that a kept function, event, or type refers to, and
removes the rest. Only after that does it shorten the names. This is also more
precise than markers were: a type that has a marker but that nothing in the spec
names is removed.

Markers are still needed for entries that nothing else in the spec refers to:

- **Events.** No spec entry refers to an event. The marker sits inside
  `publish()`.
- **Errors used only in panics.** A contract can use an error only by passing it
  to `panic_with_error!` without any function returning it. The panic triggers
  the marker. An error returned in a function's `Result` is referred to by that
  function and needs no marker.

In the SDK's end-to-end shaking test contract, the set of entries kept is the
same as before, and the number of markers drops from 49 to 8: its 6 published
events and the 2 errors it only panics with.

## Only error types can be passed to `panic_with_error!`

`panic_with_error!` and `Env::panic_with_error` require the error type to carry
a spec marker, so that panicking with it keeps its entry in the spec. In v29
integer enums declared with `#[contracttype]` carried one too, and could be
passed to them. That support was accidental. In v30 `contracttype` types no
longer carry markers, so those calls no longer compile. Declare the type with
`#[contracterror]` instead:

```rust
#[contracterror]
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
}

panic_with_error!(&env, Error::NotFound);
```

## A muxed address can be a contract

[CAP-67](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0067.md)
added muxed account addresses, `M...`. A muxed address is an account paired with
a `u64` id, so a single account can track any number of off-chain balances.
CAP-84 does the same for contracts. A muxed contract address is a contract
address paired with a `u64` id, and its strkey starts with `W`. The Bell
announcement covers this under "One contract, many customers".

Before v30, a `MuxedAddress` with an id was always a muxed account. In v30 it can
also be a muxed contract. `MuxedAddress::address()` returns the contract address
(`C...`), and `MuxedAddress::id()` returns the id.

```rust
pub fn deposit(env: Env, to: MuxedAddress) {
    // ...
    Deposit {
        to: to.address(),     // 👈 G... or C..., whether or not `to` is muxed
        to_muxed_id: to.id(), // 👈 Some for a muxed account or muxed contract
    }
    .publish(&env);
}
```

A contract that reads `id()` and assumes the address is an account will be wrong
when it gets a `W...` address. Use `address()` for the party that holds the
balance or authorizes, whether that is an account or a contract.

In tests, `MuxedAddress::new(&contract_address, id)` now creates a muxed
contract address. Before v30 it panicked when given a contract address.

As with muxed accounts, a muxed contract address cannot be used directly as a
storage key. The host rejects it.

## ML-DSA signatures can be verified on chain

The signature schemes contracts could check before Protocol 30, ed25519,
secp256k1, and secp256r1, are all elliptic-curve schemes that a large enough
quantum computer could break. CAP-87 adds host functions that verify ML-DSA, the module-lattice
signature scheme standardized by NIST in
[FIPS 204](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf). There is
one function for each standardized parameter set. The Bell announcement covers
this under "Quantum-safe signatures for smart contracts".

The SDK exposes them on `Crypto`. They work the same way as `ed25519_verify`:
they return nothing and panic if the signature is invalid.

```rust
env.crypto().ml_dsa_44_verify(&public_key, &message, &signature, &context);
env.crypto().ml_dsa_65_verify(&public_key, &message, &signature, &context);
env.crypto().ml_dsa_87_verify(&public_key, &message, &signature, &context);
```

The public key and signature are fixed-size `BytesN`, encoded as FIPS 204
specifies:

| Function           | Public key | Signature |
| ------------------ | ---------- | --------- |
| `ml_dsa_44_verify` | 1312 bytes | 2420 bytes |
| `ml_dsa_65_verify` | 1952 bytes | 3309 bytes |
| `ml_dsa_87_verify` | 2592 bytes | 4627 bytes |

`context` is the FIPS 204 domain separation string, at most 255 bytes long.
Pass empty `Bytes` if the signer did not use one, which is the default for most
signers. Verification uses the pure, non-pre-hashed form of `ML-DSA.Verify`, so
`message` is the full message and not a hash of it. Custom accounts are the
obvious users: a `__check_auth` can now accept a post-quantum signature over the
signature payload.

## Contract types must be empty structs

The SDK never constructs the type marked with `#[contract]`. Every contract
function is an associated function that does not take `self`. Fields on that
struct used to compile and were then ignored, with only a warning when building
for Wasm. A contract with fields probably comes from a misunderstanding of how
the contract type works. Generics on the type made `#[contractimpl]` panic with
an unhelpful message.

In v30 both are compile errors. Only `pub struct Contract;` and
`pub struct Contract {}` are accepted:

```text
error: contract struct must not have fields, e.g. `pub struct Contract;`
error: contract struct must not have generics
```

Tuple structs are rejected as well, including `struct Contract();`. Contract
state belongs in storage.

## For tool builders

These changes do not affect contract code, but tools that read Wasm or put
together specs themselves need to know about them.

The `rssdk_spec_shaking` contract meta entry is no longer written. Tools should
decide how to shake a spec from the SDK version in the `rssdkver` meta entry.
For contracts built with v30 or newer, follow references between spec entries
instead of expecting a marker for each entry kept. A tool that predates v30 finds
no `rssdk_spec_shaking` entry and leaves the spec as it is, instead of removing
every type for lack of markers.

The `spec_xdr` functions that `contracttype`, `contracterror`, `contractevent`,
`contractimpl`, and `contracttrait` generate return entries as the SDK emits
them: unshaken, with full-path names, and with references to full paths. They do
not match the spec of a built contract:

```rust
let entry = ScSpecEntry::from_xdr(State::spec_xdr(), Limits::none()).unwrap();
let ScSpecEntry::UdtStructV0(s) = entry else { panic!() };
let name = s.name.to_utf8_string_lossy();
assert!(name.starts_with("::"));
assert!(name.ends_with("::State"));
```

A tool that builds a spec from these functions and publishes it should reduce
the names with `soroban_spec::reduce::reduce`, as `stellar contract build` does.
To see what is in a built contract, read the spec from the built Wasm. The
`xdr()` functions of `soroban-token-spec` and `stellar-asset-spec` now return
the reduced spec and are no longer `const`. The unprocessed spec is available
from the `const` function `xdr_unprocessed()`.

## Known issue

When a contract is built without LTO, spec entries can be missing for some
`#[contracttype]` types defined in a dependency crate: integer enums, union
enums, and tuple structs. A fix was attempted and reverted before this release
candidate. The issue is tracked in
[#2125](https://github.com/stellar/rs-soroban-sdk/issues/2125).

## Summary

- Build with `stellar contract build` from `stellar-cli` v30.0.0 or newer,
  locally and in CI.
- If the contract has duplicate type, event, or error names, expect its spec
  and generated clients to change. Rename the types to keep names stable.
- Replace any `#[contracttype]` integer enum passed to `panic_with_error!` or
  `Env::panic_with_error` with a `#[contracterror]` type.
- Where a contract reads `MuxedAddress::id()`, do not assume the address is an
  account. Use `MuxedAddress::address()` for the party that holds the balance or
  authorizes.
- Remove any fields, tuple fields, or generics from the `#[contract]` struct.
- Tools: choose the shaking approach from `rssdkver`, reduce names from
  `spec_xdr` output, and switch `const` uses of the token and asset spec `xdr()`
  functions to `xdr_unprocessed()`.

The full set of breaking changes, with runnable examples, is in the
[migration guide](https://docs.rs/soroban-sdk/30.0.0-rc.1/soroban_sdk/_migrating/index.html).
Everything else is in the
[v30.0.0-rc.1 release notes](https://github.com/stellar/rs-soroban-sdk/releases/tag/v30.0.0-rc.1).
For network-side dates and requirements, see the
[Bell upgrade guide](https://stellar.org/blog/developers/bell-protocol-30-upgrade-guide).
