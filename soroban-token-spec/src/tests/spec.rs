extern crate std;

use crate::XDR_INPUT;
use soroban_sdk::xdr::{Error, Limited, Limits, ReadXdr, ScSpecEntry};
use std::vec::Vec;

#[test]
fn test_token_spec_xdr_is_reduced() -> Result<(), Error> {
    // The macros name events by their fully qualified path, and the published
    // spec is the same entries with those names reduced to simple names, as
    // `stellar contract build` reduces them.
    let read = |xdr: &[u8]| {
        ScSpecEntry::read_xdr_iter(&mut Limited::new(std::io::Cursor::new(xdr), Limits::none()))
            .collect::<Result<Vec<_>, _>>()
    };
    let raw = read(&XDR_INPUT.concat())?;
    let reduced = read(crate::xdr())?;
    assert_eq!(raw.len(), reduced.len());
    let mut events = 0;
    for (raw, reduced) in raw.iter().zip(&reduced) {
        match (raw, reduced) {
            (ScSpecEntry::EventV0(raw), ScSpecEntry::EventV0(reduced)) => {
                let raw_name: &[u8] = raw.name.as_ref();
                let reduced_name: &[u8] = reduced.name.as_ref();
                assert!(raw_name.windows(2).any(|w| w == b"::"));
                assert_eq!(reduced_name, soroban_spec::reduce::last_segment(raw_name));
                events += 1;
            }
            (raw, reduced) => assert_eq!(raw, reduced),
        }
    }
    assert!(events > 0);
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
