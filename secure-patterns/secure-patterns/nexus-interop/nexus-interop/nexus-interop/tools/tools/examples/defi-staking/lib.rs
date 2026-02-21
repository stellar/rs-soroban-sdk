use soroban_sdk::{contract, contractimpl, Env, Address};

#[contract]
pub struct StakingContract;

#[contractimpl]
impl StakingContract {
    pub fn stake(e: Env, user: Address, amount: i128) {
        user.require_auth();
        e.events().publish(("stake", user), amount);
    }
}
