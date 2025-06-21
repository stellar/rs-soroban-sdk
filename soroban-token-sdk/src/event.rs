use soroban_sdk::{symbol_short, Address, Env, EnvBase, IntoVal, MuxedAddress, Symbol};

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

    pub fn transfer_with_muxed_address(&self, from: Address, to: MuxedAddress, amount: i128) {
        let to_muxed_id = to.id();
        let topics = (symbol_short!("transfer"), from, to);
        let amount_val = amount.into_val(&self.env);
        let data = match to_muxed_id {
            None => amount_val,
            Some(to_muxed_id) => self
                .env
                .map_new_from_slices(
                    &["amount", "to_muxed_id"],
                    &[amount_val, to_muxed_id.into_val(&self.env)],
                )
                .unwrap()
                .into(),
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
