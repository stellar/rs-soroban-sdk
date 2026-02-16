#![no_std]
use soroban_sdk::contract;

#[contract]
pub struct Contract;

#[cfg(test)]
mod example {
    extern crate std;
    use bytes_lit::bytes;
    use soroban_ledger_snapshot_source_tx::Network;
    use soroban_ledger_snapshot_source_tx::TxSnapshotSource;
    use soroban_sdk::{token::TokenClient, Address, Env};

    #[test]
    fn test() {
        const NATIVE_ADDRESS: &str = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";
        let source = TxSnapshotSource::new(
            Network::testnet(),
            // Ledger.
            8655,
            // Tx to look at state just before.
            Some(bytes!(
                0x580bad1826d02b45634a3742049a23bf652fb2d4bc0814c83f2f58a7a9810ac9
            )),
        );
        let env = Env::from_ledger_snapshot(source);
        let contract = Address::from_str(&env, NATIVE_ADDRESS);
        let client = TokenClient::new(&env, &contract);

        // Lookup balance of address at that ledger just before the given transaction.
        let addr = Address::from_str(
            &env,
            "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM",
        );
        let bal = client.balance(&addr);
        assert_eq!(bal, 47);
    }
}

#[cfg(test)]
mod testnet {
    extern crate std;
    use bytes_lit::bytes;
    use soroban_ledger_snapshot_source_tx::Network;
    use soroban_ledger_snapshot_source_tx::TxSnapshotSource;
    use soroban_sdk::{token::TokenClient, Address, Env};

    // read a tx that is mid ledger
    //
    // where a change occurred to a ledger entry before it, at it, and after it
    //
    // where a change occurred in the ledger before as well
    //
    // and before that ledger, a change didn't occur until back in a history archive
    //
    // SEQ  :  + :  = : TX HASH
    // 8511 :  1 :  1 : 51b21c588f3b397f957d3c14685debf96224c27c5a45b81b27eb5ee0a56517bf
    // 8613 :  2 :  3 : 52e9a67fdd78105ed84f6d073f50a464e14c1c587a5e4b45842e4b3386336751
    // 8622 :  3 :  6 : a5dd4e98df837da3b1a7de52e9698c1a28ae15973f52651c765f1654a3033993
    // 8632 :  6 : 12 : 7a6143351fc48aed1b34d9d0b1690bfe11404cc10cc10f5d52ec06758d0f2b5b
    // 8633 :  7 : 19 : d5f676598fd331867f728c4f1c28ea6045c2d31a80d48bfb873d8302816a6304
    // 8633 :  8 : 27 : 34e420d6cf03ec2ad1560545f6f98ae281bbb6ac9837d23dbc44d158b2942a65
    // 8655 : 11 : 38 : 52ec73f485fb132fb5777c6f2be6c0358e9587c64f4a9ef1f861b56923493400
    // 8655 :  9 : 47 : ccede7115f962f3680dc7ff45b21e52e5ea02004f6c2e5b47cf35b853a91203a
    // 8655 : 10 : 57 : 580bad1826d02b45634a3742049a23bf652fb2d4bc0814c83f2f58a7a9810ac9

    #[rustfmt::skip]
    fn test() {
      test_internal(8510, None);
      test_internal(8511, Some(bytes!(0x51b21c588f3b397f957d3c14685debf96224c27c5a45b81b27eb5ee0a56517bf)));
      test_internal(8613, Some(bytes!(0x52e9a67fdd78105ed84f6d073f50a464e14c1c587a5e4b45842e4b3386336751)));
      test_internal(8622, Some(bytes!(0xa5dd4e98df837da3b1a7de52e9698c1a28ae15973f52651c765f1654a3033993)));
      test_internal(8632, Some(bytes!(0x7a6143351fc48aed1b34d9d0b1690bfe11404cc10cc10f5d52ec06758d0f2b5b)));
      test_internal(8633, Some(bytes!(0xd5f676598fd331867f728c4f1c28ea6045c2d31a80d48bfb873d8302816a6304)));
      test_internal(8633, Some(bytes!(0x34e420d6cf03ec2ad1560545f6f98ae281bbb6ac9837d23dbc44d158b2942a65)));
      test_internal(8633, None);
      test_internal(8655, Some(bytes!(0x52ec73f485fb132fb5777c6f2be6c0358e9587c64f4a9ef1f861b56923493400)));
      test_internal(8655, Some(bytes!(0x580bad1826d02b45634a3742049a23bf652fb2d4bc0814c83f2f58a7a9810ac9)));
      test_internal(8655, Some(bytes!(0xccede7115f962f3680dc7ff45b21e52e5ea02004f6c2e5b47cf35b853a91203a)));
      test_internal(8655, None);
    }

