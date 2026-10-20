#![no_std]
use soroban_sdk::{contract, contractimpl, crypto::Hash, Bytes, Env, MuxedAddress};

// =============================================================================
// Test: Types in default fns resolve where the trait is defined
// The default fns reference types that are only in scope in the trait's module,
// and none of them are imported where the contract implements the trait.
// =============================================================================

pub mod traits {
    use soroban_sdk::{
        contracterror, contracttrait, contracttype, crypto::Hash, Address, Bytes, Env,
        MuxedAddress, Vec,
    };

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Point {
        pub x: u32,
        pub y: u32,
    }

    #[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Error {
        Odd = 1,
    }

    #[contracttrait]
    pub trait TypesTrait {
        fn point(_env: &Env, x: u32, y: u32) -> Point {
            Point { x, y }
        }

        fn points(env: &Env, p: Point) -> Vec<Point> {
            Vec::from_array(env, [p.clone(), p])
        }

        fn even(_env: &Env, v: u32) -> Result<u32, Error> {
            if v % 2 == 0 {
                Ok(v)
            } else {
                Err(Error::Odd)
            }
        }

        fn address(_env: &Env, a: Address, _m: MuxedAddress) -> Address {
            a
        }

        fn hash(env: &Env, b: Bytes) -> Hash<32> {
            env.crypto().sha256(&b)
        }
    }
}

#[contract]
pub struct Contract;

#[contractimpl(contracttrait)]
impl traits::TypesTrait for Contract {}

#[contractimpl]
impl Contract {
    pub fn hash_direct(env: &Env, b: Bytes) -> Hash<32> {
        env.crypto().sha256(&b)
    }

    pub fn muxed_direct(_env: &Env, m: MuxedAddress) -> MuxedAddress {
        m
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, MuxedAddress};

    #[test]
    fn test_types() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let p = traits::Point { x: 1, y: 2 };
        assert_eq!(client.point(&1, &2), p);
        assert_eq!(client.points(&p).len(), 2);
        assert_eq!(client.even(&2), 2);
        assert_eq!(client.try_even(&3), Err(Ok(traits::Error::Odd)));

        // A muxed address argument accepts anything that converts into one.
        let a = Address::generate(&e);
        assert_eq!(client.address(&a, &a), a);
        assert_eq!(client.address(&a, a.clone()), a);
        let m: MuxedAddress = a.clone().into();
        assert_eq!(client.address(&a, &m), a);
        assert_eq!(client.muxed_direct(&a).address(), a);

        // A returned hash is received by the client as its bytes.
        let b = Bytes::from_array(&e, &[1, 2, 3]);
        let expected: BytesN<32> = e.crypto().sha256(&b).into();
        let h: BytesN<32> = client.hash(&b);
        assert_eq!(h, expected);
        let h: BytesN<32> = client.hash_direct(&b);
        assert_eq!(h, expected);
    }
}
