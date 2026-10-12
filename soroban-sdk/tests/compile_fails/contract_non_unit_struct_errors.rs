use core::marker::PhantomData;
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct ContractWithFields {
    a: u32,
}

#[contract]
pub struct ContractTuple(u32);

#[contract]
pub struct ContractEmptyTuple();

#[contract]
pub struct ContractWithGenerics<T> {
    _t: PhantomData<T>,
}

#[contractimpl]
impl ContractWithGenerics<u32> {
    pub fn f() {}
}

#[contractimpl]
impl<T> ContractWithGenerics<T> {
    pub fn g() {}
}

#[contract]
pub struct ContractWithLifetime<'a> {
    _a: PhantomData<&'a ()>,
}

#[contract]
pub struct ContractWithWhereClause
where
    (): Sized, {}

#[contract]
pub enum ContractEnum {}

#[contract]
pub union ContractUnion {
    a: u32,
}

fn main() {}
