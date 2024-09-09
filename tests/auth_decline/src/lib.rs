#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Val};

#[contract]
pub struct Contract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Decline = 1,
}

#[contractimpl]
impl Contract {
    #[allow(non_snake_case)]
    pub fn __check_auth(
        _signature_payload: Val,
        _signatures: Val,
        _auth_context: Val,
    ) -> Result<(), Error> {
        Err(Error::Decline)
    }
}
