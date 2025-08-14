use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub struct Events {
    env: Env,
}

#[allow(deprecated)]
impl Events {
    #[inline(always)]
    pub fn new(env: &Env) -> Events {
        Events { env: env.clone() }
    }

    #[deprecated = "use soroban_token_sdk::events::Approve::publish"]
    pub fn approve(&self, from: Address, to: Address, amount: i128, expiration_ledger: u32) {
        let topics = (Symbol::new(&self.env, "approve"), from, to);
        self.env
            .events()
            .publish(topics, (amount, expiration_ledger));
    }

    #[deprecated = "use soroban_token_sdk::events::Transfer::publish"]
    pub fn transfer(&self, from: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("transfer"), from, to);
        self.env.events().publish(topics, amount);
    }

    #[deprecated = "use soroban_token_sdk::events::Mint::publish"]
    pub fn mint(&self, admin: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("mint"), admin, to);
        self.env.events().publish(topics, amount);
    }

    #[deprecated = "use soroban_token_sdk::events::Clawback::publish"]
    pub fn clawback(&self, admin: Address, from: Address, amount: i128) {
        let topics = (symbol_short!("clawback"), admin, from);
        self.env.events().publish(topics, amount);
    }

    #[deprecated = "define a contractevent instead:\n\
        #[contractevent(data_format = \"single-value\")]\n\
        pub struct SetAuthorized {\n\
            #[topic]\n\
            admin: Address,\n\
            #[topic]\n\
            id: Address,\n\
            authorize: bool,\n\
        }"]
    pub fn set_authorized(&self, admin: Address, id: Address, authorize: bool) {
        let topics = (Symbol::new(&self.env, "set_authorized"), admin, id);
        self.env.events().publish(topics, authorize);
    }

    #[deprecated = "define a contractevent instead:\n\
        #[contractevent(data_format = \"single-value\")]\n\
        pub struct SetAdmin {\n\
            #[topic]\n\
            admin: Address,\n\
            new_admin: Address,\n\
        }"]
    pub fn set_admin(&self, admin: Address, new_admin: Address) {
        let topics = (symbol_short!("set_admin"), admin);
        self.env.events().publish(topics, new_admin);
    }

    #[deprecated = "use soroban_token_sdk::events::Burn::publish"]
    pub fn burn(&self, from: Address, amount: i128) {
        let topics = (symbol_short!("burn"), from);
        self.env.events().publish(topics, amount);
    }
}
