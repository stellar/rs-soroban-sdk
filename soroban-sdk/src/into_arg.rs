//! IntoArg is the trait for the values a generated contract client accepts for
//! the arguments of contract functions.

use core::borrow::Borrow;

use crate::{client::Type, Address, Env, IntoVal, MuxedAddress, Val};

/// Trait for the values a client function accepts for an argument of type `T`.
///
/// Any value that borrows as `T` is accepted, so that an argument can be passed
/// by value or by reference. Types that other types convert into accept those
/// types too, such as a [`MuxedAddress`] argument accepting an [`Address`].
pub trait IntoArg<T> {
    fn into_arg(self, env: &Env) -> Val;
}

impl<T, R> IntoArg<T> for R
where
    T: Type + IntoVal<Env, Val>,
    R: Borrow<T>,
{
    fn into_arg(self, env: &Env) -> Val {
        let t: &T = self.borrow();
        t.into_val(env)
    }
}

impl IntoArg<MuxedAddress> for Address {
    fn into_arg(self, env: &Env) -> Val {
        MuxedAddress::from(self).into_val(env)
    }
}

impl IntoArg<MuxedAddress> for &Address {
    fn into_arg(self, env: &Env) -> Val {
        MuxedAddress::from(self).into_val(env)
    }
}
