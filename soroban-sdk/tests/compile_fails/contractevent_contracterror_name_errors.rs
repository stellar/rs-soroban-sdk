use soroban_sdk::{contracterror, contractevent};

// Events and errors are defined by name in the spec, as user-defined types are,
// so they cannot take the name of a soroban_sdk type.

#[contractevent]
pub struct Address {
    pub amount: u32,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Symbol {
    A = 1,
}

// An event can borrow its fields with a lifetime, but cannot be generic over a
// type, as the spec has no generics.

#[contractevent]
pub struct Generic<T> {
    pub amount: T,
}

fn main() {}
