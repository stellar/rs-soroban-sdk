//! Prints the contract spec in a wasm as a stream of XDR-JSON values.
//!
//! Each spec entry is printed as one pretty formatted JSON value after the
//! other, so that a diff of the output points at the entry that changed.
//!
//! Entries are sorted, rather than printed in the order they appear in the
//! wasm, so that moving an item around in the source, which changes the order
//! its entry is written to the wasm, does not change the output.
//!
//! ```console
//! cargo run --package spec-json -- contract.wasm
//! ```

use std::{env, fs, process::exit};

use stellar_xdr::ScSpecEntry;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: spec-json <wasm>");
        exit(2);
    };
    let wasm = fs::read(&path).unwrap_or_else(|e| {
        eprintln!("error: reading {path}: {e}");
        exit(1);
    });
    let mut entries: Vec<ScSpecEntry> = soroban_spec::read::from_wasm(&wasm).unwrap_or_else(|e| {
        eprintln!("error: reading spec from {path}: {e}");
        exit(1);
    });
    entries.sort();
    for entry in entries {
        println!("{}", serde_json::to_string_pretty(&entry).unwrap());
    }
}
