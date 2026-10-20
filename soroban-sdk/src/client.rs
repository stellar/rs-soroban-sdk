//! Types that generated contract clients use for the arguments and return
//! values of contract functions.
//!
//! [`Type`] gives the types that a client uses for a Rust type that appears in
//! a contract function, [`Arg`] the values a client accepts for an argument,
//! and [`Output`] the values a client returns.
//!
//! The macros refer to every argument and return type of a client function
//! through these traits, rather than deciding by the type's name, so that the
//! compiler resolves the type and the client types come from the type the name
//! resolves to. A type alias therefore has the client types of the type it
//! aliases.

use core::borrow::Borrow;

use crate::{
    crypto::{
        bls12_381::{Bls12381Fp, Bls12381Fp2, Bls12381Fr, Bls12381G1Affine, Bls12381G2Affine},
        bn254::{Bn254Fp, Bn254Fr, Bn254G1Affine, Bn254G2Affine},
        Hash,
    },
    Address, Bytes, BytesN, Duration, Env, Error, IntoVal, InvokeError, Map, MuxedAddress, String,
    Symbol, Timepoint, TryFromVal, Val, Vec, I256, U256,
};

/// Marker for [`Type::Arg`] that a client takes the argument as a reference
/// to any value that borrows as it, such as `&T` or `&&T`.
pub struct ArgByBorrow;

/// Marker for [`Type::Arg`] that a client takes the argument as any value
/// that converts into it, as `impl Into<T>`.
pub struct ArgByInto;

/// Trait for types that can be an argument or return value of a contract
/// client function.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used in a contract client",
    label = "not a contract type",
    note = "types used in contract functions must be supported by contract clients, such as types marked `#[contracttype]`"
)]
pub trait Type {
    /// How a client takes the type as an argument, either [`ArgByBorrow`]
    /// or [`ArgByInto`].
    type Arg;
    /// The type a client returns for the type.
    type Ret;
}

/// Trait for the values a client function accepts for an argument of type `T`,
/// which are decided by the type's [`Type::Arg`].
///
/// `Mode` defaults to the type's [`Type::Arg`] and is only a parameter so that
/// the impls for each mode do not overlap, as the compiler does not use an
/// associated type to tell impls apart.
pub trait Arg<T: Type, Mode = <T as Type>::Arg> {
    fn into_val_for_client(self, env: &Env) -> Val;
}

// Any reference that borrows as the type is accepted, so that a reference to a
// reference is accepted as it would be by deref coercion.
impl<'a, T, R> Arg<T, ArgByBorrow> for &'a R
where
    T: Type + IntoVal<Env, Val>,
    R: ?Sized + Borrow<T>,
{
    fn into_val_for_client(self, env: &Env) -> Val {
        let t: &T = self.borrow();
        t.into_val(env)
    }
}

impl<T, X> Arg<T, ArgByInto> for X
where
    T: Type + IntoVal<Env, Val>,
    X: Into<T>,
{
    fn into_val_for_client(self, env: &Env) -> Val {
        let t: T = self.into();
        t.into_val(env)
    }
}

/// Trait for the return type of a contract function, giving the value a client
/// returns, and the error a client returns when the contract returns an error.
pub trait Output {
    type Ok;
    type Err;
}

impl<T: Type> Output for T {
    type Ok = T::Ret;
    type Err = Error;
}

// Result does not implement Type, so that it is only unpacked as the
// return type of a contract function.
impl<T: Type, E> Output for Result<T, E> {
    type Ok = T::Ret;
    type Err = E;
}

/// The value a client function returns for a contract function that returns
/// `T`.
///
/// It is the [`Type::Ret`] of `T`, or of the ok type if `T` is a `Result`. For
/// example, a client returns `BytesN<32>` for a contract function that returns
/// `Hash<32>`.
pub type Ret<T> = <T as Output>::Ok;

/// The value a client's `try_` function returns for a contract function that
/// returns `T`.
///
/// The outer result is an error if the invocation fails, holding the contract's
/// error if it returned one, or the [`InvokeError`] otherwise. The inner result
/// is an error if the returned value cannot be converted into [`Ret<T>`].
pub type TryRet<T> = Result<
    Result<Ret<T>, <Ret<T> as TryFromVal<Env, Val>>::Error>,
    Result<<T as Output>::Err, InvokeError>,
>;

macro_rules! impl_client_type {
    ($($t:ty),* $(,)?) => {
        $(
            impl Type for $t {
                type Arg = ArgByBorrow;
                type Ret = $t;
            }
        )*
    };
}

impl_client_type! {
    Val,
    bool,
    (),
    Error,
    u32,
    i32,
    u64,
    i64,
    Timepoint,
    Duration,
    u128,
    i128,
    U256,
    I256,
    Bytes,
    String,
    Symbol,
    Address,
    Bls12381Fp,
    Bls12381Fp2,
    Bls12381G1Affine,
    Bls12381G2Affine,
    Bls12381Fr,
    Bn254Fp,
    Bn254G1Affine,
    Bn254G2Affine,
    Bn254Fr,
}

// A muxed address argument accepts anything that converts into one, such as an
// Address.
impl Type for MuxedAddress {
    type Arg = ArgByInto;
    type Ret = MuxedAddress;
}

impl<const N: usize> Type for BytesN<N> {
    type Arg = ArgByBorrow;
    type Ret = BytesN<N>;
}

// A hash can only be constructed by the host, so a client receives the bytes of
// a returned hash.
impl<const N: usize> Type for Hash<N> {
    type Arg = ArgByBorrow;
    type Ret = BytesN<N>;
}

impl<T: Type> Type for Option<T> {
    type Arg = ArgByBorrow;
    type Ret = Option<T::Ret>;
}

impl<T: Type> Type for Vec<T> {
    type Arg = ArgByBorrow;
    type Ret = Vec<T::Ret>;
}

impl<K: Type, V: Type> Type for Map<K, V> {
    type Arg = ArgByBorrow;
    type Ret = Map<K::Ret, V::Ret>;
}

macro_rules! impl_client_type_for_tuple {
    ($($t:ident),+) => {
        impl<$($t: Type),+> Type for ($($t,)+) {
            type Arg = ArgByBorrow;
            type Ret = ($($t::Ret,)+);
        }
    };
}

impl_client_type_for_tuple!(T0);
impl_client_type_for_tuple!(T0, T1);
impl_client_type_for_tuple!(T0, T1, T2);
impl_client_type_for_tuple!(T0, T1, T2, T3);
impl_client_type_for_tuple!(T0, T1, T2, T3, T4);
impl_client_type_for_tuple!(T0, T1, T2, T3, T4, T5);
impl_client_type_for_tuple!(T0, T1, T2, T3, T4, T5, T6);
impl_client_type_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_client_type_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_client_type_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_client_type_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_client_type_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
