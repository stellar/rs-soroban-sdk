use soroban_sdk::{contracttype, Env};

#[contracttype]
#[derive(Clone)]
pub struct PauseState {
    pub paused: bool,
}

impl PauseState {
    pub fn require_not_paused(&self) {
        if self.paused {
            panic!("Contract is paused");
        }
    }
}
