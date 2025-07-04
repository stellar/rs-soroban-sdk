use crate as soroban_sdk;
use soroban_sdk::contractmeta;
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{Limits, ReadXdr, ScMetaEntry, ScMetaV0};

#[test]
fn test_meta_key_val() {
    contractmeta!(key = "mykey", val = "myval");

    let entry = ScMetaEntry::from_xdr(__CONTRACT_KEY_6d796b6579, Limits::none()).unwrap();
    let expect = ScMetaEntry::ScMetaV0(ScMetaV0 {
        key: "mykey".try_into().unwrap(),
        val: "myval".try_into().unwrap(),
    });

    assert_eq!(entry, expect);
}

#[test]
fn test_meta_env_macro_support() {
    contractmeta!(key = "binver", val = env!("CARGO_PKG_VERSION"));

    let entry = ScMetaEntry::from_xdr(__CONTRACT_KEY_62696e766572, Limits::none()).unwrap();
    let expect = ScMetaEntry::ScMetaV0(ScMetaV0 {
        key: "binver".try_into().unwrap(),
        val: env!("CARGO_PKG_VERSION").try_into().unwrap(),
    });

    assert_eq!(entry, expect);
}
