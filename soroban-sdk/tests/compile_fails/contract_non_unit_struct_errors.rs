use soroban_sdk::contract;

#[contract]
pub struct ContractWithFields {
    a: u32,
}

#[contract]
pub struct ContractTuple(u32);

#[contract]
pub struct ContractEmptyTuple();

#[contract]
pub struct ContractWithGenerics<T>(T);

fn main() {}
