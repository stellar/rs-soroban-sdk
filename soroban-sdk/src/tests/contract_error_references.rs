//! References to types named `Error` resolve to the type the name refers to:
//! `soroban_sdk::Error` has the built-in error spec type, and a user-defined
//! `Error` is referred to by its own qualified name, like any other
//! user-defined type.

use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contracterror, contractimpl};
use stellar_xdr::{Limits, ReadXdr, ScSpecEntry, ScSpecTypeDef, ScSpecTypeResult, ScSpecTypeUdt};

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

fn error_type(xdr: &[u8]) -> ScSpecTypeDef {
    match ScSpecEntry::from_xdr(xdr, Limits::none()).unwrap() {
        ScSpecEntry::FunctionV0(f) => match &f.outputs[..] {
            [ScSpecTypeDef::Result(r)] => {
                let ScSpecTypeResult { error_type, .. } = r.as_ref();
                error_type.as_ref().clone()
            }
            o => panic!("unexpected outputs {o:?}"),
        },
        e => panic!("unexpected entry {e:?}"),
    }
}

fn udt(name: &str) -> ScSpecTypeDef {
    ScSpecTypeDef::Udt(ScSpecTypeUdt {
        name: name.try_into().unwrap(),
    })
}

#[test]
fn test_sdk_error_is_the_builtin_error_type() {
    assert_eq!(error_type(&Contract::spec_xdr_sdk()), ScSpecTypeDef::Error);
}

#[test]
fn test_user_defined_errors_are_referred_to_by_their_own_names() {
    assert_eq!(
        error_type(&Contract::spec_xdr_local()),
        udt("::soroban_sdk::tests::contract_error_references::Error")
    );
    assert_eq!(
        error_type(&Contract::spec_xdr_a()),
        udt("::soroban_sdk::tests::contract_error_references::a::Error")
    );
    assert_eq!(
        error_type(&Contract::spec_xdr_b()),
        udt("::soroban_sdk::tests::contract_error_references::b::Error")
    );
}
