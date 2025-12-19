# soroban-ledger-snapshot-source-tx

A `SnapshotSource` implementation for the Soroban SDK that fetches ledger entries by downloading and searching ledger meta from multiple sources: SEP-54 meta storage, RPC, and history archives.

## Usage

Add the dependency:

```toml
[dependencies]
soroban-ledger-snapshot-source-tx = "23"
```

Use it in tests:

```rust
use bytes_lit::bytes;
use soroban_ledger_fetch::Network;
use soroban_ledger_snapshot_source_tx::TxSnapshotSource;
use soroban_sdk::Env;

let tx_hash = bytes!(0x6fc2e483896276816b6d3b8d1df778bc978521f51561faa407ab8bb1949e6a1b);

// Use mainnet with default URLs
let source = TxSnapshotSource::new(
    Network::mainnet(),
    59914751,  // Ledger sequence
    Some(tx_hash),
);

let env = Env::from_ledger_snapshot(source);
```

Or with custom network configuration:

```rust
use bytes_lit::bytes;
use soroban_ledger_fetch::Network;
use soroban_ledger_snapshot_source_tx::TxSnapshotSource;
use soroban_sdk::Env;

let tx_hash = bytes!(0x6fc2e483896276816b6d3b8d1df778bc978521f51561faa407ab8bb1949e6a1b);

let network = Network {
    passphrase: "Public Global Stellar Network ; September 2015".to_string(),
    meta_url: "https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/pubnet".to_string(),
    rpc_url: "https://mainnet.sorobanrpc.com".to_string(),
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
