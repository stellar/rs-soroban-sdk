use crate::{self as soroban_sdk};
use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Test {
    Variant(Option<Address>, Address, i128),
}
