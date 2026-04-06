#![no_std]
use soroban_sdk::{contracterror, contractevent, contracttype, Address, Vec};

// Structs (named fields) - A, B, C

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructA {
    pub f1: u32,
    pub f2: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructB {
    pub f1: i64,
    pub f2: soroban_sdk::String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructC {
    pub f1: Vec<u32>,
    pub f2: Address,
}

// Struct tuples (newtypes) - A, B, C

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructTupleA(pub i64, pub i64);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructTupleB(pub u128, pub u128);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructTupleC(pub Address, pub i128);

// Enums / Unions (with variants) - A, B, C

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnumA {
    V1,
    V2,
    V3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnumB {
    V1,
    V2(i64),
    V3(i64, i64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnumC {
    V1,
    V2(StructA),
    V3(StructTupleA),
}

// Enum Int (integer discriminants) - A, B, C

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EnumIntA {
    V1 = 1,
    V2 = 2,
    V3 = 3,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EnumIntB {
    V1 = 10,
    V2 = 20,
    V3 = 30,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EnumIntC {
    V1 = 100,
    V2 = 200,
    V3 = 300,
}

// Error Enums - A, B, C

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorA {
    E1 = 1,
    E2 = 2,
    E3 = 3,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorB {
    E1 = 10,
    E2 = 11,
    E3 = 12,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorC {
    E1 = 100,
    E2 = 101,
    E3 = 102,
}

// Events - A, B, C

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventA {
    #[topic]
    pub f1: Address,
    pub f2: soroban_sdk::String,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventB {
    #[topic]
    pub f1: Address,
    #[topic]
    pub f2: Address,
    pub f3: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventC {
    #[topic]
    pub f1: soroban_sdk::Symbol,
    pub f2: i64,
    pub f3: i64,
}
