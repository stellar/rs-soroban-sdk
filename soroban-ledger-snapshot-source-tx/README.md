# soroban-ledger-snapshot-source-tx

A `SnapshotSource` implementation for the Soroban SDK that fetches ledger entries by downloading and searching ledger meta from multiple sources: SEP-54 meta storage, RPC, and history archives.

## Usage

Add the dependency:

```toml
[dependencies]
soroban-ledger-snapshot-source-tx = "27"
```

Use it in tests:

```rust
use bytes_lit::bytes;
use soroban_ledger_snapshot_source_tx::{Network, TxSnapshotSource};
use soroban_sdk::Env;

let tx_hash = bytes!(0x6fc2e483896276816b6d3b8d1df778bc978521f51561faa407ab8bb1949e6a1b);

// Use mainnet with default URLs
let source = TxSnapshotSource::new(
    Network::mainnet(None),
    59914751,  // Ledger sequence
    Some(tx_hash),
);

let env = Env::from_ledger_snapshot(source);
```

Or with custom network configuration:

```rust
use bytes_lit::bytes;
use soroban_ledger_snapshot_source_tx::{Network, TxSnapshotSource};
use soroban_sdk::Env;

let tx_hash = bytes!(0x6fc2e483896276816b6d3b8d1df778bc978521f51561faa407ab8bb1949e6a1b);

let network = Network {
    name: "mainnet".to_string(),
    passphrase: "Public Global Stellar Network ; September 2015".to_string(),
    meta_url: "https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/pubnet".to_string(),
    rpc_url: Some("https://mainnet.sorobanrpc.com".to_string()),
    archive_url: "https://history.stellar.org/prd/core-live/core_live_001".to_string(),
    archive_checkpoint_ledger_count: 64,
};

let source = TxSnapshotSource::new(
    network,
    59914751,  // Ledger sequence
    Some(tx_hash),
);

let env = Env::from_ledger_snapshot(source);
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
