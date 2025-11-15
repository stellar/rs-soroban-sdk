#![no_std]

#[cfg(test)]
mod test {
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

        let a2 = Address::from_str(
            &e,
            "GCDRCSMJAYHQ5VL4K56STZLJYNHKFXZD4VKA43XCT5UUBFFZCLRR32CP",
        );
        let res = t.balance(&a2);
        assert_eq!(res, 199999944857);
    }
}
