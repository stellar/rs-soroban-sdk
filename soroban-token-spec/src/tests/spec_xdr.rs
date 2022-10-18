use soroban_sdk::xdr::{ReadXdr, ScSpecEntry};

use crate::spec_xdr;

extern crate std;

#[test]
fn test_spec_xdr() {
    let xdr = spec_xdr();
    let mut cursor = std::io::Cursor::new(xdr);
    for spec_entry in ScSpecEntry::read_xdr_iter(&mut cursor) {
        std::println!("{:?}", spec_entry);
    }
}
