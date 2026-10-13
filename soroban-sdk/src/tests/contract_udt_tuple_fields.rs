use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, contracttype, Env, IntoVal, TryFromVal, Val};
use stellar_xdr::{
    Limits, ReadXdr, ScMap, ScMapEntry, ScSpecEntry, ScSpecTypeDef, ScSpecTypeTuple,
    ScSpecUdtStructFieldV0, ScSpecUdtStructV0, ScSpecUdtUnionCaseTupleV0, ScSpecUdtUnionCaseV0,
    ScSpecUdtUnionV0, ScSymbol, ScVal, ScVec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Struct {
    pub tuple: (u32, i64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TupleStruct(pub (u32, i64));

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Enum {
    Tuple((u32, i64)),
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn echo(s: Struct, t: TupleStruct, e: Enum) -> (Struct, TupleStruct, Enum) {
        (s, t, e)
    }
}

fn values() -> (Struct, TupleStruct, Enum) {
    (
        Struct { tuple: (1, -2) },
        TupleStruct((3, -4)),
        Enum::Tuple((5, -6)),
    )
}

#[test]
fn test_val_roundtrip() {
    let env = Env::default();
    let (s, t, e) = values();

    let val: Val = s.clone().into_val(&env);
    assert_eq!(Struct::try_from_val(&env, &val), Ok(s));
    let val: Val = t.clone().into_val(&env);
    assert_eq!(TupleStruct::try_from_val(&env, &val), Ok(t));
    let val: Val = e.clone().into_val(&env);
    assert_eq!(Enum::try_from_val(&env, &val), Ok(e));
}

fn scvec(items: std::vec::Vec<ScVal>) -> ScVal {
    ScVal::Vec(Some(ScVec(items.try_into().unwrap())))
}

fn symbol(s: &str) -> ScVal {
    ScVal::Symbol(ScSymbol(s.try_into().unwrap()))
}

#[test]
fn test_scval_roundtrip() {
    let env = Env::default();
    let (s, t, e) = values();

    let scval: ScVal = (&s).try_into().unwrap();
    assert_eq!(
        scval,
        ScVal::Map(Some(
            ScMap::sorted_from(std::vec![ScMapEntry {
                key: symbol("tuple"),
                val: scvec(std::vec![ScVal::U32(1), ScVal::I64(-2)]),
            }])
            .unwrap()
        ))
    );
    assert_eq!(Struct::try_from_val(&env, &scval), Ok(s));

    let scval: ScVal = (&t).try_into().unwrap();
    assert_eq!(
        scval,
        scvec(std::vec![scvec(std::vec![ScVal::U32(3), ScVal::I64(-4)])])
    );
    assert_eq!(TupleStruct::try_from_val(&env, &scval), Ok(t));

    let scval: ScVal = (&e).try_into().unwrap();
    assert_eq!(
        scval,
        scvec(std::vec![
            symbol("Tuple"),
            scvec(std::vec![ScVal::U32(5), ScVal::I64(-6)]),
        ])
    );
    assert_eq!(Enum::try_from_val(&env, &scval), Ok(e));
}

fn tuple_type() -> ScSpecTypeDef {
    ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
        value_types: std::vec![ScSpecTypeDef::U32, ScSpecTypeDef::I64]
            .try_into()
            .unwrap(),
    }))
}

#[test]
fn test_spec() {
    let entry = ScSpecEntry::from_xdr(Struct::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        entry,
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "::soroban_sdk::tests::contract_udt_tuple_fields::Struct"
                .try_into()
                .unwrap(),
            fields: [ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "tuple".try_into().unwrap(),
                type_: tuple_type(),
            }]
            .try_into()
            .unwrap(),
        }),
    );

    let entry = ScSpecEntry::from_xdr(TupleStruct::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        entry,
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "::soroban_sdk::tests::contract_udt_tuple_fields::TupleStruct"
                .try_into()
                .unwrap(),
            fields: [ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "0".try_into().unwrap(),
                type_: tuple_type(),
            }]
            .try_into()
            .unwrap(),
        }),
    );

    let entry = ScSpecEntry::from_xdr(Enum::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        entry,
        ScSpecEntry::UdtUnionV0(ScSpecUdtUnionV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "::soroban_sdk::tests::contract_udt_tuple_fields::Enum"
                .try_into()
                .unwrap(),
            cases: [ScSpecUdtUnionCaseV0::TupleV0(ScSpecUdtUnionCaseTupleV0 {
                doc: "".try_into().unwrap(),
                name: "Tuple".try_into().unwrap(),
                type_: [tuple_type()].try_into().unwrap(),
            })]
            .try_into()
            .unwrap(),
        }),
    );
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let (s, t, e) = values();
    assert_eq!(client.echo(&s, &t, &e), (s, t, e));
}
