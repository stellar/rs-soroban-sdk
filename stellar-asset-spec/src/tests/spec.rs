extern crate std;

use crate::{XDR_INPUT, XDR_LEN};
use soroban_sdk::xdr::{Error, Limited, Limits, ReadXdr, ScSpecEntry};
use std::collections::HashSet;

#[test]
fn test_stellar_asset_spec_xdr_len() {
    let len = XDR_INPUT.iter().fold(0usize, |sum, x| sum + x.len());
    assert_eq!(XDR_LEN, len);
}

#[test]
fn test_stellar_asset_spec_includes_token_spec() -> Result<(), Error> {
    // Read all token spec entries
    let token_xdr = soroban_token_spec::xdr();
    let token_cursor = std::io::Cursor::new(token_xdr);
    let token_entries: HashSet<ScSpecEntry> =
        ScSpecEntry::read_xdr_iter(&mut Limited::new(token_cursor, Limits::none()))
            .collect::<Result<HashSet<_>, _>>()?;

    // Read all StellarAssetSpec entries
    let stellar_asset_xdr = crate::xdr();
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
