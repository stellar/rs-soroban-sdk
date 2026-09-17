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
let source = TxSnapshotSource::new(Network::mainnet(None), 61340000, Some(tx));
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

- `Network::mainnet` takes an optional RPC URL.
- `Network::testnet` takes the start date of the testnet epoch to read,
  because testnet is reset periodically and its meta is partitioned by
  epoch, so there is no stable default.
- `Network::local` reads from a local [stellar/quickstart] network run
  with `--enable rpc,galexie`.

[stellar/quickstart]: https://github.com/stellar/quickstart

Any field can be overridden, or the struct built directly, to read from
other infrastructure:

```rust
use soroban_ledger_snapshot_source_tx::Network;

// Mainnet, but reading meta from somewhere else and never asking an RPC.
let network = Network {
    meta_url: "https://meta.example.com/pubnet".to_string(),
    rpc_url: None,
    ..Network::mainnet(None)
};
```

### Where entries come from

For each entry a test reads, the sources are tried in the order below, and
the first that answers wins. Each is slower than the one before it, so the
ordering is what keeps a fork fast:

1. The meta of the transactions already looked at in the queried ledger.
2. The meta of the transactions before it in the same ledger.
3. The RPC, if `rpc_url` is set, and only when what it returns is known to
   be correct for the queried ledger.
4. The meta of preceding ledgers, back as far as the last checkpoint.
5. The history archive checkpoint covering the ledger.

An RPC is an optimisation rather than a requirement: it answers in one
request what would otherwise be a walk back through meta and archives. With
`rpc_url` set to `None` every entry is resolved from meta and archives
alone.

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

## Hubble (BigQuery) checkpoint source

By default, resolving an entry that was not touched in the ledgers being
replayed falls back to downloading and linearly scanning the history archive's
bucket files at the enclosing checkpoint. That is authoritative but slow, and
the bucket set can be many gigabytes.

The optional `hubble` feature adds a faster path for that final step, backed by
[Hubble], the Stellar Development Foundation's public BigQuery dataset. Hubble
supports random access by ledger key, so a checkpoint lookup becomes a single
point query instead of a bulk download.

[Hubble]: https://developers.stellar.org/docs/data/analytics/hubble

```toml
[dependencies]
soroban-ledger-snapshot-source-tx = { version = "27", features = ["hubble"] }
```

```rust
use soroban_ledger_snapshot_source_tx::{
    HubbleConfig, HubbleSource, Network, TxSnapshotSource,
};

// Query compute is billed to your own Google Cloud project.
let hubble = HubbleSource::new(HubbleConfig::mainnet("my-gcp-project"));

let source = TxSnapshotSource::new(Network::mainnet(None), 59914751, Some(tx_hash))
    .with_hubble(hubble);
```

### Authentication

This crate never reads, stores, or logs credentials, and performs no implicit
credential discovery. You supply an OAuth 2.0 access token with the
`https://www.googleapis.com/auth/bigquery.readonly` scope, either through the
environment (the default) or explicitly:

```console
export GOOGLE_OAUTH_ACCESS_TOKEN="$(gcloud auth print-access-token)"
```

To source tokens some other way — a service account, a workload identity, a
refresh loop — implement `AccessTokenSource` and pass it to
`HubbleSource::with_token`. `HubbleSource::with_transport` likewise replaces the
HTTP transport, which is how the test suite exercises every query and response
path without network access.

### Cost controls

Hubble's storage is paid for by the SDF, but **every query is billed to your
project**. This crate therefore:

- sends `maximumBytesBilled` on every request, so a query that would scan more
  than the configured cap (8 GiB by default) *fails* instead of running. There
  is no way to express "unlimited";
- always filters by an exact ledger key and an upper ledger bound, orders by
  `ledger_sequence`, and applies `LIMIT 1`;
- passes every varying value as a named query parameter, never interpolating it
  into SQL. Project, dataset, and table names cannot be parameterized by
  BigQuery, so they are validated against a strict identifier allowlist first;
