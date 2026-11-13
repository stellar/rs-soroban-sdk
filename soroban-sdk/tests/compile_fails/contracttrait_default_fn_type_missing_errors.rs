use soroban_sdk::{contract, contractimpl};

// The trait's default fn names a type defined in its module, and the trait is
// implemented in a scope that does not import the type.
mod inner {
    use soroban_sdk::{contracttrait, contracttype, Env};

    #[contracttype]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Value {
        A = 1,
    }

    #[contracttrait]
    pub trait Trait {
        fn f(env: Env, v: Value) -> Value {
            let _ = env;
            v
        }
    }
}
use inner::Trait;

#[contract]
pub struct Contract;

#[contractimpl(contracttrait)]
impl Trait for Contract {}

fn main() {}
