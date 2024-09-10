#![no_std]
use soroban_sdk::{contract, contractimpl, Val};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    #[allow(non_snake_case)]
    pub fn __check_auth(_signature_payload: Val, _signatures: Val, _auth_context: Val) {}
}
