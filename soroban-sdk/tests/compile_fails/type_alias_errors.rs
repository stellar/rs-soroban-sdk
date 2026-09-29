use soroban_sdk::{contract, contractevent, contractimpl, contracttype, Address};

// The macros only see an alias's name, not the type it aliases, so an alias of
// a primitive or SDK type is not supported and is reported where it is used.
pub type Amount = i128;
pub type Owner = Address;

#[contracttype]
pub struct Holding {
    pub owner: Owner,
    pub amount: Amount,
}

#[contracttype]
pub enum Action {
    Send(Amount),
}

#[contractevent]
pub struct Sent {
    pub amount: Amount,
}

// An alias of a contract type is supported.
pub type Held = Holding;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Amount, b: Amount) -> Amount {
        a + b
    }

    pub fn held(h: Held) -> Held {
        h
    }
}

fn main() {}
