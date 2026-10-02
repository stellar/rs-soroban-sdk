use crate::{self as soroban_sdk, contract, contractimpl, contracttrait, Env};

#[contracttrait]
pub trait Echo<T> {
    fn echo(_env: Env, t: T) -> T {
        t
    }

    fn overridden(_env: Env, t: T) -> T {
        t
    }

    fn not_generic(_env: Env) -> u32 {
        7
    }
}

#[contract]
pub struct ContractU32;

#[contractimpl(contracttrait)]
impl Echo<u32> for ContractU32 {
    fn overridden(_env: Env, t: u32) -> u32 {
        t + 1
    }
}

#[contract]
pub struct ContractI64;

#[contractimpl(contracttrait)]
impl Echo<i64> for ContractI64 {}

#[test]
fn test_generic_default_fns() {
    let e = Env::default();

    let id = e.register(ContractU32, ());
    let client = ContractU32Client::new(&e, &id);
    assert_eq!(client.echo(&5), 5);
    assert_eq!(client.overridden(&5), 6);
    assert_eq!(client.not_generic(), 7);

    let id = e.register(ContractI64, ());
    let client = ContractI64Client::new(&e, &id);
    assert_eq!(client.echo(&-5), -5);
    assert_eq!(client.overridden(&-5), -5);
}

#[test]
fn test_trait_client_omits_generic_fns() {
    let e = Env::default();
    let id = e.register(ContractU32, ());
    // Functions that don't mention the trait's generic params are still on the trait client.
    let client = EchoClient::new(&e, &id);
    assert_eq!(client.not_generic(), 7);
}
