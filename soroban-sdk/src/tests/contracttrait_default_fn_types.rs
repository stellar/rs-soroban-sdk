use crate as soroban_sdk;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    xdr::{Limits, ReadXdr, ScSpecEntry, ScSpecTypeDef},
    Env,
};

// The trait and the types its default fn names are defined in a module, and the
// trait is implemented in a scope that has unrelated types of the same names.
// The client, args, and spec generated for the default fn must use the types as
// the trait names them, not the types the names resolve to where it is
// implemented.
mod inner {
    use crate as soroban_sdk;
    use soroban_sdk::{contracterror, contracttrait, contracttype, Env};

    #[contracttype]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Value {
        A = 1,
        B = 2,
    }

    #[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Error {
        Failed = 1,
    }

    #[contracttrait]
    pub trait Trait {
        fn f(env: Env, v: Value) -> Result<Value, Error> {
            let _ = env;
            match v {
                Value::A => Ok(Value::B),
                Value::B => Err(Error::Failed),
            }
        }
    }
}
use inner::Trait;

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Value {
    C = 3,
    D = 4,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Other = 2,
}

#[contract]
pub struct Contract;

#[contractimpl(contracttrait)]
impl Trait for Contract {}

#[test]
fn test_client_uses_trait_types() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    assert_eq!(client.f(&inner::Value::A), inner::Value::B);
    assert_eq!(
        client.try_f(&inner::Value::B),
        Err(Ok(inner::Error::Failed))
    );
}

#[test]
fn test_args_use_trait_types() {
    let (v,): (&inner::Value,) = ContractArgs::f(&inner::Value::A);
    assert_eq!(v, &inner::Value::A);
}

#[test]
fn test_spec_uses_trait_types() {
    let entry = ScSpecEntry::from_xdr(Contract::spec_xdr_f(), Limits::none()).unwrap();
    let ScSpecEntry::FunctionV0(f) = entry else {
        panic!("expected function spec entry");
    };
    let udt_name = |t: &ScSpecTypeDef| match t {
        ScSpecTypeDef::Udt(u) => u.name.to_utf8_string_lossy(),
        t => panic!("expected udt, got {t:?}"),
    };
    let ScSpecTypeDef::Result(r) = &f.outputs[0] else {
        panic!("expected result output");
    };
    assert!(udt_name(&f.inputs[0].type_).ends_with("::inner::Value"));
    assert!(udt_name(&r.ok_type).ends_with("::inner::Value"));
    assert!(udt_name(&r.error_type).ends_with("::inner::Error"));
}
