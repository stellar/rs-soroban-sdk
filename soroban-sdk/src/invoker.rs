use crate::{contracttype, AccountId, BytesN};

/// Invoker is the invoker of a contract.
// The Invoker type is a contracttype and transmitted to the host like an enum,
// however it is not exported into contract specs because it has its own spec
// definition type dedicated to it.
#[contracttype(crate_path = "crate", lib = "soroban_sdk", export = false)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Invoker {
    Account(AccountId),
    Contract(BytesN<32>),
}
