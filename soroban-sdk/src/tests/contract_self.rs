use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, contracttrait, symbol_short, Env};
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn set(&self, v: u32) {
        self.env().storage().instance().set(&symbol_short!("v"), &v);
    }

    pub fn get(&self) -> u32 {
        self.env()
            .storage()
            .instance()
            .get(&symbol_short!("v"))
            .unwrap()
    }

    pub fn add_ref_env(&self, _env: &Env, a: u32, b: u32) -> u32 {
        a + b
    }

    pub fn add_env(&self, _env: Env, a: u32, b: u32) -> u32 {
        a + b
    }

    pub fn add_no_self(a: u32, b: u32) -> u32 {
        a + b
    }
}

#[contracttrait]
pub trait Trait {
    fn trait_get(&self) -> u32;

    fn trait_default(&self, a: u32) -> u32 {
        self.trait_get() + a
    }
}

#[contractimpl(contracttrait)]
impl Trait for Contract {
    fn trait_get(&self) -> u32 {
        self.get()
    }
}

#[test]
fn test_functional() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    client.set(&5);
    assert_eq!(client.get(), 5);
    assert_eq!(client.add_ref_env(&1, &2), 3);
    assert_eq!(client.add_env(&1, &2), 3);
    assert_eq!(client.add_no_self(&1, &2), 3);
    assert_eq!(client.trait_get(), 5);
    assert_eq!(client.trait_default(&1), 6);
}

#[test]
fn test_env() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());
    e.as_contract(&contract_id, || {
        let contract = Contract::__soroban_new(e.clone());
        contract.set(5);
        let v: u32 = e.storage().instance().get(&symbol_short!("v")).unwrap();
        assert_eq!(v, 5);
    });
}

#[test]
fn test_spec_excludes_self() {
    let entries = ScSpecEntry::from_xdr(Contract::spec_xdr_add_ref_env(), Limits::none()).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "add_ref_env".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::U32,
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::U32,
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::U32].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}
