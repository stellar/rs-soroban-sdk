use soroban_sdk::{contracttype, symbol_short, token::muxed_ext::Mux, Address, Env, Symbol};

pub struct Events {
    env: Env,
}

impl Events {
    #[inline(always)]
    pub fn new(env: &Env) -> Events {
        Events { env: env.clone() }
    }

    pub fn approve(&self, from: Address, to: Address, amount: i128, expiration_ledger: u32) {
        let topics = (Symbol::new(&self.env, "approve"), from, to);
        self.env
            .events()
            .publish(topics, (amount, expiration_ledger));
    }

    pub fn transfer(&self, from: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("transfer"), from, to);
        self.env.events().publish(topics, amount);
    }

    pub fn transfer_muxed(
        &self,
        from: Address,
        from_mux: Mux,
        to: Address,
        to_mux: Mux,
        amount: i128,
    ) {
        let topics = (Symbol::new(&self.env, "transfer_muxed"), from, to);
        let data = TransferMuxedData {
            amount,
            from_mux,
            to_mux,
        };
        self.env.events().publish(topics, data);
    }

    pub fn mint(&self, admin: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("mint"), admin, to);
        self.env.events().publish(topics, amount);
    }

    pub fn clawback(&self, admin: Address, from: Address, amount: i128) {
        let topics = (symbol_short!("clawback"), admin, from);
        self.env.events().publish(topics, amount);
    }

    pub fn set_authorized(&self, admin: Address, id: Address, authorize: bool) {
        let topics = (Symbol::new(&self.env, "set_authorized"), admin, id);
        self.env.events().publish(topics, authorize);
    }

    pub fn set_admin(&self, admin: Address, new_admin: Address) {
        let topics = (symbol_short!("set_admin"), admin);
        self.env.events().publish(topics, new_admin);
    }

    pub fn burn(&self, from: Address, amount: i128) {
        let topics = (symbol_short!("burn"), from);
        self.env.events().publish(topics, amount);
    }
}

#[contracttype]
struct TransferMuxedData {
    amount: i128,
    from_mux: Mux,
    to_mux: Mux,
}
