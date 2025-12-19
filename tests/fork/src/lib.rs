#![no_std]
use soroban_sdk::contract;

#[contract]
pub struct Contract;

#[cfg(test)]
mod example {
    extern crate std;
    use bytes_lit::bytes;
    use soroban_ledger_fetch::Network;
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
mod test {
    extern crate std;
    use bytes_lit::bytes;
    use soroban_ledger_fetch::Network;
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
