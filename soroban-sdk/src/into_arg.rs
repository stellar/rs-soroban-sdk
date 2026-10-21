//! IntoArg is the trait for the values a generated contract client accepts for
//! the arguments of contract functions.

use core::borrow::Borrow;

use crate::{client::Type, Address, MuxedAddress};

/// Trait for the values a client function accepts for an argument of type `T`.
///
/// Any value that borrows as `T` is accepted, so that an argument can be passed
/// by value or by reference. Types that other types convert into accept those
/// types too, such as a [`MuxedAddress`] argument accepting an [`Address`].
pub trait IntoArg<T> {
    type Arg: Borrow<T>;
    fn into_arg(self) -> Self::Arg;
}

impl<T, R> IntoArg<T> for R
where
    T: Type,
    R: Borrow<T>,
{
    type Arg = R;
    fn into_arg(self) -> R {
        self
    }
}

#[diagnostic::do_not_recommend]
impl IntoArg<MuxedAddress> for Address {
    type Arg = MuxedAddress;
    fn into_arg(self) -> MuxedAddress {
        self.into()
    }
}

#[diagnostic::do_not_recommend]
impl IntoArg<MuxedAddress> for &Address {
    type Arg = MuxedAddress;
    fn into_arg(self) -> MuxedAddress {
        self.into()
    }
}