    #[rustfmt::skip]
    fn test_internal(ledger: u32, tx: Option<[u8; 32]>) {
      let s = TxSnapshotSource::new(Network::testnet(), ledger, tx);
      let e = Env::from_ledger_snapshot(s);
      let contract = Address::from_str(&e, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");
      let client = TokenClient::new(&e, &contract);
      let addr = Address::from_str(&e, "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM");
      let res = client.balance(&addr);
      std::println!("\x1b[32m{ledger} {} bal = {res}\x1b[0m", tx.map(|t| std::format!("{:02x}{:02x}{:x}", t[0], t[1], t[2] >> 4)).unwrap_or_default());
    }

    #[rustfmt::skip]
    mod tests {
        use super::test;
        #[test] fn test_1() { test() }
        //#[test] fn test_2() { test() }
        //#[test] fn test_3() { test() }
        //#[test] fn test_4() { test() }
        //#[test] fn test_5() { test() }
        //#[test] fn test_6() { test() }
        //#[test] fn test_7() { test() }
        //#[test] fn test_8() { test() }
        //#[test] fn test_9() { test() }
        //#[test] fn test_10() { test() }
        //#[test] fn test_11() { test() }
        //#[test] fn test_12() { test() }
        //#[test] fn test_13() { test() }
        //#[test] fn test_14() { test() }
        //#[test] fn test_15() { test() }
        //#[test] fn test_16() { test() }
        //#[test] fn test_17() { test() }
        //#[test] fn test_18() { test() }
        //#[test] fn test_19() { test() }
        //#[test] fn test_20() { test() }
        //#[test] fn test_21() { test() }
        //#[test] fn test_22() { test() }
        //#[test] fn test_23() { test() }
        //#[test] fn test_24() { test() }
        //#[test] fn test_25() { test() }
        //#[test] fn test_26() { test() }
        //#[test] fn test_27() { test() }
        //#[test] fn test_28() { test() }
        //#[test] fn test_29() { test() }
        //#[test] fn test_30() { test() }
        //#[test] fn test_31() { test() }
        //#[test] fn test_32() { test() }
        //#[test] fn test_33() { test() }
        //#[test] fn test_34() { test() }
        //#[test] fn test_35() { test() }
        //#[test] fn test_36() { test() }
        //#[test] fn test_37() { test() }
        //#[test] fn test_38() { test() }
        //#[test] fn test_39() { test() }
        //#[test] fn test_40() { test() }
        //#[test] fn test_41() { test() }
        //#[test] fn test_42() { test() }
        //#[test] fn test_43() { test() }
        //#[test] fn test_44() { test() }
        //#[test] fn test_45() { test() }
        //#[test] fn test_46() { test() }
        //#[test] fn test_47() { test() }
        //#[test] fn test_48() { test() }
        //#[test] fn test_49() { test() }
        //#[test] fn test_50() { test() }
    }
}

#[cfg(test)]
mod local {
    extern crate std;
    use ed25519_dalek::{Signer, SigningKey};
    use sha2::{Digest, Sha256};
    use soroban_ledger_snapshot_source_tx::Network;
    use soroban_ledger_snapshot_source_tx::TxSnapshotSource;
    use soroban_sdk::{testutils::EnvTestConfig, token::TokenClient, Address, Env};
    use std::str::FromStr;
    use stellar_rpc_client::Client;
    use stellar_xdr::curr::{
        self as xdr, AccountId, Asset, ContractExecutable, ContractIdPreimage, CreateContractArgs,
        DecoratedSignature, Hash, HashIdPreimage, HashIdPreimageContractId, HostFunction,
        Int128Parts, InvokeContractArgs, InvokeHostFunctionOp, Memo, MuxedAccount, Operation,
        OperationBody, Preconditions, PublicKey, ReadXdr, ScAddress, ScSymbol, ScVal,
        SequenceNumber, Signature, SignatureHint, SorobanAuthorizationEntry, Transaction,
        TransactionEnvelope, TransactionExt, TransactionSignaturePayload,
        TransactionSignaturePayloadTaggedTransaction, TransactionV1Envelope, Uint256, VecM,
        WriteXdr,
    };

