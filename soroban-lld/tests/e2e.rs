//! Builds a real contract with cargo alone, with `soroban-lld` as the linker,
//! and checks that the spec in the wasm names only what the contract uses.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use stellar_xdr::{Limited, Limits, ReadXdr, ScSpecEntry};

const TARGET: &str = "wasm32v1-none";
const SHAKING_ENV: &str = "SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2";
const LINKER_ENV: &str = "CARGO_TARGET_WASM32V1_NONE_LINKER";

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/contract")
}

/// Builds the fixture contract for wasm, optionally with `soroban-lld` as the
/// linker, and returns the wasm.
fn build(with_shim: bool) -> Vec<u8> {
    let target_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(if with_shim {
        "with-shim"
    } else {
        "without-shim"
    });

    let mut cmd = Command::new(env!("CARGO"));
    cmd.current_dir(fixture())
        .args(["build", "--release", "--target", TARGET])
        .env("CARGO_TARGET_DIR", &target_dir)
        .env_remove(SHAKING_ENV)
        .env_remove(LINKER_ENV);
    if with_shim {
        // Only the linker is configured. The SDK has to work out for itself
        // that the spec will be shaken, so this asserts that detection too.
        cmd.env(LINKER_ENV, env!("CARGO_BIN_EXE_soroban-lld"));
    } else {
        // No shim, so claim the shaking the SDK demands without doing it. That
        // is the point of the comparison: it shows what the SDK emits before
        // anything has shaken it.
        cmd.env(SHAKING_ENV, "1");
    }

    let out = cmd.output().expect("running cargo");
    assert!(
        out.status.success(),
        "building the fixture contract failed:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    std::fs::read(target_dir.join(TARGET).join("release/contract.wasm")).expect("reading wasm")
}

/// Returns the names of the entries in the wasm's spec, as `kind:name`.
fn spec_names(wasm: &[u8]) -> BTreeSet<String> {
    let mut xdr = Vec::new();
    for payload in wasmparser::Parser::new(0).parse_all(wasm) {
        if let wasmparser::Payload::CustomSection(s) = payload.unwrap() {
            if s.name() == "contractspecv0" {
                xdr.extend_from_slice(s.data());
            }
        }
    }
    assert!(!xdr.is_empty(), "wasm has no contractspecv0 section");

    let mut read = Limited::new(std::io::Cursor::new(&xdr), Limits::depth(500));
    ScSpecEntry::read_xdr_iter(&mut read)
        .map(|e| match e.unwrap() {
            ScSpecEntry::FunctionV0(v) => format!("fn:{}", v.name.to_utf8_string_lossy()),
            ScSpecEntry::UdtStructV0(v) => format!("struct:{}", v.name.to_utf8_string_lossy()),
            ScSpecEntry::UdtUnionV0(v) => format!("union:{}", v.name.to_utf8_string_lossy()),
            ScSpecEntry::UdtEnumV0(v) => format!("enum:{}", v.name.to_utf8_string_lossy()),
            ScSpecEntry::UdtErrorEnumV0(v) => format!("error:{}", v.name.to_utf8_string_lossy()),
            ScSpecEntry::EventV0(v) => format!("event:{}", v.name.to_utf8_string_lossy()),
        })
        .collect()
}

#[test]
fn linking_with_the_shim_shakes_the_spec() {
    let unshaken = spec_names(&build(false));
    let shaken = spec_names(&build(true));

    // The contract's function is always kept, and so is everything it uses.
    for kept in ["fn:hello", "struct:Used", "event:Live"] {
        assert!(shaken.contains(kept), "{kept} missing from {shaken:?}");
    }

    // The type and the event the contract never touches are gone, and were
    // there before the shim ran, so the shim is what removed them.
    for dropped in ["struct:Unused", "event:Dead"] {
        assert!(
            unshaken.contains(dropped),
            "{dropped} missing from {unshaken:?}"
        );
        assert!(!shaken.contains(dropped), "{dropped} still in {shaken:?}");
    }

    assert!(shaken.is_subset(&unshaken));
    assert!(shaken.len() < unshaken.len());
}
