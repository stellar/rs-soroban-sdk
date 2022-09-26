use crate::{contracttype, BytesN};

/// Invoker is the invoker of a contract.
#[contracttype(crate_path = "crate", lib = "soroban_sdk", export = false)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Invoker {
    Account(BytesN<32>),
    Contract(BytesN<32>),
}
