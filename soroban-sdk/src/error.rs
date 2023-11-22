use core::convert::Infallible;

use crate::xdr;

/// InvokeError captures errors returned from the invocation of another
/// contract.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InvokeError {
    /// Abort occurs if the invoked contract function exited due to some
    /// irrecoverable error, such as exceeding budget, or out-of-bounds access.
    Abort,
    /// Contract error occurs if the invoked contract function exited due to
    /// some error that the contract has indicated with a code. May be mappable
    /// to an enum type if the contract has defined a
    /// [`contracterror`][crate::contracterror].
    Contract(u32),
}

impl From<crate::Error> for InvokeError {
    fn from(e: crate::Error) -> Self {
        if e.is_type(xdr::ScErrorType::Contract) {
            InvokeError::Contract(e.get_code())
        } else {
            InvokeError::Abort
        }
    }
}

impl From<Infallible> for InvokeError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
