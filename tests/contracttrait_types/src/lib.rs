#![no_std]
use soroban_sdk::{contract, contractimpl};

// =============================================================================
// Test: Types in default fns resolve where the trait is defined
// The default fns reference types that are only in scope in the trait's module,
// and none of them are imported where the contract implements the trait.
// =============================================================================

pub mod traits {
    use soroban_sdk::{
        contracterror, contracttrait, contracttype, Address, Env, MuxedAddress, Vec,
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
    }
}

#[contract]
pub struct Contract;

#[contractimpl(contracttrait)]
impl traits::TypesTrait for Contract {}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

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

        let a = Address::generate(&e);
        assert_eq!(client.address(&a, &a), a);
    }
}
