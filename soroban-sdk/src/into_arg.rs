//! IntoArg is the trait for the values a generated contract client accepts for
//! the arguments of contract functions.

use core::borrow::Borrow;

use crate::{
    client::{Borrowed, Converted, Type},
    Env, IntoVal, Val,
};

/// Trait for the values a client function accepts for an argument of type `T`,
/// which are decided by the type's [`Type::ArgMethod`].
///
/// `Method` defaults to the type's [`Type::ArgMethod`] and is only a parameter
/// so that the impls for each method do not overlap, as the compiler does not
/// use an associated type to tell impls apart.
pub trait IntoArg<T: Type, Method = <T as Type>::ArgMethod> {
    fn into_arg(self, env: &Env) -> Val;
}

// Any reference that borrows as the type is accepted, so that a reference to a
// reference is accepted as it would be by deref coercion.
impl<'a, T, R> IntoArg<T, Borrowed> for &'a R
where
    T: Type + IntoVal<Env, Val>,
    R: ?Sized + Borrow<T>,
{
    fn into_arg(self, env: &Env) -> Val {
        let t: &T = self.borrow();
        t.into_val(env)
    }
}

impl<T, X> IntoArg<T, Converted> for X
where
    T: Type + IntoVal<Env, Val>,
    X: Into<T>,
{
    fn into_arg(self, env: &Env) -> Val {
        let t: T = self.into();
        t.into_val(env)
    }
}
