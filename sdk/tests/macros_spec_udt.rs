#![cfg(feature = "testutils")]

use std::io::Cursor;

use stellar_contract_sdk::{
    contractimpl, ConversionError, Env, EnvVal, IntoEnvVal, IntoVal, RawVal, TryFromVal,
};
use stellar_xdr::{
    ReadXdr, ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeTuple, ScSpecTypeUdt,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Udt {
    a: i32,
    b: i32,
}

impl TryFrom<EnvVal> for Udt {
    type Error = ConversionError;

    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let (a, b): (i32, i32) = ev.try_into()?;
        Ok(Udt { a, b })
    }
}

impl IntoEnvVal<Env, RawVal> for Udt {
    fn into_env_val(self, env: &Env) -> EnvVal {
        (self.a, self.b).into_env_val(env)
    }
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Udt, b: Udt) -> (Udt, Udt) {
        (a, b)
    }
}

#[test]
fn test_functional() {
    let e = Env::default();
    let a = Udt { a: 5, b: 7 };
    let b = Udt { a: 10, b: 14 };
    let c = __add::call_raw(e.clone(), a.into_val(&e), b.into_val(&e));
    let c = <(Udt, Udt)>::try_from_val(&e, c).unwrap();
    assert_eq!(c, (a, b));
}

#[test]
fn test_spec() {
    let entries = ScSpecEntry::read_xdr(&mut Cursor::new(&__SPEC_XDR_ADD)).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        name: "add".try_into().unwrap(),
        input_types: vec![
            ScSpecTypeDef::Udt(ScSpecTypeUdt {
                name: "Udt".try_into().unwrap(),
            }),
            ScSpecTypeDef::Udt(ScSpecTypeUdt {
                name: "Udt".try_into().unwrap(),
            }),
        ]
        .try_into()
        .unwrap(),
        output_types: vec![ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
            value_types: vec![
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            ]
            .try_into()
            .unwrap(),
        }))]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entries, expect);
}
