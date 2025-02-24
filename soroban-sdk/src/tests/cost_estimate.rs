use crate as soroban_sdk;
use crate::env::xdr::ContractCostType;
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

#[test]
fn test_cost_estimate_budget() {
    let e = Env::default();

    let contract_id = e.register(contract_data::WASM, ());
    let client = contract_data::Client::new(&e, &contract_id);

    client.put(&symbol_short!("k1"), &symbol_short!("v1"));

    // Budget breakdown corresponds to the last invocation only.
    expect![[r#"
        ===============================================================================================================================================================================
        Cpu limit: 100000000; used: 455853
        Mem limit: 41943040; used: 1162241
        ===============================================================================================================================================================================
        CostType                           iterations     input          cpu_insns      mem_bytes      const_term_cpu      lin_term_cpu        const_term_mem      lin_term_mem        
        WasmInsnExec                       284            None           1136           0              4                   0                   0                   0                   
        MemAlloc                           27             Some(1052425)  143269         1052857        434                 16                  16                  128                 
        MemCpy                             95             Some(9665)     5186           0              42                  16                  0                   0                   
        MemCmp                             43             Some(1049)     2012           0              44                  16                  0                   0                   
        DispatchHostFunction               1              None           310            0              310                 0                   0                   0                   
        VisitObject                        2              None           122            0              61                  0                   0                   0                   
        ValSer                             0              Some(0)        0              0              230                 29                  242                 384                 
        ValDeser                           0              Some(0)        0              0              59052               4001                0                   384                 
        ComputeSha256Hash                  1              Some(0)        3738           0              3738                7012                0                   0                   
        ComputeEd25519PubKey               0              None           0              0              40253               0                   0                   0                   
        VerifyEd25519Sig                   0              Some(0)        0              0              377524              4068                0                   0                   
        VmInstantiation                    0              Some(0)        0              0              451626              45405               130065              5064                
        VmCachedInstantiation              0              Some(0)        0              0              41142               634                 69472               1217                
        InvokeVmFunction                   1              None           1948           14             1948                0                   14                  0                   
        ComputeKeccak256Hash               0              Some(0)        0              0              3766                5969                0                   0                   
        DecodeEcdsaCurve256Sig             0              None           0              0              710                 0                   0                   0                   
        RecoverEcdsaSecp256k1Key           0              None           0              0              2315295             0                   181                 0                   
        Int256AddSub                       0              None           0              0              4404                0                   99                  0                   
        Int256Mul                          0              None           0              0              4947                0                   99                  0                   
        Int256Div                          0              None           0              0              4911                0                   99                  0                   
        Int256Pow                          0              None           0              0              4286                0                   99                  0                   
        Int256Shift                        0              None           0              0              913                 0                   99                  0                   
        ChaCha20DrawBytes                  0              Some(0)        0              0              1058                501                 0                   0                   
        ParseWasmInstructions              1              Some(137)      100273         24475          73077               25410               17564               6457                
        ParseWasmFunctions                 1              Some(5)        21123          1854           0                   540752              0                   47464               
        ParseWasmGlobals                   1              Some(3)        4133           314            0                   176363              0                   13420               
        ParseWasmTableEntries              1              Some(0)        0              0              0                   29989               0                   6285                
        ParseWasmTypes                     1              Some(5)        41462          2526           0                   1061449             0                   64670               
        ParseWasmDataSegments              1              Some(0)        0              0              0                   237336              0                   29074               
        ParseWasmElemSegments              1              Some(0)        0              0              0                   328476              0                   48095               
        ParseWasmImports                   1              Some(4)        21932          3225           0                   701845              0                   103229              
        ParseWasmExports                   1              Some(7)        23481          1990           0                   429383              0                   36394               
        ParseWasmDataSegmentBytes          1              Some(0)        0              0              0                   28                  0                   257                 
        InstantiateWasmInstructions        1              None           43030          70704          43030               0                   70704               0                   
        InstantiateWasmFunctions           1              Some(5)        295            570            0                   7556                0                   14613               
        InstantiateWasmGlobals             1              Some(3)        251            160            0                   10711               0                   6833                
        InstantiateWasmTableEntries        1              Some(0)        0              0              0                   3300                0                   1025                
        InstantiateWasmTypes               1              None           0              0              0                   0                   0                   0                   
        InstantiateWasmDataSegments        1              Some(0)        0              0              0                   23038               0                   129632              
        InstantiateWasmElemSegments        1              Some(0)        0              0              0                   42488               0                   13665               
        InstantiateWasmImports             1              Some(4)        25905          3051           0                   828974              0                   97637               
        InstantiateWasmExports             1              Some(7)        16247          501            0                   297100              0                   9176                
        InstantiateWasmDataSegmentBytes    1              Some(0)        0              0              0                   14                  0                   126                 
        Sec1DecodePointUncompressed        0              None           0              0              1882                0                   0                   0                   
        VerifyEcdsaSecp256r1Sig            0              None           0              0              3000906             0                   0                   0                   
        Bls12381EncodeFp                   0              None           0              0              661                 0                   0                   0                   
        Bls12381DecodeFp                   0              None           0              0              985                 0                   0                   0                   
        Bls12381G1CheckPointOnCurve        0              None           0              0              1934                0                   0                   0                   
        Bls12381G1CheckPointInSubgroup     0              None           0              0              730510              0                   0                   0                   
        Bls12381G2CheckPointOnCurve        0              None           0              0              5921                0                   0                   0                   
        Bls12381G2CheckPointInSubgroup     0              None           0              0              1057822             0                   0                   0                   
        Bls12381G1ProjectiveToAffine       0              None           0              0              92642               0                   0                   0                   
        Bls12381G2ProjectiveToAffine       0              None           0              0              100742              0                   0                   0                   
        Bls12381G1Add                      0              None           0              0              7689                0                   0                   0                   
        Bls12381G1Mul                      0              None           0              0              2458985             0                   0                   0                   
        Bls12381G1Msm                      0              Some(0)        0              0              2426722             96397671            109494              354667              
        Bls12381MapFpToG1                  0              None           0              0              1541554             0                   5552                0                   
        Bls12381HashToG1                   0              Some(0)        0              0              3211191             6713                9424                0                   
        Bls12381G2Add                      0              None           0              0              25207               0                   0                   0                   
        Bls12381G2Mul                      0              None           0              0              7873219             0                   0                   0                   
        Bls12381G2Msm                      0              Some(0)        0              0              8035968             309667335           219654              354667              
        Bls12381MapFp2ToG2                 0              None           0              0              2420202             0                   3344                0                   
        Bls12381HashToG2                   0              Some(0)        0              0              7050564             6797                6816                0                   
        Bls12381Pairing                    0              Some(0)        0              0              10558948            632860943           2204                9340474             
        Bls12381FrFromU256                 0              None           0              0              1994                0                   0                   0                   
        Bls12381FrToU256                   0              None           0              0              1155                0                   248                 0                   
        Bls12381FrAddSub                   0              None           0              0              74                  0                   0                   0                   
        Bls12381FrMul                      0              None           0              0              332                 0                   0                   0                   
        Bls12381FrPow                      0              Some(0)        0              0              691                 74558               0                   128                 
        Bls12381FrInv                      0              None           0              0              35421               0                   0                   0                   
        ===============================================================================================================================================================================
        Internal details (diagnostics info, does not affect fees) 
        Total # times meter was called: 192
        Shadow cpu limit: 100000000; used: 34332
        Shadow mem limit: 41943040; used: 27725
        ===============================================================================================================================================================================


    "#]]
    .assert_eq(format!("{:#?}", e.cost_estimate().budget()).as_str());

    // Data for a specific cost type (one row in the budget table above) can be
    // obtained as well via `tracker` function.
    expect![[r#"
        CostTracker {
            iterations: 284,
            inputs: None,
            cpu: 1136,
            mem: 0,
        }"#]]
    .assert_eq(
        format!(
            "{:#?}",
            e.cost_estimate()
                .budget()
                .tracker(ContractCostType::WasmInsnExec)
        )
        .as_str(),
    );
}
