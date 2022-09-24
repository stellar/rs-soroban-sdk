use crate::BytesN;

/// TODO: Make Invoker storable and convertible to RawVal.

/// Invoker is the invoker of a contract.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Invoker {
    Account(BytesN<32>),
    Contract(BytesN<32>),
}
