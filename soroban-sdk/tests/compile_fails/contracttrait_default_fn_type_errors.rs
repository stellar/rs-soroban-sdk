use soroban_sdk::{contract, contracterror, contractimpl, contracttype};

// The trait's default fns name types defined in its module, and the trait is
// implemented in a scope where the same names are unrelated types. The client,
// args, and spec generated at the impl for the default fns would use the
// unrelated types, so the impl must fail to compile.
mod inner {
    use soroban_sdk::{contracterror, contracttrait, contracttype, Env};

    #[contracttype]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Value {
        A = 1,
    }

    #[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Error {
        Failed = 1,
    }

    #[contracttrait]
    pub trait Trait {
        fn f(env: Env, v: Value) -> Value {
            let _ = env;
            v
        }

        fn g(env: &Env) -> Result<u32, Error> {
            let _ = env;
            Ok(1)
        }
    }
}
use inner::Trait;

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Value {
    C = 3,
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

fn main() {}
