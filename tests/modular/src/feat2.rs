use soroban_sdk::contractimpl;

use crate::ContractArgs;
use crate::ContractClient;

#[contractimpl]
impl super::Contract {
    pub fn two() -> u32 {
        2
    }
}
