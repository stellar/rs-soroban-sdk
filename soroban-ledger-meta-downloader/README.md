# soroban-ledger-meta-downloader

A Rust crate for downloading LedgerCloseMeta from SEP-54 compatible ledger storage.

## Overview

This crate provides **synchronous** functionality to download LedgerCloseMeta data from S3-compatible storage systems that follow the SEP-54 specification for storing Stellar ledger metadata. Unlike typical async S3 clients, this crate provides a 100% synchronous API.

## Features

- **100% synchronous API** - No async/await required
- Download uncompressed LedgerCloseMeta for specific ledger sequences
- Discover the latest available ledger sequence in storage
- SEP-54 compatible path structure

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
soroban-ledger-meta-downloader = "23.1.0"
```

### Basic Example

```rust
use soroban_ledger_meta_downloader::{download_ledger_close_meta, discover_latest_ledger_sequence, S3Config};
use rusoto_core::Region;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure S3 connection
    let config = S3Config {
        bucket: "stellar-ledger-data".to_string(),
        region: Region::UsEast1,
        root_path: "ledgers".to_string(),
    };

    // Discover the latest available ledger
    let latest_sequence = discover_latest_ledger_sequence(&config)?;
    println!("Latest ledger sequence: {}", latest_sequence);

    // Download metadata for a specific ledger
    let meta = download_ledger_close_meta(12345, &config)?;
    println!("Downloaded metadata for ledger {}", meta.ledger_header.sequence);

    Ok(())
}
```

## API Reference

### Structs

#### `S3Config`

Configuration for connecting to S3-compatible storage.

```rust
use rusoto_core::Region;

pub struct S3Config {
    pub bucket: String,      // S3 bucket name
    pub region: Region,      // AWS region (e.g., Region::UsEast1)
    pub root_path: String,   // Root path within the bucket
}
```

### Functions

#### `download_ledger_close_meta(sequence: u32, config: &S3Config) -> Result<LedgerCloseMeta, Error>`

Downloads LedgerCloseMeta for the specified ledger sequence.

- `sequence`: The ledger sequence number (must be > 0)
- `config`: S3 configuration
- Returns: The uncompressed LedgerCloseMeta data

#### `discover_latest_ledger_sequence(config: &S3Config) -> Result<u32, Error>`

Discovers the highest ledger sequence number available in the storage.

- `config`: S3 configuration
- Returns: The latest available ledger sequence number

### Error Types

```rust
pub enum Error {
    S3(S3Error),                    // S3 operation errors
    Io(std::io::Error),            // I/O errors
    Xdr(stellar_xdr::Error),       // XDR parsing errors
    LedgerNotFound(u32),           // Specified ledger not found
    InvalidSequence(u32),          // Invalid ledger sequence (e.g., 0)
}
```

## SEP-54 Compatibility

This crate follows the SEP-54 specification for ledger metadata storage:

- Path structure: `{root_path}/ledgers/{sequence:09d}/ledger-close-meta-{sequence:09d}.xdr`
- Uncompressed XDR format
- S3-compatible object storage

## Dependencies

- `rusoto_s3`: For S3 operations (synchronous)
- `rusoto_core`: For Rusoto core functionality
- `rusoto_credential`: For AWS credential management
- `stellar-xdr`: For XDR parsing
- `thiserror`: For error handling
- `serde`: For serialization

## License

Licensed under Apache-2.0.