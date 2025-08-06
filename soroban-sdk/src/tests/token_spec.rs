use crate as soroban_sdk;
use std::collections::HashSet;

use soroban_sdk::{
    token::{
        StellarAssetSpec, TokenSpec, STELLAR_ASSET_SPEC_XDR_INPUT, STELLAR_ASSET_SPEC_XDR_LEN,
        TOKEN_SPEC_XDR_INPUT, TOKEN_SPEC_XDR_LEN,
    },
    xdr::{Error, Limited, Limits, ReadXdr, ScSpecEntry},
};

extern crate std;
#[test]
fn test_stellar_asset_spec_xdr_len() {
    let len = STELLAR_ASSET_SPEC_XDR_INPUT
        .iter()
        .fold(0usize, |sum, x| sum + x.len());
    assert_eq!(STELLAR_ASSET_SPEC_XDR_LEN, len);
}

#[test]
fn test_token_spec_xdr_len() {
    let len = TOKEN_SPEC_XDR_INPUT
        .iter()
        .fold(0usize, |sum, x| sum + x.len());
    assert_eq!(TOKEN_SPEC_XDR_LEN, len);
}

#[test]
fn test_spec_xdr() -> Result<(), Error> {
    let xdr = StellarAssetSpec::spec_xdr();
    let cursor = std::io::Cursor::new(xdr);
    for spec_entry in ScSpecEntry::read_xdr_iter(&mut Limited::new(cursor, Limits::none())) {
        spec_entry?;
    }
    Ok(())
}

#[test]
fn test_token_spec_xdr() -> Result<(), Error> {
    let xdr = TokenSpec::spec_xdr();
    let cursor = std::io::Cursor::new(xdr);
    for spec_entry in ScSpecEntry::read_xdr_iter(&mut Limited::new(cursor, Limits::none())) {
        spec_entry?;
    }
    Ok(())
}

#[test]
fn test_stellar_asset_spec_includes_token_spec() -> Result<(), Error> {
    // Read all TokenSpec entries
    let token_xdr = TokenSpec::spec_xdr();
    let token_cursor = std::io::Cursor::new(token_xdr);
    let token_entries: HashSet<ScSpecEntry> =
        ScSpecEntry::read_xdr_iter(&mut Limited::new(token_cursor, Limits::none()))
            .collect::<Result<HashSet<_>, _>>()?;

    // Read all StellarAssetSpec entries
    let stellar_asset_xdr = StellarAssetSpec::spec_xdr();
    let stellar_asset_cursor = std::io::Cursor::new(stellar_asset_xdr);
    let stellar_asset_entries: HashSet<ScSpecEntry> =
        ScSpecEntry::read_xdr_iter(&mut Limited::new(stellar_asset_cursor, Limits::none()))
            .collect::<Result<HashSet<_>, _>>()?;
    // Check that the token entries are a subset of stellar entries
    assert!(
        token_entries.is_subset(&stellar_asset_entries),
        "StellarAssetSpec is missing entries from TokenSpec"
    );
    Ok(())
}
