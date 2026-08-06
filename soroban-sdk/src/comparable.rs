use core::{cmp::Ordering, fmt::Debug};

use super::{env::internal::Compare, ConversionError, Env, IntoVal, TryFromVal, Val};

use crate::unwrap::UnwrapInfallible;

#[cfg(not(target_family = "wasm"))]
use super::xdr::ScVal;

/// Comparable wraps a value so that it is comparable, by comparing it in the
/// Host.
///
/// Some values, such as [Val], are not comparable in the Guest because their
/// contents are stored in the Host. Wrapping such a value in a [Comparable]
/// gives it [PartialEq], [Eq], [PartialOrd], and [Ord] implementations that ask
/// the Host to do the comparison, and so a wrapped value can be used in
/// contract types that derive those traits.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{contracttype, Comparable, Env, IntoVal, Val};
///
/// #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
/// #[contracttype]
/// pub struct Message {
///     pub id: u32,
///     pub payload: Comparable<Val>,
/// }
///
/// let env = Env::default();
///
/// let m1 = Message { id: 1, payload: Comparable::new(&env, 8u32.into_val(&env)) };
/// let m2 = Message { id: 1, payload: Comparable::new(&env, 8u32.into_val(&env)) };
/// assert_eq!(m1, m2);
///
/// let m3 = Message { id: 1, payload: Comparable::new(&env, 9u32.into_val(&env)) };
/// assert!(m1 < m3);
/// ```
pub struct Comparable<T> {
    env: Env,
    val: T,
}

impl<T> Clone for Comparable<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            env: self.env.clone(),
            val: self.val.clone(),
        }
    }
}

impl<T> Debug for Comparable<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Comparable({:?})", self.val)
    }
}

impl<T> Eq for Comparable<T> where T: IntoVal<Env, Val> {}

impl<T> PartialEq for Comparable<T>
where
    T: IntoVal<Env, Val>,
{
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<T> PartialOrd for Comparable<T>
where
    T: IntoVal<Env, Val>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<T> Ord for Comparable<T>
where
    T: IntoVal<Env, Val>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let self_val = self.to_val();
        let other_val = other.to_val();
        #[cfg(not(target_family = "wasm"))]
        if !self.env.is_same_env(&other.env) {
            return ScVal::try_from_val(&self.env, &self_val)
                .unwrap()
                .cmp(&ScVal::try_from_val(&other.env, &other_val).unwrap());
        }
        // Compare defers to the Host for values stored in the Host, and does
        // the comparison in the Guest for values that are small enough to be
        // stored in the value itself.
        self.env.compare(&self_val, &other_val).unwrap_infallible()
    }
}

impl<T> TryFromVal<Env, Val> for Comparable<T>
where
    T: TryFromVal<Env, Val>,
{
    type Error = T::Error;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        Ok(Comparable {
            env: env.clone(),
            val: T::try_from_val(env, val)?,
        })
    }
}

impl<T> TryFromVal<Env, Comparable<T>> for Val
where
    T: IntoVal<Env, Val>,
{
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Comparable<T>) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl<T> TryFromVal<Env, &Comparable<T>> for Val
where
    T: IntoVal<Env, Val>,
{
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&Comparable<T>) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl<T> From<&Comparable<T>> for Comparable<T>
where
    T: Clone,
{
    #[inline(always)]
    fn from(v: &Comparable<T>) -> Self {
        v.clone()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> From<&Comparable<T>> for ScVal
where
    T: IntoVal<Env, Val>,
{
    fn from(v: &Comparable<T>) -> Self {
        // This conversion occurs only in test utilities, and theoretically all
        // values should convert to an ScVal because the Env won't let the host
        // type to exist otherwise, unwrapping. Even if there are edge cases
        // that don't, this is a trade off for a better test developer
        // experience.
        ScVal::try_from_val(&v.env, &v.to_val()).unwrap()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> From<Comparable<T>> for ScVal
where
    T: IntoVal<Env, Val>,
{
    fn from(v: Comparable<T>) -> Self {
        (&v).into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFromVal<Env, ScVal> for Comparable<T>
where
    T: TryFromVal<Env, Val>,
{
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        let val = Val::try_from_val(env, val)?;
        <Comparable<T> as TryFromVal<Env, Val>>::try_from_val(env, &val)
            .map_err(|_| ConversionError)
    }
}

impl<T> Comparable<T> {
    /// Wrap the value, making it comparable in the environment.
    pub fn new(env: &Env, val: T) -> Self {
        Self {
            env: env.clone(),
            val,
        }
    }

    #[inline(always)]
    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Returns a reference to the wrapped value.
    pub fn as_inner(&self) -> &T {
        &self.val
    }

    /// Returns the wrapped value.
    pub fn into_inner(self) -> T {
        self.val
    }
}

impl<T> Comparable<T>
where
    T: IntoVal<Env, Val>,
{
    pub fn to_val(&self) -> Val {
        self.val.into_val(&self.env)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Bytes, FromVal, Vec};

    #[test]
    fn small_vals_are_comparable() {
        let env = Env::default();

        let a = Comparable::new(&env, 1u32.into_val(&env));
        let b = Comparable::new(&env, 1u32.into_val(&env));
        let c: Comparable<Val> = Comparable::new(&env, 2u32.into_val(&env));

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a < c);
    }

    #[test]
    fn object_vals_are_comparable() {
        let env = Env::default();

        let a = Comparable::new(&env, Bytes::from_slice(&env, &[1, 2, 3]).to_val());
        let b = Comparable::new(&env, Bytes::from_slice(&env, &[1, 2, 3]).to_val());
        let c: Comparable<Val> = Comparable::new(&env, Bytes::from_slice(&env, &[4]).to_val());

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a < c);
    }

    #[test]
    fn vals_of_different_types_are_comparable() {
        let env = Env::default();

        let a: Comparable<Val> = Comparable::new(&env, 1u32.into_val(&env));
        let b: Comparable<Val> = Comparable::new(&env, Bytes::from_slice(&env, &[1]).to_val());

        assert_ne!(a, b);
    }

    #[test]
    fn vals_are_comparable_across_envs() {
        let e1 = Env::default();
        let e2 = Env::default();

        let a: Comparable<Val> = Comparable::new(&e1, Bytes::from_slice(&e1, &[1, 2, 3]).to_val());
        let b: Comparable<Val> = Comparable::new(&e2, Bytes::from_slice(&e2, &[1, 2, 3]).to_val());
        let c: Comparable<Val> = Comparable::new(&e2, Bytes::from_slice(&e2, &[4]).to_val());

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a < c);
    }

    #[test]
    fn any_value_can_be_wrapped() {
        let env = Env::default();

        let a = Comparable::new(&env, Vec::from_array(&env, [1u32, 2, 3]));
        let b = Comparable::new(&env, Vec::from_array(&env, [1u32, 2, 3]));

        assert_eq!(a, b);
        assert_eq!(a.as_inner().len(), 3);
        assert_eq!(b.into_inner(), Vec::from_array(&env, [1u32, 2, 3]));
    }

    #[test]
    fn to_and_from_val() {
        let env = Env::default();

        let c: Comparable<Val> = Comparable::new(&env, 5u32.into_val(&env));
        let val: Val = c.to_val();
        let rt: Comparable<Val> = val.into_val(&env);

        assert_eq!(c, rt);
        assert_eq!(u32::from_val(&env, &rt.to_val()), 5);
    }
}
