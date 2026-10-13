use soroban_sdk::{contracttype, BytesN};

// The spec holds a BytesN's length as a u32, so a BytesN longer than that,
// reached through an alias the macros cannot see through, cannot be in a spec.
type Huge = BytesN<{ u32::MAX as usize + 1 }>;

#[contracttype]
pub struct HoldsHuge {
    pub val: Huge,
}

fn main() {}
