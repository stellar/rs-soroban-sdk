use core::convert::Infallible;

use crate::xdr;

/// InvokeError captures errors returned from the invocation of another
/// contract.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InvokeError {
    /// Abort occurs if the invoke contract panicks with a [`panic!`], or a host
    /// function of the environment has a failure, or a runtime error occurs.
    Abort,
    /// Contract error occurs if the invoked contract function exited returning
    /// an error or called [`panic_with_error!`][crate::panic_with_error!] with
    /// a [`contracterror`][crate::contracterror].
    ///
    /// If the contract defines a [`contracterror`][crate::contracterror] type
    /// as part of its interface, this variant of the error will be convertible
    /// to that type, but if that conversion failed then this variant of the
    /// error would be used to represent the error.
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
