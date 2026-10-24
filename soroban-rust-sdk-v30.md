# Soroban Rust SDK v30

Version 30 of the Soroban Rust SDK supports Protocol 30,
[Bell](https://stellar.org/blog/developers/introducing-bell-protocol-30-on-stellar).
It adds the two Protocol 30 features contracts can use, and changes how
contract interfaces are generated. Most contracts upgrade without code changes.

- Build with stellar-cli v30
- Muxed contract addresses
- ML-DSA signature verification
- Unique names in contract interfaces
- Spec shaking by reference

## Build with stellar-cli v30

Contracts currently must be built with at least `stellar-cli` v25.2.0. In
soroban-sdk v30, contracts must be built with the same major version
`stellar-cli`, v30.0.0 or newer. Building with an older version of the
`stellar-cli` will result in an error.

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
    pub fn get(a: a::State, b: b::State) -> (a::State, b::State) {
        (a, b)
    }
}
```

The built spec contains `State` for `a::State` and `State2` for `b::State`:

| Rust type  | Name before v30 | Name in v30 |
| ---------- | --------------- | ----------- |
| `a::State` | `State`         | `State`     |
| `b::State` | `State`         | `State2`    |

The type whose full Rust path sorts first keeps the short name. `contractimport!`
and generated bindings handle imported specs the same way.

Contracts with no duplicate names will see no change. Contracts that have them
get a new spec after upgrading, and clients generated from it see the new names.
To keep names stable, give each type, event, and error a unique name.

## Spec shaking by reference

This change is largely an implementation detail between the soroban-sdk and the
stellar-cli's `stellar contract build` command. It is primarily of interest to
tool builders who inspect Wasm or contract spec internals, and needs no changes
to contracts.

Prior to v30 the build kept a spec entry only if a marker for it survived in the
Wasm, which meant a marker for every type, event, and error was contained in the
final Wasm. This is spec shaking v2, described in the "Spec shaking v2" section
of the [v28 post](https://stellar.org/blog/developers/soroban-rust-sdk-v28) and
in the
[v28 migration guide](https://docs.rs/soroban-sdk/28.0.0/soroban_sdk/_migrating/v28_spec_shaking/index.html).

In v30 the build keeps every entry the contract's functions refer to, directly
or through other types. Markers remain only for events and for errors passed to
`panic_with_error!`, which a function may never refer to. Contracts carry fewer
markers which is a small optimisation.

Tools that read spec markers, the `rssdk_spec_shaking` meta entry, or the
`spec_xdr` functions directly need updating. See the
[v30 migration guide](https://docs.rs/soroban-sdk/30.0.0/soroban_sdk/_migrating/index.html),
in particular
[spec markers](https://docs.rs/soroban-sdk/30.0.0/soroban_sdk/_migrating/v30_spec_markers/index.html)
and
[`spec_xdr` output](https://docs.rs/soroban-sdk/30.0.0/soroban_sdk/_migrating/v30_spec_xdr_unprocessed/index.html).

## Upgrading to v30

When upgrading to soroban-sdk v30:

- Build with `stellar contract build` from `stellar-cli` v30.0.0 or newer.
- Review contracts that read `MuxedAddress::id()`, as the address may now be a
  contract.
- Expect the spec, and generated clients, to change for contracts with duplicate
  type, event, or error names. Tooling and clients could not interact with
  those contracts correctly because of the duplicate entries, so this is as much
  a bug fix as new functionality.

This post provides a technical overview only. Developers should review the
complete migration documentation and test their contracts and integrations
before deploying with Soroban Rust SDK v30.

The full set of migration instructions, with runnable examples, is in the SDK's
Rust docs:

- https://docs.rs/soroban-sdk/30.0.0/soroban_sdk/_migrating
- https://docs.rs/soroban-sdk/30.0.0

For a full set of release notes, see the SDK's release page:

- https://github.com/stellar/rs-soroban-sdk/releases/tag/v30.0.0

For more information about the Bell upgrade, see the
[Bell upgrade guide](https://stellar.org/blog/developers/bell-protocol-30-upgrade-guide).

This version is not yet audited. For a list of audited versions of the Soroban
Rust SDK, consult:

- https://developers.stellar.org/docs/networks/audits/soroban-sdk
