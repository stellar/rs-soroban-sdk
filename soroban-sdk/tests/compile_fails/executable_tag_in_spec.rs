// ExecutableTag has no contract spec type, so using it in places where
// contract spec types are required (like contract interfaces) is not allowed.

use soroban_sdk::{contract, contractimpl, contracttype, Env, ExecutableTag};

#[contract]
pub struct Contract;

#[contracttype]
pub struct MyType {
    pub tag: ExecutableTag,
}

#[contractimpl]
impl Contract {
    pub fn takes_tag(_env: Env, _tag: ExecutableTag) {}
}

fn main() {}
