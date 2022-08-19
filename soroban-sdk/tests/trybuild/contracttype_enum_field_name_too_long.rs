use soroban_sdk::contracttype;

#[contracttype]
pub enum MyType {
    Variant1,
    Variant2TooLong,
}

pub fn main() {}
