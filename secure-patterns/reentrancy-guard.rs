use soroban_sdk::{contracttype, Env};

#[contracttype]
#[derive(Clone)]
pub struct ReentrancyGuard {
    locked: bool,
}

impl ReentrancyGuard {
    pub fn new() -> Self {
        Self { locked: false }
    }

    pub fn enter(&mut self) {
        if self.locked {
            panic!("Reentrancy detected");
        }
        self.locked = true;
    }

    pub fn exit(&mut self) {
        self.locked = false;
    }
}
