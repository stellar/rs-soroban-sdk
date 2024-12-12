use crate as soroban_sdk;
use expect_test::expect;
use soroban_sdk::Env;
use soroban_sdk_macros::symbol_short;

mod contract_data {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/test_contract_data.wasm"
    );
}

// Update the test data in this test via running it with `UPDATE_EXPECT=1`.
#[test]
fn test_invocation_metering_with_storage() {
    let e = Env::default();
    e.enable_invocation_metering();

    let contract_id = e.register(contract_data::WASM, ());

    // Register operations does both Wasm upload and creating the contract
    // instance. The latter is the last invocation, i.e. the following resources
    // were metered for creating a contract instance.
    expect![[r#"
        InvocationResources {
            instructions: 918961,
            mem_bytes: 2325733,
            read_entries: 2,
            write_entries: 2,
            read_bytes: 928,
            write_bytes: 176,
            contract_events_size_bytes: 0,
            persistent_rent_ledger_bytes: 425880,
            persistent_entry_rent_bumps: 1,
            temporary_rent_ledger_bytes: 454463928,
            temporary_entry_rent_bumps: 1,
        }"#]]
    .assert_eq(format!("{:#?}", e.get_last_invocation_resources().unwrap()).as_str());
    // Note, that the fee mostly comes from bumping the temporary nonce entry
    // to the maximum possible TTL, which is the worst possible case for the
    // signatures that don't expire for as long as possible.
    expect![[r#"
        FeeEstimate {
            total: 1340707,
            instructions: 2298,
            read_entries: 25000,
            write_entries: 20000,
            read_bytes: 1619,
            write_bytes: 2063,
            contract_events: 0,
            persistent_entry_rent: 12937,
            temporary_entry_rent: 1276790,
        }"#]]
    .assert_eq(format!("{:#?}", e.estimate_last_invocation_fee().unwrap()).as_str());

    let client = contract_data::Client::new(&e, &contract_id);

    // Write a single new entry to the storage.
    client.put(&symbol_short!("k1"), &symbol_short!("v1"));
    expect![[r#"
        InvocationResources {
            instructions: 455853,
            mem_bytes: 1162245,
            read_entries: 2,
            write_entries: 1,
            read_bytes: 1032,
            write_bytes: 80,
            contract_events_size_bytes: 0,
            persistent_rent_ledger_bytes: 327600,
            persistent_entry_rent_bumps: 1,
            temporary_rent_ledger_bytes: 0,
            temporary_entry_rent_bumps: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.get_last_invocation_resources().unwrap()).as_str());
    expect![[r#"
        FeeEstimate {
            total: 45017,
            instructions: 1140,
            read_entries: 18750,
            write_entries: 10000,
            read_bytes: 1800,
            write_bytes: 938,
            contract_events: 0,
            persistent_entry_rent: 12389,
            temporary_entry_rent: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.estimate_last_invocation_fee().unwrap()).as_str());

    // Read an entry from the storage. Now there are no write-related resources
    // and fees consumed.
    assert_eq!(client.get(&symbol_short!("k1")), Some(symbol_short!("v1")));
    expect![[r#"
        InvocationResources {
            instructions: 454080,
            mem_bytes: 1161342,
            read_entries: 3,
            write_entries: 0,
            read_bytes: 1112,
            write_bytes: 0,
            contract_events_size_bytes: 0,
            persistent_rent_ledger_bytes: 0,
            persistent_entry_rent_bumps: 0,
            temporary_rent_ledger_bytes: 0,
            temporary_entry_rent_bumps: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.get_last_invocation_resources().unwrap()).as_str());
    expect![[r#"
        FeeEstimate {
            total: 21826,
            instructions: 1136,
            read_entries: 18750,
            write_entries: 0,
            read_bytes: 1940,
            write_bytes: 0,
            contract_events: 0,
            persistent_entry_rent: 0,
            temporary_entry_rent: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.estimate_last_invocation_fee().unwrap()).as_str());

    // Delete the entry. There is 1 write_entry, but 0 write_bytes and no rent
    // as this is deletion.
    client.del(&symbol_short!("k1"));
    expect![[r#"
        InvocationResources {
            instructions: 452458,
            mem_bytes: 1161562,
            read_entries: 2,
            write_entries: 1,
            read_bytes: 1112,
            write_bytes: 0,
            contract_events_size_bytes: 0,
            persistent_rent_ledger_bytes: 0,
            persistent_entry_rent_bumps: 0,
            temporary_rent_ledger_bytes: 0,
            temporary_entry_rent_bumps: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.get_last_invocation_resources().unwrap()).as_str());
    expect![[r#"
        FeeEstimate {
            total: 31822,
            instructions: 1132,
            read_entries: 18750,
            write_entries: 10000,
            read_bytes: 1940,
            write_bytes: 0,
            contract_events: 0,
            persistent_entry_rent: 0,
            temporary_entry_rent: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.estimate_last_invocation_fee().unwrap()).as_str());

    // Read an entry again, now it no longer exists, so there is less read_bytes
    // than in the case when the entry is present.
    assert_eq!(client.get(&symbol_short!("k1")), None);
    expect![[r#"
        InvocationResources {
            instructions: 452445,
            mem_bytes: 1161206,
            read_entries: 3,
            write_entries: 0,
            read_bytes: 1032,
            write_bytes: 0,
            contract_events_size_bytes: 0,
            persistent_rent_ledger_bytes: 0,
            persistent_entry_rent_bumps: 0,
            temporary_rent_ledger_bytes: 0,
            temporary_entry_rent_bumps: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.get_last_invocation_resources().unwrap()).as_str());
    expect![[r#"
        FeeEstimate {
            total: 21682,
            instructions: 1132,
            read_entries: 18750,
            write_entries: 0,
            read_bytes: 1800,
            write_bytes: 0,
            contract_events: 0,
            persistent_entry_rent: 0,
            temporary_entry_rent: 0,
        }"#]]
    .assert_eq(format!("{:#?}", e.estimate_last_invocation_fee().unwrap()).as_str());
}
