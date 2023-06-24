use crate as soroban_sdk;

use soroban_sdk::{
    token::{StellarAssetSpec, SPEC_XDR_INPUT, SPEC_XDR_LEN},
    xdr::{Error, ReadXdr, ScSpecEntry},
};

extern crate std;

#[test]
fn test_spec_xdr_len() {
    let len = SPEC_XDR_INPUT.iter().fold(0usize, |sum, x| sum + x.len());
    assert_eq!(SPEC_XDR_LEN, len);
}

#[test]
fn test_spec_xdr() -> Result<(), Error> {
    let xdr = StellarAssetSpec::spec_xdr();
    let mut cursor = std::io::Cursor::new(xdr);
    for spec_entry in ScSpecEntry::read_xdr_iter(&mut cursor) {
        spec_entry?;
    }
    Ok(())
}
