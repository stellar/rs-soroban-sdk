use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, Env};

// ---------------------------------------------------------------------------
// Test: when #[contractimpl] is applied to `impl Trait for Type`, the
// generated WASM export must call the *trait* associated function, not an
// inherent associated function that happens to share the same name.
//
// Rust's name-resolution rules say that `<Type>::func()` resolves to the
// inherent associated function when one exists, even if a trait with the
// same function is in scope.  The only way to force the trait version is
// Universal Function Call Syntax: `<Type as Trait>::func()`.
//
// This test creates a contract where both the inherent and trait versions
// of `value` return a different u32.  We then check which value was
// returned to determine which function was called.
// ---------------------------------------------------------------------------

#[contract]
pub struct Contract;

/// Inherent function — returns 1.
impl Contract {
    pub fn value(_env: Env) -> u32 {
        1
    }
}

pub trait ContractTrait {
    fn value(env: Env) -> u32;
}

/// Trait implementation — returns 2.
/// The macro-generated WASM export for "value" MUST call this version.
#[contractimpl]
impl ContractTrait for Contract {
    fn value(_env: Env) -> u32 {
        2
    }
}

/// The exported `value` entry point must call the trait function, which
/// returns 2.
#[test]
fn calls_trait_fn() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    let result = client.value();

    assert_eq!(result, 2);
}
