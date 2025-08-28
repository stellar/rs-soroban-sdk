use soroban_sdk::{contractevent, Address};

#[contractevent(topics = ["approve"], data_format = "vec")]
pub struct Approve {
    #[topic]
    pub from: Address,
    #[topic]
    pub spender: Address,
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contractevent(topics = ["transfer"], data_format = "single-value")]
pub struct TransferWithAmountOnly {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub amount: i128,
}

#[contractevent(topics = ["transfer"], data_format = "map")]
pub struct Transfer {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub to_muxed_id: Option<u64>,
    pub amount: i128,
}

#[contractevent(topics = ["burn"], data_format = "single-value")]
pub struct Burn {
    #[topic]
    pub from: Address,
    pub amount: i128,
}

#[contractevent(topics = ["mint"], data_format = "single-value")]
pub struct MintWithAmountOnly {
    #[topic]
    pub to: Address,
    pub amount: i128,
}

#[contractevent(topics = ["mint"], data_format = "map")]
pub struct Mint {
    #[topic]
    pub to: Address,
    pub to_muxed_id: Option<u64>,
    pub amount: i128,
}

#[contractevent(topics = ["clawback"], data_format = "single-value")]
pub struct Clawback {
    #[topic]
    pub from: Address,
    pub amount: i128,
}
