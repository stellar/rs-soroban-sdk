//! Type aliases in contract types and functions have the spec type of the type
//! they alias. The macros only see an alias's name, so they refer to it through
//! the `SpecTypeDef` trait, which the compiler resolves on the aliased type.

use crate::{self as soroban_sdk};
use soroban_sdk::{
    contract, contractimpl, contracttype, testutils::Address as _, Address, BytesN, Env, Vec,
};
use stellar_xdr::{Limits, ReadXdr, ScSpecEntry, ScSpecTypeDef};

pub type Amount = i128;
pub type Owner = Address;
pub type Id = BytesN<32>;
pub type Amounts = Vec<Amount>;
pub type Pair = (Amount, Owner);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Holding {
    pub owner: Owner,
    pub amount: Amount,
}

pub type Held = Holding;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Aliased {
    pub amount: Amount,
    pub owner: Owner,
    pub id: Id,
    pub amounts: Amounts,
    pub pair: Pair,
    pub maybe: Option<Amount>,
    pub held: Held,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Direct {
    pub amount: i128,
    pub owner: Address,
    pub id: BytesN<32>,
    pub amounts: Vec<i128>,
    pub pair: (i128, Address),
    pub maybe: Option<i128>,
    pub held: Holding,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn aliased(a: Amount, b: Amounts, c: Held) -> Amount {
        let _ = (b, c);
        a
    }

    pub fn direct(a: i128, b: Vec<i128>, c: Holding) -> i128 {
        let _ = (b, c);
        a
    }
}

fn field_types(xdr: &[u8]) -> std::vec::Vec<ScSpecTypeDef> {
    match ScSpecEntry::from_xdr(xdr, Limits::none()).unwrap() {
        ScSpecEntry::UdtStructV0(s) => s.fields.iter().map(|f| f.type_.clone()).collect(),
        e => panic!("unexpected entry {e:?}"),
    }
}

fn fn_types(xdr: &[u8]) -> (std::vec::Vec<ScSpecTypeDef>, std::vec::Vec<ScSpecTypeDef>) {
    match ScSpecEntry::from_xdr(xdr, Limits::none()).unwrap() {
        ScSpecEntry::FunctionV0(f) => (
            f.inputs.iter().map(|i| i.type_.clone()).collect(),
            f.outputs.to_vec(),
        ),
        e => panic!("unexpected entry {e:?}"),
    }
}

#[test]
fn test_struct_fields_with_aliases_have_the_aliased_spec_types() {
    assert_eq!(
        field_types(&Aliased::spec_xdr()),
        field_types(&Direct::spec_xdr())
    );
}

#[test]
fn test_fn_with_aliases_has_the_aliased_spec_types() {
    assert_eq!(
        fn_types(&Contract::spec_xdr_aliased()),
        fn_types(&Contract::spec_xdr_direct())
    );
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let held = Holding {
        owner: Address::generate(&env),
        amount: 3,
    };
    assert_eq!(client.aliased(&5, &Vec::new(&env), &held), 5);
}
