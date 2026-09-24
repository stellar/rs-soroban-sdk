//! Prints the contract spec in a wasm as a pretty formatted JSON array of
//! XDR-JSON values, one per spec entry.
//!
//! Entries are sorted by kind and name, then by the entry itself, rather than
//! printed in the order they appear in the wasm, so that moving an item around
//! in the source, which changes the order its entry is written to the wasm,
//! does not change the output.
//!
//! ```console
//! cargo run --package spec-json -- contract.wasm
//! ```

use std::{cmp::Ordering, env, fs, process::exit};

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
    entries.sort_by(compare);
    println!("{}", serde_json::to_string_pretty(&entries).unwrap());
}

/// Compares entries by kind and name, then by the entry itself.
fn compare(a: &ScSpecEntry, b: &ScSpecEntry) -> Ordering {
    (a.discriminant(), name(a), a).cmp(&(b.discriminant(), name(b), b))
}

/// Returns the name of the entry.
fn name(entry: &ScSpecEntry) -> &[u8] {
    match entry {
        ScSpecEntry::FunctionV0(e) => e.name.0.as_ref(),
        ScSpecEntry::UdtStructV0(e) => e.name.as_ref(),
        ScSpecEntry::UdtUnionV0(e) => e.name.as_ref(),
        ScSpecEntry::UdtEnumV0(e) => e.name.as_ref(),
        ScSpecEntry::UdtErrorEnumV0(e) => e.name.as_ref(),
        ScSpecEntry::EventV0(e) => e.name.as_ref(),
    }
}