    const RPC_URL: &str = "http://localhost:8000/rpc";
    const NETWORK_PASSPHRASE: &str = "Standalone Network ; February 2017";
    const TARGET_ADDRESS: &str = "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM";

    #[test]
    fn test_local_fork() {
        // Run async part to submit transactions
        // tx1: transfers 5, before checkpoint
        // tx2: transfers 10, after checkpoint
        // tx3: transfers 20, after checkpoint (same ledger as tx2)
        let result = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(submit_transactions_async());

        // Verify tx1 balance (requires archive lookup since it's before checkpoint)
        std::println!("Verifying tx1 balance (requires archive lookup)...");
        let balance_before_tx1 =
            get_balance_at(&result.sac_contract, result.ledger1, Some(result.hash1));
        let balance_after_tx1 = get_balance_at(&result.sac_contract, result.ledger1, None);
        std::println!("Balance before tx1 (transfers 5): {balance_before_tx1}");
        std::println!("Balance after tx1: {balance_after_tx1}");
        assert_eq!(
            balance_after_tx1,
            balance_before_tx1 + 5,
            "tx1 should add 5"
        );

        // Verify balance 8 ledgers before tx2/tx3 (requires archive lookup across checkpoint)
        let ledger_before_checkpoint = result.ledger2 - 1;
        std::println!(
            "Verifying balance at ledger {} (8 ledgers before tx2, requires archive lookup)...",
            ledger_before_checkpoint
        );
        let balance_at_checkpoint =
            get_balance_at(&result.sac_contract, ledger_before_checkpoint, None);
        std::println!(
            "Balance at ledger {}: {balance_at_checkpoint}",
            ledger_before_checkpoint
        );
        // This should equal the balance after tx1 since no other transactions affected TARGET_ADDRESS
        assert_eq!(
            balance_at_checkpoint, balance_after_tx1,
            "balance 8 ledgers before tx2 should equal balance after tx1"
        );

        // Verify tx2/tx3 balances (intra-ledger state)
        if result.ledger2 == result.ledger3 {
            let ledger = result.ledger2;
            std::println!("tx2 and tx3 in same ledger {ledger}! Testing intra-ledger state...");

            // Get balances at different points in the ledger
            let balance_before_tx2 =
                get_balance_at(&result.sac_contract, ledger, Some(result.hash2));
            let balance_before_tx3 =
                get_balance_at(&result.sac_contract, ledger, Some(result.hash3));
            let balance_end = get_balance_at(&result.sac_contract, ledger, None);

            // Determine transaction order based on balances
            // tx2 transfers 10, tx3 transfers 20
            // The one with lower "before" balance came first
            if balance_before_tx2 < balance_before_tx3 {
                // tx2 came first: before_tx2 -> +10 -> before_tx3 -> +20 -> end
                std::println!("Transaction order: tx2 then tx3");
                std::println!("Balance before tx2 (transfers 10): {balance_before_tx2}");
                std::println!("Balance before tx3 (transfers 20): {balance_before_tx3}");
                std::println!("Balance at end of ledger: {balance_end}");
                assert_eq!(
                    balance_before_tx3,
                    balance_before_tx2 + 10,
                    "tx2 should add 10"
                );
                assert_eq!(balance_end, balance_before_tx3 + 20, "tx3 should add 20");
            } else {
                // tx3 came first: before_tx3 -> +20 -> before_tx2 -> +10 -> end
                std::println!("Transaction order: tx3 then tx2");
                std::println!("Balance before tx3 (transfers 20): {balance_before_tx3}");
                std::println!("Balance before tx2 (transfers 10): {balance_before_tx2}");
                std::println!("Balance at end of ledger: {balance_end}");
                assert_eq!(
                    balance_before_tx2,
                    balance_before_tx3 + 20,
                    "tx3 should add 20"
                );
                assert_eq!(balance_end, balance_before_tx2 + 10, "tx2 should add 10");
            }

            // Either way, total change from tx2+tx3 should be 30
            let initial_balance = std::cmp::min(balance_before_tx2, balance_before_tx3);
            assert_eq!(
                balance_end,
                initial_balance + 30,
                "tx2+tx3 total change should be 30"
            );

            // Verify the balance before tx2/tx3 equals balance after tx1 (with +5 from tx1)
            assert_eq!(
                initial_balance, balance_after_tx1,
                "balance before tx2/tx3 should equal balance after tx1"
            );
        } else {
            std::println!("tx2 and tx3 landed in different ledgers, testing cross-ledger state...");

            // Verify state at end of each ledger
            let balance2 = get_balance_at(&result.sac_contract, result.ledger2, None);
            let balance3 = get_balance_at(&result.sac_contract, result.ledger3, None);
            std::println!("Balance at ledger {}: {balance2}", result.ledger2);
            std::println!("Balance at ledger {}: {balance3}", result.ledger3);
            assert_eq!(balance2, balance_after_tx1 + 10, "tx2 should add 10");
            assert_eq!(
                balance3,
                balance_after_tx1 + 30,
                "tx2+tx3 should add 30 total"
            );
        }

        // Verify total: balance should have increased by 5 + 10 + 20 = 35
        let final_balance = get_balance_at(
            &result.sac_contract,
            result.ledger3.max(result.ledger2),
            None,
        );
        assert_eq!(
            final_balance,
            balance_before_tx1 + 35,
            "total change should be 35"
        );
    }

