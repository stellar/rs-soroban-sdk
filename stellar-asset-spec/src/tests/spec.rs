extern crate std;

use crate::{XDR_INPUT, XDR_LEN};
use soroban_sdk::xdr::{
    Error, Limited, Limits, ReadXdr, ScSpecEntry, ScSpecEventParamLocationV0, ScSpecEventParamV0,
    ScSpecTypeDef, StringM,
};
use std::collections::HashSet;

#[test]
fn test_stellar_asset_spec_xdr_len() {
    let len = XDR_INPUT.iter().fold(0usize, |sum, x| sum + x.len());
    assert_eq!(XDR_LEN, len);
}

fn strip_doc(entry: &mut ScSpecEntry) {
    match entry {
        ScSpecEntry::FunctionV0(f) => {
            f.doc = StringM::default();
            for input in f.inputs.iter_mut() {
                input.doc = StringM::default();
            }
        }
        ScSpecEntry::EventV0(e) => {
            e.doc = StringM::default();
            for param in e.params.iter_mut() {
                param.doc = StringM::default();
            }
        }
        _ => {}
    }
}

fn add_sep0011_asset_topic(entry: &mut ScSpecEntry) {
    if let ScSpecEntry::EventV0(ref mut e) = entry {
        let mut params: std::vec::Vec<ScSpecEventParamV0> = e.params.iter().cloned().collect();
        // Insert sep0011_asset topic after the last existing topic-list param.
        let insert_pos = params
            .iter()
            .rposition(|p| p.location == ScSpecEventParamLocationV0::TopicList)
            .map(|i| i + 1)
            .unwrap_or(0);
        params.insert(
            insert_pos,
            ScSpecEventParamV0 {
                doc: StringM::default(),
                name: "sep0011_asset".try_into().unwrap(),
                type_: ScSpecTypeDef::String,
                location: ScSpecEventParamLocationV0::TopicList,
            },
        );
        e.params = params.try_into().unwrap();
    }
}

#[test]
fn test_stellar_asset_spec_includes_token_spec() -> Result<(), Error> {
    // Read token spec entries, strip docs, and add sep0011_asset to event topics.
    let token_xdr = soroban_token_spec::xdr();
    let token_cursor = std::io::Cursor::new(token_xdr);
    let token_entries: HashSet<ScSpecEntry> =
        ScSpecEntry::read_xdr_iter(&mut Limited::new(token_cursor, Limits::none()))
            .map(|e| {
                e.map(|mut e| {
                    strip_doc(&mut e);
                    if matches!(e, ScSpecEntry::EventV0(_)) {
                        add_sep0011_asset_topic(&mut e);
                    }
                    e
                })
            })
            .collect::<Result<HashSet<_>, _>>()?;

    // Read stellar asset spec entries and strip docs.
    let stellar_asset_xdr = crate::xdr();
    let stellar_asset_cursor = std::io::Cursor::new(stellar_asset_xdr);
    let stellar_asset_entries: HashSet<ScSpecEntry> =
        ScSpecEntry::read_xdr_iter(&mut Limited::new(stellar_asset_cursor, Limits::none()))
            .map(|e| {
                e.map(|mut e| {
                    strip_doc(&mut e);
                    e
                })
            })
            .collect::<Result<HashSet<_>, _>>()?;

    // Check that all token entries (with sep0011_asset added to events) are
    // present in the stellar asset spec (which uses SAC-specific event types
    // that already include the sep0011_asset topic).
    assert!(
        token_entries.is_subset(&stellar_asset_entries),
        "StellarAssetSpec is missing entries from TokenSpec"
    );
    Ok(())
}
