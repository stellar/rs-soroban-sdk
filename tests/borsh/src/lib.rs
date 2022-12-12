#![no_std]
#![feature(default_alloc_error_handler)]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate alloc;

use borsh::{BorshDeserialize, BorshSerialize};
use soroban_sdk::{contractimpl, Bytes, Env};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct Request {
    a: u64,
    b: u64,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct Response {
    c: u64,
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(e: Env, req: Bytes) -> Bytes {
        let req = req.to_vec();
        let req = Request::try_from_slice(&req).unwrap();
        let c = req.a + req.b;
        let resp = Response { c };
        let resp = resp.try_to_vec().unwrap();
        Bytes::from_slice(&e, &resp)
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use soroban_sdk::{BytesN, Env};

    use super::*;

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let req = Request { a: 5, b: 7 };
        let req = req.try_to_vec().unwrap();
        let req = &Bytes::from_slice(&e, &req);

        let resp = client.add(req);
        let resp_bytes = resp.to_vec();
        let resp = Response::try_from_slice(&resp_bytes);
        println!("{resp:?}");
    }
}
