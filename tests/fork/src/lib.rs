#![no_std]
use soroban_sdk::contract;

#[contract]
pub struct Contract;

#[cfg(test)]
mod test {
    extern crate std;
    use soroban_sdk::{token::TokenClient, Address, Env};

    #[test]
    fn test_hello() {
        let rpc = soroban_snapshot_source_rpc::RpcSnapshotSource::new("http://localhost:8000/rpc");
        let e = Env::from_ledger_snapshot(rpc);
        let a = Address::from_str(
            &e,
            "CDMLFMKMMD7MWZP3FKUBZPVHTUEDLSX4BYGYKH4GCESXYHS3IHQ4EIG4",
        );
        let t = TokenClient::new(&e, &a);

        std::println!("test one");
        let a2 = Address::from_str(
            &e,
            "GCDRCSMJAYHQ5VL4K56STZLJYNHKFXZD4VKA43XCT5UUBFFZCLRR32CP",
        );
        let res = t.balance(&a2);
        std::println!("result {res}");

        std::println!("waiting");
        std::thread::sleep(std::time::Duration::from_secs(20));

        std::println!("test two");
        let a3 = Address::from_str(
            &e,
            "GAFUHA24GY66NPICT7KIP4QWZVCWQEL6OMR5QSY6JNDTCBMX72LHPKKQ",
        );
        let res2 = t.balance(&a3);
        std::println!("result {res2}");
    }
}
