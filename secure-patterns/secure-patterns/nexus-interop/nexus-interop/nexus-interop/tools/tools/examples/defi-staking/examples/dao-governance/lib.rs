use soroban_sdk::{contract, contractimpl, Env, Address, Symbol};

#[contract]
pub struct DaoContract;

#[contractimpl]
impl DaoContract {
    pub fn vote(e: Env, voter: Address, proposal: Symbol, support: bool) {
        voter.require_auth();
        e.events().publish((proposal, voter), support);
    }
}
