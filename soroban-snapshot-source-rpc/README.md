# soroban-snapshot-source-rpc

A Soroban snapshot source that fetches ledger entries from a stellar-rpc server.

## Usage

```rust
use soroban_snapshot_source_rpc::RpcSnapshotSource;
use soroban_sdk::Env;

// Create an RPC snapshot source
let source = RpcSnapshotSource::new("http://localhost:8000/rpc".to_string());

// Create an Env with the RPC snapshot source
let env = Env::from_ledger_snapshot(source);
```

## Features

- Implements the `SnapshotSource` trait for fetching ledger entries
- Uses synchronous HTTP requests to stellar-rpc's `getLedgerEntries` method
