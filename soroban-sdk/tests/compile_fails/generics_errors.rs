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
    fn plain(_env: Env) {}
}

// Implementing a trait whose `#[contracttrait]` expansion was rejected should
// surface only the trait's own error, not a `cannot find macro`/`cannot find
// trait` cascade from a dropped trait declaration.
#[contract]
pub struct D;

#[contractimpl(contracttrait)]
impl Tr for D {}

fn main() {}
