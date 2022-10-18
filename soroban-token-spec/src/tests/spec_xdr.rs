use soroban_sdk::xdr::{Error, ReadXdr, ScSpecEntry};

use crate::spec_xdr;

extern crate std;

#[test]
fn test_spec_xdr() -> Result<(), Error> {
    let xdr = spec_xdr();
    let mut cursor = std::io::Cursor::new(xdr);
    for spec_entry in ScSpecEntry::read_xdr_iter(&mut cursor) {
        spec_entry?;
    }
    Ok(())
}
