use soroban_sdk::{contracttype, Bytes, BytesN};

#[contracttype]
#[derive(Clone)]
pub struct NexusMessage {
    pub src_chain: Bytes,
    pub dst_chain: Bytes,
    pub sender: BytesN<32>,
    pub payload: Bytes,
}
