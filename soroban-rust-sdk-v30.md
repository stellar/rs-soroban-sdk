# Soroban Rust SDK v30

Version 30 of the Soroban Rust SDK supports Protocol 30,
[Bell](https://stellar.org/blog/developers/introducing-bell-protocol-30-on-stellar).
It adds the two Protocol 30 features contracts can use, and changes how
contract specs are built. Most contracts upgrade without code changes, but every
contract must be built with `stellar-cli` v30.0.0 or newer.

- Muxed contract addresses
- ML-DSA signature verification
- Unique names in contract interfaces
- Spec shaking by reference

## Muxed contract addresses

[CAP-84](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0084.md)
lets a contract address be muxed the same way an account address can: a
contract address paired with a `u64` id, with a strkey starting with `W`. A
single contract can then hold balances for many off-chain customers.

Before v30 a `MuxedAddress` with an id was always a muxed account (`M...`). In
v30 it may also be a muxed contract. `address()` returns the contract address
(`C...`) and `id()` returns the id.

```rust
pub fn deposit(env: Env, to: MuxedAddress) {
    // ...
    Deposit {
        to: to.address(),     // 👈 G... or C...
        to_muxed_id: to.id(), // 👈 Some for a muxed account or muxed contract
    }
    .publish(&env);
}
```

A contract that reads `id()` must not assume the address is an account. Use
`address()` for whoever holds the balance or authorizes.

In tests, `MuxedAddress::new(&contract_address, id)` now creates a muxed
contract address. It used to panic.

## ML-DSA signature verification

[CAP-87](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0087.md)
adds verification of ML-DSA, the post-quantum signature scheme standardized in
[FIPS 204](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf). There is
one function per parameter set. Like `ed25519_verify`, each one panics if the
signature is invalid.

```rust
env.crypto().ml_dsa_44_verify(&public_key, &message, &signature, &context);
env.crypto().ml_dsa_65_verify(&public_key, &message, &signature, &context);
env.crypto().ml_dsa_87_verify(&public_key, &message, &signature, &context);
```

| Function           | Public key | Signature  |
| ------------------ | ---------- | ---------- |
| `ml_dsa_44_verify` | 1312 bytes | 2420 bytes |
| `ml_dsa_65_verify` | 1952 bytes | 3309 bytes |
| `ml_dsa_87_verify` | 2592 bytes | 4627 bytes |

`message` is the full message, not a hash. `context` is the FIPS 204 context
string, up to 255 bytes. Pass empty `Bytes` if the signer used none, which is
the default for most signers. Custom accounts can use these in `__check_auth` to
accept post-quantum signatures.

## Unique names in contract interfaces

Before v30 the contract interface named each type, event, and error by its Rust
name alone. Two types called `State` in different modules or crates produced
two spec entries called `State`, and client generators could not tell which one
a function used.

In v30 duplicate names are given unique names, and every reference in the spec
points at the right one.

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

The built spec contains `State` for `a::State` and `State2` for `b::State`. The
type whose full Rust path sorts first keeps the short name. `contractimport!`
and generated bindings handle imported specs the same way.

Contracts with no duplicate names keep the same spec. Contracts that have them
get a new spec after upgrading, and clients generated from it see the new names.
To keep names stable, give each type, event, and error a unique name.

The renaming happens in `stellar contract build`, so the SDK now requires a
`stellar-cli` with the same major version or newer. Building for Wasm with
anything older fails:

```bash
error: soroban-sdk requires stellar-cli v30.0.0+ to build a contract
```

The check only fires for Wasm builds. Tests are unaffected and can be run with
`cargo test`.

## Spec shaking by reference

This change is largely an internal implementation detail. It is primarily of
interest to tool builders who inspect Wasm or contract spec internals, and needs
no changes to contracts.

Prior to v30 the build kept a spec entry only if a marker for it survived in the
Wasm, which meant a marker for every type, event, and error. In v30 the build
keeps every entry the contract's functions refer to, directly or through other
types. Markers remain only for events and for errors passed to
`panic_with_error!`, which a function may never refer to. Contracts carry fewer
markers. The SDK's test contract went from 49 to 8.

Tools that read spec markers, the `rssdk_spec_shaking` meta entry, or the
`spec_xdr` functions directly need updating. The migration guide covers the
details.

## Upgrading to v30

When upgrading to soroban-sdk v30:

- Build with `stellar contract build` from `stellar-cli` v30.0.0 or newer.
- Review contracts that read `MuxedAddress::id()`, as the address may now be a
  contract.
- Expect the spec, and generated clients, to change for contracts with duplicate
  type, event, or error names.

Known issue: when building without LTO, spec entries can be missing for some
`#[contracttype]` types defined in a dependency crate. It is tracked in
[#2125](https://github.com/stellar/rs-soroban-sdk/issues/2125).

This post provides a technical overview only. Developers should review the
complete migration documentation and test their contracts and integrations
before deploying with Soroban Rust SDK v30.

The full set of migration instructions, with runnable examples, is in the SDK's
Rust docs:

- https://docs.rs/soroban-sdk/30.0.0-rc.1/soroban_sdk/_migrating
- https://docs.rs/soroban-sdk/30.0.0-rc.1

For a full set of release notes, see the SDK's release page:

- https://github.com/stellar/rs-soroban-sdk/releases/tag/v30.0.0-rc.1

For network upgrade dates, see the
[Bell upgrade guide](https://stellar.org/blog/developers/bell-protocol-30-upgrade-guide).

This version is not yet audited. For a list of audited versions of the Soroban
Rust SDK, consult:

- https://developers.stellar.org/docs/networks/audits/soroban-sdk
