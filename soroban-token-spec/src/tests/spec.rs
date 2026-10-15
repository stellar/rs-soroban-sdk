extern crate std;

use crate::XDR_INPUT;
use soroban_sdk::xdr::{Error, Limited, Limits, ReadXdr, ScSpecEntry};
use std::vec::Vec;

#[test]
fn test_token_spec_xdr_is_reduced() -> Result<(), Error> {
    // The spec is the entries the macros emit, with their names reduced as
    // `stellar contract build` reduces them.
    let raw = XDR_INPUT.concat();
    let entries =
        ScSpecEntry::read_xdr_iter(&mut Limited::new(std::io::Cursor::new(raw), Limits::none()))
            .collect::<Result<Vec<_>, _>>()?;
    let reduced = soroban_spec::reduce::reduce(&entries).unwrap();
    assert!(reduced.renames().any(|r| r.renamed()));
    let expected: Vec<ScSpecEntry> = reduced.into_entries().collect();
    let actual = ScSpecEntry::read_xdr_iter(&mut Limited::new(
        std::io::Cursor::new(crate::xdr()),
        Limits::none(),
    ))
    .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(actual, expected);
    Ok(())
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
