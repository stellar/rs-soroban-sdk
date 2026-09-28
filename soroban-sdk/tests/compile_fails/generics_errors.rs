use soroban_sdk::{contract, contractevent, contractimpl, contracttrait, Address, Env, IntoVal, Val};

#[contract]
pub struct C;

// The spec has no way to represent a type or const parameter, so only lifetimes
// are accepted on events, contract functions, and contract traits.

#[contractevent]
pub struct WithType<'a, T: IntoVal<Env, Val>> {
    #[topic]
    from: &'a Address,
    value: T,
}

#[contractevent]
pub struct WithConst<const N: u32> {
    #[topic]
    from: Address,
}

#[contractimpl]
impl C {
    pub fn with_type<T: soroban_sdk::TryFromVal<Env, Val>>(_env: Env, _x: T) {}
}

#[contracttrait]
pub trait Tr {
    fn with_const<const N: u32>(_env: Env) {}
}

fn main() {}
