extern crate std;

use crate::{XDR_INPUT, XDR_LEN};
use soroban_sdk::xdr::{Error, Limited, Limits, ReadXdr, ScSpecEntry};

#[test]
fn test_token_spec_xdr_len() {
    let len = XDR_INPUT.iter().fold(0usize, |sum, x| sum + x.len());
    assert_eq!(XDR_LEN, len);
}

#[test]
fn test_spec_xdr_unprocessed() -> Result<(), Error> {
    let xdr = crate::xdr_unprocessed();
    let cursor = std::io::Cursor::new(xdr);
    for spec_entry in ScSpecEntry::read_xdr_iter(&mut Limited::new(cursor, Limits::none())) {
        spec_entry?;
    }
    Ok(())
}

#[test]
fn test_spec_xdr_unprocessed_names() -> Result<(), Error> {
    let xdr = crate::xdr_unprocessed();
    let cursor = std::io::Cursor::new(xdr);
    let mut events = std::vec::Vec::new();
    for spec_entry in ScSpecEntry::read_xdr_iter(&mut Limited::new(cursor, Limits::none())) {
        if let ScSpecEntry::EventV0(e) = spec_entry? {
            events.push(e.name.to_utf8_string_lossy());
        }
    }
    assert_eq!(
        events,
        [
            "::soroban_token_sdk::events::Approve",
            "::soroban_token_sdk::events::TransferWithAmountOnly",
            "::soroban_token_sdk::events::Transfer",
            "::soroban_token_spec::TransferWithMuxedString",
            "::soroban_token_spec::TransferWithMuxedBytes",
            "::soroban_token_sdk::events::Burn",
            "::soroban_token_sdk::events::MintWithAmountOnly",
            "::soroban_token_sdk::events::Mint",
            "::soroban_token_spec::MintWithMuxedString",
            "::soroban_token_spec::MintWithMuxedBytes",
            "::soroban_token_sdk::events::Clawback",
        ]
    );
    Ok(())
}

#[test]
fn test_spec_xdr_reduced_names() -> Result<(), Error> {
    let xdr = crate::xdr();
    let cursor = std::io::Cursor::new(xdr);
    let mut events = std::vec::Vec::new();
    for spec_entry in ScSpecEntry::read_xdr_iter(&mut Limited::new(cursor, Limits::none())) {
        if let ScSpecEntry::EventV0(e) = spec_entry? {
            events.push(e.name.to_utf8_string_lossy());
        }
    }
    assert_eq!(
        events,
        [
            "Approve",
            "TransferWithAmountOnly",
            "Transfer",
            "TransferWithMuxedString",
            "TransferWithMuxedBytes",
            "Burn",
            "MintWithAmountOnly",
            "Mint",
            "MintWithMuxedString",
            "MintWithMuxedBytes",
            "Clawback",
        ]
    );
    Ok(())
}
