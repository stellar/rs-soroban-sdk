#![no_std]

use minicbor::{Decode, Encode};
use soroban_sdk::{contractimpl, Bytes, Env};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Request {
    #[n(0)]
    a: u64,
    #[n(1)]
    b: u64,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Response {
    #[n(0)]
    c: u64,
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(e: Env, req: Bytes) -> Bytes {
        let buffer = &mut [0u8; 16];

        let buf = buffer.as_mut();
        req.copy_into_slice(buf);
        let req_buf = &buf[0..req.len() as usize];
        let req: Request = minicbor::decode(req_buf).unwrap();

        let c = req.a + req.b;

        let resp = Response { c };
        let mut buf = buffer.as_mut();
        minicbor::encode(&resp, &mut buf).unwrap();
        Bytes::from_slice(&e, buf)
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

        let buffer = &mut [0u8; 16];
        let req = Request { a: 5, b: 7 };
        minicbor::encode(&req, buffer.as_mut()).unwrap();
        let req = &Bytes::from_slice(&e, buffer);

        let resp = client.add(req);
        let resp_bytes = resp.to_vec();
        let resp: Response = minicbor::decode(&resp_bytes).unwrap();
        println!("{resp:?}");
    }
}
