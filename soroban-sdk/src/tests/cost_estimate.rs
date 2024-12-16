use crate as soroban_sdk;
use expect_test::expect;
use soroban_sdk::Env;
use soroban_sdk_macros::symbol_short;

mod contract_data {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(file = "test_wasms/test_contract_data.wasm");
}

// Update the test data in this test via running it with `UPDATE_EXPECT=1`.
#[test]
fn test_cost_estimate_with_storage() {
    let e = Env::default();
    e.cost_estimate().enable();

    let contract_id = e.register(contract_data::WASM, ());
    let client = contract_data::Client::new(&e, &contract_id);

    // Write a single new entry to the storage.
    client.put(&symbol_short!("k1"), &symbol_short!("v1"));
    expect![[r#"
        InvocationResources {
            instructions: 455853,
            mem_bytes: 1162241,
            read_entries: 2,
            write_entries: 1,
            read_bytes: 1028,
            write_bytes: 80,
            contract_events_size_bytes: 0,
            persistent_rent_ledger_bytes: 327600,
            persistent_entry_rent_bumps: 1,
            temporary_rent_ledger_bytes: 0,
            temporary_entry_rent_bumps: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.cost_estimate().resources()).as_str());
    expect![[r#"
        FeeEstimate {
            total: 45010,
            instructions: 1140,
            read_entries: 18750,
            write_entries: 10000,
            read_bytes: 1793,
            write_bytes: 938,
            contract_events: 0,
            persistent_entry_rent: 12389,
            temporary_entry_rent: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.cost_estimate().fee()).as_str());

    // Read an entry from the storage. Now there are no write-related resources
    // and fees consumed.
    assert_eq!(client.get(&symbol_short!("k1")), Some(symbol_short!("v1")));
    expect![[r#"
        InvocationResources {
            instructions: 454080,
            mem_bytes: 1161338,
            read_entries: 3,
            write_entries: 0,
            read_bytes: 1108,
            write_bytes: 0,
            contract_events_size_bytes: 0,
            persistent_rent_ledger_bytes: 0,
            persistent_entry_rent_bumps: 0,
            temporary_rent_ledger_bytes: 0,
            temporary_entry_rent_bumps: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.cost_estimate().resources()).as_str());
    expect![[r#"
        FeeEstimate {
            total: 21819,
            instructions: 1136,
            read_entries: 18750,
            write_entries: 0,
            read_bytes: 1933,
            write_bytes: 0,
            contract_events: 0,
            persistent_entry_rent: 0,
            temporary_entry_rent: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.cost_estimate().fee()).as_str());

    // Delete the entry. There is 1 write_entry, but 0 write_bytes and no rent
    // as this is deletion.
    client.del(&symbol_short!("k1"));
    expect![[r#"
        InvocationResources {
            instructions: 452458,
            mem_bytes: 1161558,
            read_entries: 2,
            write_entries: 1,
            read_bytes: 1108,
            write_bytes: 0,
            contract_events_size_bytes: 0,
            persistent_rent_ledger_bytes: 0,
            persistent_entry_rent_bumps: 0,
            temporary_rent_ledger_bytes: 0,
            temporary_entry_rent_bumps: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.cost_estimate().resources()).as_str());
    expect![[r#"
        FeeEstimate {
            total: 31815,
            instructions: 1132,
            read_entries: 18750,
            write_entries: 10000,
            read_bytes: 1933,
            write_bytes: 0,
            contract_events: 0,
            persistent_entry_rent: 0,
            temporary_entry_rent: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.cost_estimate().fee()).as_str());

    // Read an entry again, now it no longer exists, so there is less read_bytes
    // than in the case when the entry is present.
    assert_eq!(client.get(&symbol_short!("k1")), None);
    expect![[r#"
        InvocationResources {
            instructions: 452445,
            mem_bytes: 1161202,
            read_entries: 3,
            write_entries: 0,
            read_bytes: 1028,
            write_bytes: 0,
            contract_events_size_bytes: 0,
            persistent_rent_ledger_bytes: 0,
            persistent_entry_rent_bumps: 0,
            temporary_rent_ledger_bytes: 0,
            temporary_entry_rent_bumps: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.cost_estimate().resources()).as_str());
    expect![[r#"
        FeeEstimate {
            total: 21675,
            instructions: 1132,
            read_entries: 18750,
            write_entries: 0,
            read_bytes: 1793,
            write_bytes: 0,
            contract_events: 0,
            persistent_entry_rent: 0,
            temporary_entry_rent: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.cost_estimate().fee()).as_str());
}
