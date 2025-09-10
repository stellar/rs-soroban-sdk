extern crate std;

use crate::{XDR_INPUT, XDR_LEN};
use soroban_sdk::xdr::{Error, Limited, Limits, ReadXdr, ScSpecEntry};

#[test]
fn test_token_spec_xdr_len() {
    let len = XDR_INPUT.iter().fold(0usize, |sum, x| sum + x.len());
    assert_eq!(XDR_LEN, len);
}

#[test]
fn test_spec_xdr() -> Result<(), Error> {
    let xdr = crate::xdr();
    let cursor = std::io::Cursor::new(xdr);
    for spec_entry in ScSpecEntry::read_xdr_iter(&mut Limited::new(cursor, Limits::none())) {
        spec_entry?;
    }
    Ok(())
}
