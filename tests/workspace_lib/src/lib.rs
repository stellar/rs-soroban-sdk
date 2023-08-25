#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
#[derive(Debug, Eq, PartialEq)]
pub struct Value {
    pub value: i32,
}
