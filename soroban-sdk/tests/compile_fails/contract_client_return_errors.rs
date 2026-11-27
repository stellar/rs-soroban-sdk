use soroban_sdk::{
    contract, contractimpl, xdr::r#const::ScSpecTypeDef, ConversionError, Env, SpecTypeDef,
    TryFromVal, Val,
};

// A type that can't be used as a contract function return value, because the
// contract's client can't convert the value it receives into it. The type has a
// spec type and converts into a Val, so that only the client's error is
// reported.

pub struct NotAReturnType;

impl TryFromVal<Env, NotAReturnType> for Val {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, _v: &NotAReturnType) -> Result<Self, Self::Error> {
        Ok(().into())
    }
}

impl SpecTypeDef for NotAReturnType {
    const SPEC_TYPE_DEF: ScSpecTypeDef = ScSpecTypeDef::Void;
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn f() -> NotAReturnType {
        NotAReturnType
    }
}

fn main() {}
