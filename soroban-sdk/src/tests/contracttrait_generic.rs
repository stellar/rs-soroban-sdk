use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, contracttrait, Env, Vec};
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
};

// A trait's generic params can be used in its fn signatures. Each impl's default
// fns are generated with the impl's args in place of the params.
#[contracttrait]
pub trait Echo<T> {
    fn echo(_env: Env, t: T) -> T {
        t
    }

    fn echo_vec(env: Env, t: T) -> Vec<T>
    where
        T: soroban_sdk::IntoVal<Env, soroban_sdk::Val>
            + soroban_sdk::TryFromVal<Env, soroban_sdk::Val>,
    {
        Vec::from_array(&env, [t])
    }

    fn overridden(_env: Env, t: T) -> T {
        t
    }

    fn not_generic(_env: Env) -> u32 {
        1
    }
}

// An impl that leaves out a generic param gets the param's default.
#[contracttrait]
pub trait EchoDefault<T = u64> {
    fn echo_default(_env: Env, t: T) -> T {
        t
    }
}

mod contract_u32 {
    use super::*;

    #[contract]
    pub struct Contract;

    #[contractimpl(contracttrait)]
    impl Echo<u32> for Contract {
        fn overridden(_env: Env, t: u32) -> u32 {
            t + 1
        }
    }

    #[contractimpl(contracttrait)]
    impl EchoDefault for Contract {}
}

mod contract_i64 {
    use super::*;

    #[contract]
    pub struct Contract;

    #[contractimpl(contracttrait)]
    impl super::Echo<i64> for Contract {}
}

#[test]
fn test_generic_default_fns() {
    let e = Env::default();

    let id = e.register(contract_u32::Contract, ());
    let client = contract_u32::ContractClient::new(&e, &id);
    assert_eq!(client.echo(&2u32), 2u32);
    assert_eq!(client.echo_vec(&2u32), Vec::from_array(&e, [2u32]));
    assert_eq!(client.overridden(&2u32), 3u32);
    assert_eq!(client.not_generic(), 1);

    let id = e.register(contract_i64::Contract, ());
    let client = contract_i64::ContractClient::new(&e, &id);
    assert_eq!(client.echo(&-2i64), -2i64);
    assert_eq!(client.echo_vec(&-2i64), Vec::from_array(&e, [-2i64]));
    assert_eq!(client.overridden(&-2i64), -2i64);
    assert_eq!(client.not_generic(), 1);

    // The trait's client leaves out fns that mention the trait's generic
    // params, since they can't be expressed without the impl's args.
    let client = EchoClient::new(&e, &id);
    assert_eq!(client.not_generic(), 1);
}

#[test]
fn test_generic_default_param() {
    let e = Env::default();
    let id = e.register(contract_u32::Contract, ());
    let client = contract_u32::ContractClient::new(&e, &id);
    assert_eq!(client.echo_default(&2u64), 2u64);
}

#[test]
fn test_generic_default_fn_spec() {
    let entry =
        ScSpecEntry::from_xdr(contract_u32::Contract::spec_xdr_echo(), Limits::none()).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "echo".try_into().unwrap(),
        inputs: vec![ScSpecFunctionInputV0 {
            doc: "".try_into().unwrap(),
            name: "t".try_into().unwrap(),
            type_: ScSpecTypeDef::U32,
        }]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::U32].try_into().unwrap(),
    });
    assert_eq!(entry, expect);
}
