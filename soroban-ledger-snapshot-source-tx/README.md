Fork tests against real Stellar network state, at the granularity of a
single transaction.

`TxSnapshotSource` is a `SnapshotSource` for the Soroban SDK's test
`Env`. Point it at a ledger, and optionally a transaction in that ledger,
and the contracts under test read the state the network had at that point.
Entries are fetched on demand as the test touches them, so there is no
footprint to work out ahead of time and no full snapshot to download.

### Experimental

**This crate is an experimental implementation and is subject to breaking
change at any time, in any release, including patch releases.** Its API,
its on-disk cache formats, and the data sources it reads from may all
change or be removed without a deprecation period or a major version bump.
It is intended for tests, not for production code.

### Usage

Add it as a dev-dependency, alongside `soroban-sdk` with the `testutils`
feature enabled:

```toml
[dev-dependencies]
soroban-ledger-snapshot-source-tx = "28"
soroban-sdk = { version = "28", features = ["testutils"] }
```

```rust,no_run
use soroban_ledger_snapshot_source_tx::{Network, TxSnapshotSource};
use soroban_sdk::{token::TokenClient, Address, Env};

// The state just before this transaction executed, in this ledger.
let tx: [u8; 32] = hex::decode("201a1e9cd0d48ed5e7facc7642785a0621f75a50c2c8fc06d1e936a82b642312")
    .unwrap()
    .try_into()
    .unwrap();
let rpc_url = "https://rpc.example.com".to_string();
let source = TxSnapshotSource::new(Network::mainnet(rpc_url), 61340000, Some(tx));
let env = Env::from_ledger_snapshot(source);

// Read that state through any contract, here the native asset contract.
let contract = Address::from_str(&env, "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA");
let client = TokenClient::new(&env, &contract);
let balance = client.balance(&Address::from_str(
    &env,
    "GCO45COWIBDZEGJ3DRGDGCCCJXK777F2S6D6HXQKXVB3EKQVCQU7B2WA",
));
```

### Point in time

The ledger sequence passed to `TxSnapshotSource::new` means the *start* of
that ledger, and the transaction hash, when given, narrows that to the point
just before that transaction executed. Passing `None` for the transaction
gives the state at the *end* of the ledger, after every transaction in it.

So to debug what a transaction did, fork at its ledger and its hash, and the
test sees exactly what that transaction saw.

### Networks

`Network` carries the URLs the entries are fetched from. There are
constructors for the public networks:

- `Network::mainnet` takes the URL of an RPC, since there is no default.
- `Network::testnet` takes the start date of the testnet epoch to read,
  because testnet is reset periodically and its meta is partitioned by
  epoch, so there is no stable default.
- `Network::local` reads from a local [stellar/quickstart] network run
  with `--enable rpc,galexie`.

The RPC must support the `ledger` parameter of `getLedgerEntries`, which
reads entries as they were at a past ledger, and must hold the ledgers
being forked.

[stellar/quickstart]: https://github.com/stellar/quickstart

Any field can be overridden, or the struct built directly, to read from
other infrastructure:

```rust
use soroban_ledger_snapshot_source_tx::Network;

// Mainnet, but reading meta from somewhere else.
let network = Network {
    meta_url: "https://meta.example.com/pubnet".to_string(),
    ..Network::mainnet("https://rpc.example.com".to_string())
};
```

### Where entries come from

With a transaction hash, each entry a test reads is looked for first in the
meta of the transactions before it in the ledger, since only the meta has
the granularity to see state part way through a ledger. An entry those
transactions did not change is as it was at the end of the previous ledger,
and is read from the RPC at that ledger.

Without a transaction hash, each entry is read from the RPC as it was at the
end of the ledger.

### Caching

Fetching is expensive, so results are cached in two places:

- `<workspace-root>/tests-snapshot-source/<network>/`, holding the ledger
  entries the tests in this workspace have read. **Commit this directory.**
  With it present the tests replay from disk, so CI reproduces the run
  without reaching the network at all.
- The system cache directory (`~/.cache/soroban-sdk/snapshot-source-tx/` on
  Linux), holding both the found entries and the raw files downloaded to
  find them. It is shared across workspaces and is safe to delete.

The workspace root is located with `cargo metadata`, so tests must be run
from within a cargo workspace.

Cached entries record the stellar-xdr schema they were written with, and a
test fails rather than reading entries written against a different schema.
If that happens, delete `tests-snapshot-source` and re-run to regenerate it.

### Logging

Set `RUST_LOG` to see which source each entry was found in, which is the
quickest way to tell why a fork is slow or returning state you did not
expect:

```sh
RUST_LOG=soroban_ledger_snapshot_source_tx=debug cargo test -- --nocapture
```
