use soroban_env_host::{
    storage::SnapshotSource,
    xdr::{LedgerKey, Limits, ReadXdr},
    LedgerInfo,
};

use crate::LedgerSnapshot;
use std::rc::Rc;

// Tuple of (ledger key in xdr base64, live_until_ledger_seq u32) for the entries in the test snapshots
const TEST_SNAPSHOT_XDR: [(&str, u32); 4] = [
    ("AAAABgAAAAEltPzYWa7C+mNIQ4xImzw8EMmLbSG+T9PLMMtolT75dwAAABQAAAAB", 10000000),
    ("AAAABgAAAAEltPzYWa7C+mNIQ4xImzw8EMmLbSG+T9PLMMtolT75dwAAABAAAAABAAAAAgAAAA8AAAAHQmFsYW5jZQAAAAASAAAAAa3vzlmu5Slo92Bh1JTCUlt1ZZ+kKWpl9JnvKeVkd+SWAAAAAQ==", 10000001),
    ("AAAACAAAAAA=", 10000002),
    ("AAAAAAAAAAA7mRE4Dv6Yi6CokA6xz+RPNm99vpRr7QdyQPf2JN8VxQ==", 10000003)
];

#[test]
fn test_snapshot_from_v1() {
    let snapshot = LedgerSnapshot::read_file("./test_data/snapshot_v1.json").unwrap();

    for (xdr_key, live_until_ledger_seq) in TEST_SNAPSHOT_XDR.iter() {
        let ledger_key = LedgerKey::from_xdr_base64(xdr_key, Limits::none()).unwrap();
        let entry = snapshot.get(&Rc::new(ledger_key)).unwrap();
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().1, Some(live_until_ledger_seq.clone()));
    }

    let expected_str = std::fs::read_to_string("./test_data/snapshot_v2.json").unwrap();
    let mut written = Vec::new();
    snapshot.write(&mut written).unwrap();
    let written_str = String::from_utf8(written).unwrap();

    let expected_normalized = expected_str.replace("\r\n", "\n");
    let written_normalized = written_str.replace("\r\n", "\n");
    assert_eq!(written_normalized, expected_normalized);
}

#[test]
fn test_snapshot_roundtrip() {
    let snapshot = LedgerSnapshot::read_file("./test_data/snapshot_v2.json").unwrap();

    for (xdr_key, live_until_ledger_seq) in TEST_SNAPSHOT_XDR.iter() {
        let ledger_key = LedgerKey::from_xdr_base64(xdr_key, Limits::none()).unwrap();
        let entry = snapshot.get(&Rc::new(ledger_key)).unwrap();
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().1, Some(live_until_ledger_seq.clone()));
    }

    let expected_str = std::fs::read_to_string("./test_data/snapshot_v2.json").unwrap();
    let mut written = Vec::new();
    snapshot.write(&mut written).unwrap();
    let written_str = String::from_utf8(written).unwrap();

    let expected_normalized = expected_str.replace("\r\n", "\n");
    let written_normalized = written_str.replace("\r\n", "\n");
    assert_eq!(written_normalized, expected_normalized);
}

#[test]
fn test_ledger_info() {
    let mut snapshot = LedgerSnapshot::read_file("./test_data/snapshot_v2.json").unwrap();
    let info = LedgerInfo {
        protocol_version: 25,
        sequence_number: 1234567,
        timestamp: 1000000000,
        network_id: [1u8; 32],
        base_reserve: 123,
        min_persistent_entry_ttl: 42,
        min_temp_entry_ttl: 42,
        max_entry_ttl: 99999,
    };
    snapshot.set_ledger_info(info.clone());
    let got = snapshot.ledger_info();
    assert_eq!(got.protocol_version, info.protocol_version);
    assert_eq!(got.sequence_number, info.sequence_number);
    assert_eq!(got.timestamp, info.timestamp);
    assert_eq!(got.network_id, info.network_id);
    assert_eq!(got.base_reserve, info.base_reserve);
    assert_eq!(got.min_persistent_entry_ttl, info.min_persistent_entry_ttl);
    assert_eq!(got.min_temp_entry_ttl, info.min_temp_entry_ttl);
    assert_eq!(got.max_entry_ttl, info.max_entry_ttl);
}

#[test]
fn test_set_and_update_entries() {
    let base = LedgerSnapshot::read_file("./test_data/snapshot_v2.json").unwrap();
    let entries: Vec<_> = base.entries().into_iter().collect();

    // Set entries to only the first two entries
    let mut snapshot = LedgerSnapshot::default();
    snapshot.set_entries(entries[..2].iter().map(|(k, v)| (*k, (&v.0, v.1))));
    assert_eq!(snapshot.count_entries(), 2);

    // Upsert: update first entry and add third entry
    let updates: Vec<(
        Rc<LedgerKey>,
        Option<(Rc<soroban_env_host::xdr::LedgerEntry>, Option<u32>)>,
    )> = vec![
        // Update existing with new live_until
        (
            Rc::new((**entries[0].0).clone()),
            Some((Rc::new((*entries[0].1 .0).clone()), Some(99))),
        ),
        // Add new entry
        (
            Rc::new((**entries[2].0).clone()),
            Some((Rc::new((*entries[2].1 .0).clone()), entries[2].1 .1)),
        ),
    ];
    snapshot.update_entries(&updates);
    assert_eq!(snapshot.count_entries(), 3);

    let key0 = Rc::new((**entries[0].0).clone());
    let key2 = Rc::new((**entries[2].0).clone());
    assert_eq!(snapshot.get(&key0).unwrap().unwrap().1, Some(99));
    assert!(snapshot.get(&key2).unwrap().is_some());
}

#[test]
fn test_update_remove_then_serialize() {
    let mut snapshot = LedgerSnapshot::read_file("./test_data/snapshot_v2.json").unwrap();
    let ledger_key = LedgerKey::from_xdr_base64(TEST_SNAPSHOT_XDR[3].0, Limits::none()).unwrap();
    let ledger_key_rc = Rc::new(ledger_key.clone());

    let entry = snapshot.get(&ledger_key_rc).unwrap();
    assert!(entry.is_some());
    assert_eq!(snapshot.count_entries(), 4);

    // Remove via update_entries with None value
    let updates: Vec<(
        Rc<LedgerKey>,
        Option<(Rc<soroban_env_host::xdr::LedgerEntry>, Option<u32>)>,
    )> = vec![(ledger_key_rc.clone(), None)];
    snapshot.update_entries(&updates);

    let entry = snapshot.get(&ledger_key_rc).unwrap();
    assert!(entry.is_none());
    assert_eq!(snapshot.count_entries(), 3);

    // Write and read back — tombstone should not appear
    let mut buf = Vec::new();
    snapshot.write(&mut buf).unwrap();
    let restored = LedgerSnapshot::read(&buf[..]).unwrap();

    let entry = restored.get(&ledger_key_rc).unwrap();
    assert!(entry.is_none());
    assert_eq!(restored.count_entries(), 3);
    assert_eq!(snapshot, restored);
}
