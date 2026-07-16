#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // A constructor that requires authorization from the provided address.
    // Registering this contract exercises the recording authorization mode that
    // both `register` and `register_at` use when invoking the constructor.
    pub fn __constructor(_env: Env, admin: Address) {
        admin.require_auth();
    }
}
