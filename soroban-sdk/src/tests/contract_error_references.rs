//! References to types named `Error` resolve to the type the name refers to:
//! `soroban_sdk::Error` has the built-in error spec type, and a user-defined
//! `Error` is referred to by its own qualified name, like any other
//! user-defined type.

use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contracterror, contractimpl};
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult, ScSpecTypeUdt,
};

pub mod a {
    use crate::{self as soroban_sdk};
    use soroban_sdk::contracterror;

    #[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Error {
        A = 1,
    }
}

pub mod b {
    use crate::{self as soroban_sdk};
    use soroban_sdk::contracterror;

    #[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Error {
        B = 2,
    }
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    C = 3,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn sdk() -> Result<(), soroban_sdk::Error> {
        Ok(())
    }

    pub fn local() -> Result<(), Error> {
        Ok(())
    }

    pub fn a() -> Result<(), a::Error> {
        Ok(())
    }

    pub fn b() -> Result<(), b::Error> {
        Ok(())
    }
}

fn fn_returning_result_of(name: &str, error_type: ScSpecTypeDef) -> ScSpecEntry {
    ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: name.try_into().unwrap(),
        inputs: vec![].try_into().unwrap(),
        outputs: vec![ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
            ok_type: Box::new(ScSpecTypeDef::Void),
            error_type: Box::new(error_type),
        }))]
        .try_into()
        .unwrap(),
    })
}

fn udt(name: &str) -> ScSpecTypeDef {
    ScSpecTypeDef::Udt(ScSpecTypeUdt {
        name: name.try_into().unwrap(),
    })
}

#[test]
fn test_sdk_error_is_the_builtin_error_type() {
    let entry = ScSpecEntry::from_xdr(Contract::spec_xdr_sdk(), Limits::none()).unwrap();
    assert_eq!(entry, fn_returning_result_of("sdk", ScSpecTypeDef::Error));
}

#[test]
fn test_user_defined_errors_are_referred_to_by_their_own_names() {
    let entry = ScSpecEntry::from_xdr(Contract::spec_xdr_local(), Limits::none()).unwrap();
    assert_eq!(
        entry,
        fn_returning_result_of(
            "local",
            udt("::soroban_sdk::tests::contract_error_references::Error")
        )
    );

    let entry = ScSpecEntry::from_xdr(Contract::spec_xdr_a(), Limits::none()).unwrap();
    assert_eq!(
        entry,
        fn_returning_result_of(
            "a",
            udt("::soroban_sdk::tests::contract_error_references::a::Error")
        )
    );

    let entry = ScSpecEntry::from_xdr(Contract::spec_xdr_b(), Limits::none()).unwrap();
    assert_eq!(
        entry,
        fn_returning_result_of(
            "b",
            udt("::soroban_sdk::tests::contract_error_references::b::Error")
        )
    );
}
