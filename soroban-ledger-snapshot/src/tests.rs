use soroban_env_host::{
    storage::SnapshotSource,
    xdr::{LedgerKey, Limits, ReadXdr},
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
