#![no_std]
use soroban_sdk::contract;

#[contract]
pub struct Contract;

#[cfg(test)]
mod test {
    extern crate std;
    use soroban_ledger_snapshot_source_live::RpcSnapshotSource;
    use soroban_sdk::{token::TokenClient, Address, Env};

    #[test]
    #[ignore]
    fn test_rpc() {
        let rpc = RpcSnapshotSource::new("http://soroban-testnet.stellar.org");
        let e = Env::from_ledger_snapshot(rpc);
        let a = Address::from_str(
            &e,
            "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
        );
        let t = TokenClient::new(&e, &a);

        std::println!("test one");
        let a2 = Address::from_str(
            &e,
            "GCDRCSMJAYHQ5VL4K56STZLJYNHKFXZD4VKA43XCT5UUBFFZCLRR32CP",
        );
        let res = t.balance(&a2);
        std::println!("result {res}");

        //std::println!("waiting");
        //std::thread::sleep(std::time::Duration::from_secs(20));

        //std::println!("test two");
        //let a3 = Address::from_str(
        //    &e,
        //    "GAFUHA24GY66NPICT7KIP4QWZVCWQEL6OMR5QSY6JNDTCBMX72LHPKKQ",
        //);
        //let res2 = t.balance(&a3);
        //std::println!("result {res2}");
    }
}
