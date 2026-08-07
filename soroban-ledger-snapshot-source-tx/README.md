# soroban-ledger-snapshot-source-tx

A `SnapshotSource` implementation for the Soroban SDK that fetches ledger entries by downloading and searching ledger meta from multiple sources: SEP-54 meta storage, RPC, and history archives.

The optional `hubble` feature contains a small BigQuery REST prototype for
contract-data lookups. It is not enabled by default, so the existing
history-archive path remains the fallback.

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

## Hubble prototype

Enable the native client with `features = ["hubble"]`. Hubble's documented public bronze dataset is `crypto-stellar.crypto_stellar`;
transformed/current tables are in the sibling
`crypto-stellar.crypto_stellar_dbt` dataset. Access requires a Google Cloud
project with billing, the BigQuery API enabled, and an OAuth access token.
The client uses parameterized SQL and bounds each lookup to one row:

```rust
use soroban_ledger_snapshot_source_tx::{
    HubbleClient, HubbleConfig, Network, TxSnapshotSource,
};

let hubble = HubbleClient::new(HubbleConfig::mainnet(access_token));
let source = TxSnapshotSource::new_with_hubble(Network::mainnet(None), 59_914_751, None, hubble);
```

The query is limited to Hubble's `contract_data` state table, using its
`ledger_key_hash`, `ledger_sequence`, `deleted`, and `contract_data_xdr` fields.
This prototype does not query Hubble's separate `history_transactions.tx_meta`
XDR, so it does not claim to reconstruct the exact state immediately before a
transaction when several transactions touch the same key in one ledger.
Therefore the Hubble path is never used when a transaction hash is supplied.
Hubble misses, unsupported key types, and transaction-before requests continue
to the history-archive fallback. Hubble's transaction metadata may support a
future exact implementation, but it must decode and apply the XDR changes
without guessing transaction ordering.

Hubble is documented as a public mainnet dataset with no network discriminator.
Testnet reset epochs are therefore rejected by the prototype rather than
querying a potentially unrelated epoch. Hubble is batch-updated and has no
completeness watermark in this lookup, so a miss also falls back to the
history archive.

Research references:

* [Hubble overview](https://developers.stellar.org/docs/data/analytics/hubble)
* [Connecting to Hubble](https://developers.stellar.org/docs/data/analytics/hubble/analyst-guide/connecting)
* [Viewing metadata](https://developers.stellar.org/docs/data/analytics/hubble/analyst-guide/viewing-metadata)
* [History vs state tables](https://developers.stellar.org/docs/data/analytics/hubble/analyst-guide/history-vs-state-tables)
* [Contract data schema](https://developers.stellar.org/docs/data/analytics/hubble/data-catalog/data-dictionary/bronze/contract-data)

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
