use soroban_sdk::contractimpl;

use crate::Contract;
use crate::ContractClient;

#[contractimpl]
impl Contract {
    pub fn two() -> u32 {
        2
    }
}