- optionally accepts `closed_at_from` / `closed_at_to` bounds. Hubble's state
  tables are partitioned by month on `closed_at`, so a lower bound is what
  actually prunes partitions. Setting `closed_at_from` also *weakens* the
  answer: a missing row may simply predate the window, so a miss is reported as
  "cannot answer" and the history archive is consulted instead;
- resolves Hubble's ingestion high-water mark once per table read, with a query
  bounded to the last `coverage_lookback_days` (30 by default) so it prunes to
  a couple of partitions.

### Limitations

Enabling this feature can only change how fast an answer arrives, never whether
the answer is correct: anything Hubble cannot answer authoritatively — including
any Hubble error — falls back to the history archive.

- **Only contract data and TTL entries are served.** Hubble stores decoded,
  flattened columns for most entry types and raw XDR for almost none. A
  `LedgerEntry` can only be rebuilt exactly where the full contents survive:
  `contract_data` carries a [`contract_data_xdr`] column holding the
  `ContractDataEntry` XDR, and a `TtlEntry` is exactly the `key_hash` and
  `live_until_ledger_seq` columns of the [`ttl`] table.
- **Contract code is not served.** The [`contract_code`] table records only
  static analysis metrics (`n_instructions`, `n_functions`, …); the Wasm bytes
  are [deliberately omitted][contract-code-src], so the entry cannot be rebuilt.
  Configuring an RPC URL on the `Network` remains the fast path for Wasm.
- **Classic entry types are not served.** `accounts`, `trust_lines`, `offers`,
  `liquidity_pools`, and `claimable_balances` have no XDR column, so any
  reconstruction would be lossy.
- **Nonce entries are absent.** stellar-etl [discards][nonce-src] contract data
  keyed by `ScValTypeScvLedgerKeyNonce` before loading, so Hubble never holds a
  row for one. Nonce keys are therefore refused outright rather than being
  reported as unused.
- **Ledger granularity only.** State tables carry `ledger_sequence` but no
  transaction index, so Hubble alone cannot express "state as of just before
  transaction T in ledger N". This is not a limitation here: transaction-granular
  state comes from replaying ledger-close meta, and Hubble is consulted only for
  the checkpoint fallback, which is ledger-granular by construction.
- **Mainnet only.** The public dataset publishes pubnet data, and there is no
  network discriminator column. There is no public testnet or futurenet Hubble
  dataset, so testnet resets have no representation. Point `dataset_project_id`
  and `dataset` at your own dataset to use this on another network.
- **Not real-time.** Hubble is loaded in intraday batches, so it lags the
  network. Every lookup is gated on the table being read having ingested at or
  past the checkpoint being resolved; anything newer falls back to the archive.
  Coverage is measured on that same table rather than on `history_ledgers`,
  because Hubble's history and state tables are loaded by independent
  pipelines. Without this gate a not-yet-loaded entry would read as "never
  existed", and an entry changed after the high-water mark would read back
  stale.
- **`LedgerEntry` extension fields are not preserved.** Hubble does not store
  them, so `ext` is set to `V0` — the same behaviour as this crate's existing RPC
  source, and immaterial for contract data and TTL entries, which cannot be
  sponsored.

[`contract_data_xdr`]: https://developers.stellar.org/docs/data/analytics/hubble/data-catalog/data-dictionary/bronze/contract-data
[`ttl`]: https://developers.stellar.org/docs/data/analytics/hubble/data-catalog/data-dictionary/bronze/ttl
[`contract_code`]: https://developers.stellar.org/docs/data/analytics/hubble/data-catalog/data-dictionary/bronze/contract-code
[contract-code-src]: https://github.com/stellar/stellar-etl/blob/master/internal/transform/schema.go
[nonce-src]: https://github.com/stellar/stellar-etl/blob/master/internal/transform/contract_data.go
