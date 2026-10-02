//! Type aliases in contract types and functions have the spec type of the type
//! they alias. The macros only see an alias's name, so they refer to it through
//! the `SpecTypeDef` trait, which the compiler resolves on the aliased type.

use crate::{self as soroban_sdk};
use soroban_sdk::{
    contract, contractimpl, contracttype, testutils::Address as _, Address, BytesN, Env, Vec,
};
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeBytesN,
    ScSpecTypeDef, ScSpecTypeOption, ScSpecTypeTuple, ScSpecTypeUdt, ScSpecTypeVec,
    ScSpecUdtStructFieldV0, ScSpecUdtStructV0,
};

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

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn aliased(a: Amount, b: Amounts, c: Held) -> Amount {
        let _ = (b, c);
        a
    }
}

const HOLDING: &str = "::soroban_sdk::tests::contract_type_aliases::Holding";

#[test]
fn test_struct_spec() {
    let entry = ScSpecEntry::from_xdr(Aliased::spec_xdr(), Limits::none()).unwrap();
    let expect = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
        doc: "".try_into().unwrap(),
        lib: "".try_into().unwrap(),
        name: "::soroban_sdk::tests::contract_type_aliases::Aliased"
            .try_into()
            .unwrap(),
        fields: vec![
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "amount".try_into().unwrap(),
                type_: ScSpecTypeDef::I128,
            },
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "amounts".try_into().unwrap(),
                type_: ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
                    element_type: Box::new(ScSpecTypeDef::I128),
                })),
            },
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "held".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: HOLDING.try_into().unwrap(),
                }),
            },
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "id".try_into().unwrap(),
                type_: ScSpecTypeDef::BytesN(ScSpecTypeBytesN { n: 32 }),
            },
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "maybe".try_into().unwrap(),
                type_: ScSpecTypeDef::Option(Box::new(ScSpecTypeOption {
                    value_type: Box::new(ScSpecTypeDef::I128),
                })),
            },
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "owner".try_into().unwrap(),
                type_: ScSpecTypeDef::Address,
            },
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "pair".try_into().unwrap(),
                type_: ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
                    value_types: vec![ScSpecTypeDef::I128, ScSpecTypeDef::Address]
                        .try_into()
                        .unwrap(),
                })),
            },
        ]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entry, expect);
}

#[test]
fn test_fn_spec() {
    let entry = ScSpecEntry::from_xdr(Contract::spec_xdr_aliased(), Limits::none()).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "aliased".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::I128,
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
                    element_type: Box::new(ScSpecTypeDef::I128),
                })),
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "c".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: HOLDING.try_into().unwrap(),
                }),
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::I128].try_into().unwrap(),
    });
    assert_eq!(entry, expect);
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