    fn network_id() -> [u8; 32] {
        Sha256::digest(NETWORK_PASSPHRASE.as_bytes()).into()
    }

    fn signing_key_to_account_id(key: &SigningKey) -> AccountId {
        AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
            key.verifying_key().to_bytes(),
        )))
    }

    fn signing_key_to_sc_address(key: &SigningKey) -> ScAddress {
        ScAddress::Account(signing_key_to_account_id(key))
    }

    fn signing_key_to_strkey(key: &SigningKey) -> std::string::String {
        stellar_strkey::Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(
            key.verifying_key().to_bytes(),
        ))
        .to_string()
    }

    fn i128_to_sc_val(v: i128) -> ScVal {
        let hi = (v >> 64) as i64;
        let lo = v as u64;
        ScVal::I128(Int128Parts { hi, lo })
    }

    fn native_sac_contract_id() -> [u8; 32] {
        // Compute the contract ID for the native asset SAC using XDR HashIdPreimage
        let preimage = HashIdPreimage::ContractId(HashIdPreimageContractId {
            network_id: Hash(network_id()),
            contract_id_preimage: ContractIdPreimage::Asset(Asset::Native),
        });
        let preimage_xdr = preimage.to_xdr(xdr::Limits::none()).unwrap();
        Sha256::digest(&preimage_xdr).into()
    }

    fn native_sac_address() -> std::string::String {
        let contract_id = native_sac_contract_id();
        let strkey = stellar_strkey::Strkey::Contract(stellar_strkey::Contract(contract_id));
        std::format!("{strkey}")
    }

    fn build_deploy_native_sac_op() -> Operation {
        let host_function = HostFunction::CreateContract(CreateContractArgs {
            contract_id_preimage: ContractIdPreimage::Asset(Asset::Native),
            executable: ContractExecutable::StellarAsset,
        });

        Operation {
            source_account: None,
            body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                host_function,
                auth: VecM::default(),
            }),
        }
    }

    fn build_sac_transfer_op(
        sac_contract: &str,
        from: ScAddress,
        to: ScAddress,
        amount: i128,
    ) -> Operation {
        let contract_address = ScAddress::from_str(sac_contract).unwrap();
        let host_function = HostFunction::InvokeContract(InvokeContractArgs {
            contract_address,
            function_name: ScSymbol("transfer".try_into().unwrap()),
            args: std::vec![
                ScVal::Address(from),
                ScVal::Address(to),
                i128_to_sc_val(amount)
            ]
            .try_into()
            .unwrap(),
        });

        Operation {
            source_account: None,
            body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                host_function,
                auth: VecM::default(),
            }),
        }
    }

    fn build_transaction(
        source_account: &AccountId,
        sequence_number: i64,
        operations: std::vec::Vec<Operation>,
    ) -> Transaction {
        Transaction {
            source_account: MuxedAccount::Ed25519(match &source_account.0 {
                PublicKey::PublicKeyTypeEd25519(pk) => pk.clone(),
            }),
            fee: 10_000_000, // Will be updated after simulation
            seq_num: SequenceNumber(sequence_number),
            cond: Preconditions::None,
            memo: Memo::None,
            operations: operations.try_into().unwrap(),
            ext: TransactionExt::V0,
        }
    }

    fn sign_transaction(tx: &Transaction, key: &SigningKey) -> DecoratedSignature {
        let payload = TransactionSignaturePayload {
            network_id: Hash(network_id()),
            tagged_transaction: TransactionSignaturePayloadTaggedTransaction::Tx(tx.clone()),
        };
        let payload_bytes = payload.to_xdr(xdr::Limits::none()).unwrap();
        let payload_hash = Sha256::digest(&payload_bytes);
        let signature = key.sign(&payload_hash);

        let pk_bytes = key.verifying_key().to_bytes();
        let hint = SignatureHint([pk_bytes[28], pk_bytes[29], pk_bytes[30], pk_bytes[31]]);

        DecoratedSignature {
            hint,
            signature: Signature(signature.to_bytes().to_vec().try_into().unwrap()),
        }
    }

    async fn fund_account(
        client: &Client,
        address: &str,
    ) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
        let friendbot_url = client.friendbot_url().await?;
        if !friendbot_url.is_empty() {
            let fund_url = std::format!("{friendbot_url}?addr={address}");
            reqwest::get(&fund_url).await?.error_for_status()?;
        }
        Ok(())
    }

    async fn get_sequence_number(
        client: &Client,
        account_id: &AccountId,
    ) -> Result<i64, std::boxed::Box<dyn std::error::Error>> {
        use std::string::ToString;
        let account = client.get_account(&account_id.to_string()).await?;
        Ok(account.seq_num.0)
    }

    async fn simulate_and_prepare(
        client: &Client,
        tx: Transaction,
    ) -> Result<Transaction, std::boxed::Box<dyn std::error::Error>> {
        let envelope = TransactionEnvelope::Tx(TransactionV1Envelope {
            tx: tx.clone(),
            signatures: VecM::default(),
        });
        let sim = client
            .simulate_transaction_envelope(&envelope, None)
            .await?;

        // Check for simulation error
        if let Some(error) = &sim.error {
            return Err(std::format!("Simulation failed: {error}").into());
        }

        // Get resource fee and auth from simulation
        let min_resource_fee = sim.min_resource_fee;

        // Update the transaction with auth from simulation
        let mut new_tx = tx;
        new_tx.fee = new_tx.fee.saturating_add(min_resource_fee as u32);

        if let Some(result) = sim.results.first() {
            if !result.auth.is_empty() {
                let mut ops: std::vec::Vec<Operation> = new_tx.operations.to_vec();
                if let OperationBody::InvokeHostFunction(ref mut op) = ops.first_mut().unwrap().body
                {
                    let auth_vec: std::vec::Vec<SorobanAuthorizationEntry> = result
                        .auth
                        .iter()
                        .map(|a| {
                            SorobanAuthorizationEntry::from_xdr_base64(a, xdr::Limits::none())
                                .unwrap()
                        })
                        .collect();
                    op.auth = auth_vec.try_into().unwrap();
                }
                new_tx.operations = ops.try_into().unwrap();
            }
        }

        // Set transaction resources from simulation
        if !sim.transaction_data.is_empty() {
            let soroban_data = xdr::SorobanTransactionData::from_xdr_base64(
                &sim.transaction_data,
                xdr::Limits::none(),
            )?;
            new_tx.ext = TransactionExt::V1(soroban_data);
        }

        Ok(new_tx)
    }

    fn get_balance_at(sac_contract: &str, ledger: u32, tx_hash: Option<[u8; 32]>) -> i128 {
        let source = TxSnapshotSource::new(Network::local(), ledger, tx_hash);
        let mut env = Env::from_ledger_snapshot(source);
        env.set_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let contract = Address::from_str(&env, sac_contract);
        let client = TokenClient::new(&env, &contract);
        let addr = Address::from_str(&env, TARGET_ADDRESS);
        client.balance(&addr)
    }

    struct SubmitResult {
        ledger1: u32,
        ledger2: u32,
        ledger3: u32,
        hash1: [u8; 32],
        hash2: [u8; 32],
        hash3: [u8; 32],
        sac_contract: std::string::String,
    }

    async fn submit_transactions_async() -> SubmitResult {
        let client = Client::new(RPC_URL).unwrap();

        // Create two source accounts with random keys
        let key1 = SigningKey::generate(&mut rand::rngs::OsRng);
        let key2 = SigningKey::generate(&mut rand::rngs::OsRng);

        let addr1 = signing_key_to_strkey(&key1);
        let addr2 = signing_key_to_strkey(&key2);

        std::println!("Account 1: {addr1}");
        std::println!("Account 2: {addr2}");

        // Fund both accounts in parallel
        std::println!("Funding accounts...");
        let (fund1, fund2) =
            tokio::join!(fund_account(&client, &addr1), fund_account(&client, &addr2),);
        fund1.expect("failed to fund account 1");
        fund2.expect("failed to fund account 2");

        // Wait for accounts to be created
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let account1_id = signing_key_to_account_id(&key1);
        let account2_id = signing_key_to_account_id(&key2);

        // Deploy the native SAC if it doesn't exist
        let sac_contract = native_sac_address();
        std::println!("Native SAC: {sac_contract}");

        // Check if SAC already exists by trying to simulate the deploy
        let deploy_op = build_deploy_native_sac_op();
        let deploy_seq = get_sequence_number(&client, &account1_id).await.unwrap();
        let deploy_tx = build_transaction(&account1_id, deploy_seq + 1, std::vec![deploy_op]);

        // Try to simulate - if it fails with "already exists" that's fine
        let deploy_envelope = TransactionEnvelope::Tx(TransactionV1Envelope {
            tx: deploy_tx.clone(),
            signatures: VecM::default(),
        });
        let deploy_sim = client
            .simulate_transaction_envelope(&deploy_envelope, None)
            .await
            .expect("failed to simulate deploy");

        if deploy_sim.error.is_none() {
            // SAC doesn't exist yet, deploy it
            std::println!("Deploying native SAC...");
            let prepared_deploy = {
                let mut tx = deploy_tx;
                tx.fee = tx.fee.saturating_add(deploy_sim.min_resource_fee as u32);
                if !deploy_sim.transaction_data.is_empty() {
                    let soroban_data = xdr::SorobanTransactionData::from_xdr_base64(
                        &deploy_sim.transaction_data,
                        xdr::Limits::none(),
                    )
                    .unwrap();
                    tx.ext = TransactionExt::V1(soroban_data);
                }
                tx
            };

            let deploy_sig = sign_transaction(&prepared_deploy, &key1);
            let signed_deploy = TransactionEnvelope::Tx(TransactionV1Envelope {
                tx: prepared_deploy,
                signatures: std::vec![deploy_sig].try_into().unwrap(),
            });

            let deploy_hash = signed_deploy
                .hash(network_id())
                .expect("failed to hash deploy tx");
            client
                .send_transaction(&signed_deploy)
                .await
                .expect("failed to send deploy tx");
            client
                .get_transaction_polling(&Hash(deploy_hash), None)
                .await
                .expect("failed to get deploy tx result");
            std::println!("Native SAC deployed");
        } else {
            std::println!("Native SAC already exists");
        }

        // Submit tx1 first (before checkpoint gap) - transfers 5
        std::println!("Submitting tx1 (before checkpoint gap)...");
        let from1 = signing_key_to_sc_address(&key1);
        let to = ScAddress::from_str(TARGET_ADDRESS).unwrap();

        let seq1_for_tx1 = get_sequence_number(&client, &account1_id).await.unwrap();
        let op1 = build_sac_transfer_op(&sac_contract, from1.clone(), to.clone(), 5);
        let tx1 = build_transaction(&account1_id, seq1_for_tx1 + 1, std::vec![op1]);
        let prepared_tx1 = simulate_and_prepare(&client, tx1)
            .await
            .expect("failed to simulate tx1");
        let sig1 = sign_transaction(&prepared_tx1, &key1);
        let envelope1 = TransactionEnvelope::Tx(TransactionV1Envelope {
            tx: prepared_tx1,
            signatures: std::vec![sig1].try_into().unwrap(),
        });
        let hash1_bytes = envelope1.hash(network_id()).expect("failed to hash tx1");
        let hash1 = Hash(hash1_bytes);
        client
            .send_transaction(&envelope1)
            .await
            .expect("failed to send tx1");
        let tx1_result = client
            .get_transaction_polling(&hash1, None)
            .await
            .expect("failed to get tx1");
        let ledger1 = tx1_result.ledger.expect("missing ledger for tx1");
        std::println!("Tx1 hash: {}", hex::encode(hash1_bytes));
        std::println!("Tx1 ledger: {ledger1}");

        // Wait for at least 9 ledgers to pass (crosses a checkpoint boundary on local network)
        // Local quickstart has checkpoint frequency of 8
        std::println!("Waiting for 9+ ledgers to pass (checkpoint boundary)...");
        loop {
            let current = client.get_latest_ledger().await.unwrap().sequence;
            let gap = current.saturating_sub(ledger1);
            if gap >= 9 {
                std::println!("Current ledger: {current}, gap from tx1: {gap}");
                break;
            }
            std::println!("Current ledger: {current}, gap from tx1: {gap}, waiting...");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        // Get sequence numbers for tx2/tx3
        let (seq1_result, seq2_result) = tokio::join!(
            get_sequence_number(&client, &account1_id),
            get_sequence_number(&client, &account2_id),
        );
        let seq1 = seq1_result.expect("failed to get seq for account 1");
        let seq2 = seq2_result.expect("failed to get seq for account 2");

        // Build transfer operations for tx2 and tx3
        let from2 = signing_key_to_sc_address(&key2);

        let op2 = build_sac_transfer_op(&sac_contract, from1, to.clone(), 10);
        let op3 = build_sac_transfer_op(&sac_contract, from2, to, 20);

        // Build tx2 and tx3
        let tx2 = build_transaction(&account1_id, seq1 + 1, std::vec![op2]);
        let tx3 = build_transaction(&account2_id, seq2 + 1, std::vec![op3]);

        // Simulate and prepare both transactions in parallel
        std::println!("Simulating tx2 and tx3...");
        let (prepared2, prepared3) = tokio::join!(
            simulate_and_prepare(&client, tx2),
            simulate_and_prepare(&client, tx3),
        );
        let prepared_tx2 = prepared2.expect("failed to simulate tx2");
        let prepared_tx3 = prepared3.expect("failed to simulate tx3");

        // Sign both transactions
        let sig2 = sign_transaction(&prepared_tx2, &key1);
        let sig3 = sign_transaction(&prepared_tx3, &key2);

        let envelope2 = TransactionEnvelope::Tx(TransactionV1Envelope {
            tx: prepared_tx2,
            signatures: std::vec![sig2].try_into().unwrap(),
        });
        let envelope3 = TransactionEnvelope::Tx(TransactionV1Envelope {
            tx: prepared_tx3,
            signatures: std::vec![sig3].try_into().unwrap(),
        });

        // Compute hashes before sending
        let hash2_bytes = envelope2.hash(network_id()).expect("failed to hash tx2");
        let hash3_bytes = envelope3.hash(network_id()).expect("failed to hash tx3");
        let hash2 = Hash(hash2_bytes);
        let hash3 = Hash(hash3_bytes);

        // Send both transactions simultaneously to maximize chance of same ledger
        std::println!("Sending tx2 and tx3...");
        let (send2, send3) = tokio::join!(
            client.send_transaction(&envelope2),
            client.send_transaction(&envelope3),
        );
        send2.expect("failed to send tx2");
        send3.expect("failed to send tx3");

        std::println!("Tx2 hash: {}", hex::encode(hash2_bytes));
        std::println!("Tx3 hash: {}", hex::encode(hash3_bytes));

        // Poll for both transactions to complete in parallel
        std::println!("Waiting for tx2 and tx3 to complete...");
        let (poll2, poll3) = tokio::join!(
            client.get_transaction_polling(&hash2, None),
            client.get_transaction_polling(&hash3, None),
        );
        let tx2_result = poll2.expect("failed to get tx2");
        let tx3_result = poll3.expect("failed to get tx3");

        let ledger2 = tx2_result.ledger.expect("missing ledger for tx2");
        let ledger3 = tx3_result.ledger.expect("missing ledger for tx3");

        std::println!("Tx2 ledger: {ledger2}");
        std::println!("Tx3 ledger: {ledger3}");

        // Wait for ledger data to be available in meta storage
        std::println!("Waiting for ledger data to be available...");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        SubmitResult {
            ledger1,
            ledger2,
            ledger3,
            hash1: hash1_bytes,
            hash2: hash2_bytes,
            hash3: hash3_bytes,
            sac_contract,
        }
    }
}
