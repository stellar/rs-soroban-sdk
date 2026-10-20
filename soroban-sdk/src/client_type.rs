//! ClientType gives the types that a generated contract client uses for a Rust
//! type that appears in a contract function.
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
    Address, Bytes, BytesN, Duration, Env, Error, IntoVal, Map, MuxedAddress, String, Symbol,
    Timepoint, Val, Vec, I256, U256,
};

/// Marker for [`ClientType::Arg`] that a client takes the argument by
/// reference, as `&T`.
pub struct ClientArgByRef;

/// Marker for [`ClientType::Arg`] that a client takes the argument as any value
/// that converts into it, as `impl Into<T>`.
pub struct ClientArgByInto;

/// Trait for types that can be an argument or return value of a contract
/// client function.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used in a contract client",
    label = "not a contract type",
    note = "types used in contract functions must be supported by contract clients, such as types marked `#[contracttype]`"
)]
pub trait ClientType {
    /// How a client takes the type as an argument, either [`ClientArgByRef`]
    /// or [`ClientArgByInto`].
    type Arg;
    /// The type a client returns for the type.
    type Ret;
}

/// Trait for the values a client function accepts for an argument of type `T`,
/// which are decided by `Mode`, the type's [`ClientType::Arg`].
pub trait ClientArg<T, Mode> {
    fn into_val_for_client(self, env: &Env) -> Val;
}

// Any reference that borrows as the type is accepted, so that a reference to a
// reference is accepted as it would be by deref coercion.
impl<'a, T, R> ClientArg<T, ClientArgByRef> for &'a R
where
    T: IntoVal<Env, Val>,
    R: ?Sized + Borrow<T>,
{
    fn into_val_for_client(self, env: &Env) -> Val {
        let t: &T = self.borrow();
        t.into_val(env)
    }
}

impl<T, X> ClientArg<T, ClientArgByInto> for X
where
    T: IntoVal<Env, Val>,
    X: Into<T>,
{
    fn into_val_for_client(self, env: &Env) -> Val {
        let t: T = self.into();
        t.into_val(env)
    }
}

/// Trait for the return type of a contract function, giving the value a client
/// returns, and the error a client returns when the contract returns an error.
pub trait ClientOutput {
    type Ok;
    type Err;
}

impl<T: ClientType> ClientOutput for T {
    type Ok = T::Ret;
    type Err = Error;
}

// Result does not implement ClientType, so that it is only unpacked as the
// return type of a contract function.
impl<T: ClientType, E> ClientOutput for Result<T, E> {
    type Ok = T::Ret;
    type Err = E;
}

macro_rules! impl_client_type {
    ($($t:ty),* $(,)?) => {
        $(
            impl ClientType for $t {
                type Arg = ClientArgByRef;
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
impl ClientType for MuxedAddress {
    type Arg = ClientArgByInto;
    type Ret = MuxedAddress;
}

impl<const N: usize> ClientType for BytesN<N> {
    type Arg = ClientArgByRef;
    type Ret = BytesN<N>;
}

// A hash can only be constructed by the host, so a client receives the bytes of
// a returned hash.
impl<const N: usize> ClientType for Hash<N> {
    type Arg = ClientArgByRef;
    type Ret = BytesN<N>;
}

impl<T: ClientType> ClientType for Option<T> {
    type Arg = ClientArgByRef;
    type Ret = Option<T::Ret>;
}

impl<T: ClientType> ClientType for Vec<T> {
    type Arg = ClientArgByRef;
    type Ret = Vec<T::Ret>;
}

impl<K: ClientType, V: ClientType> ClientType for Map<K, V> {
    type Arg = ClientArgByRef;
    type Ret = Map<K::Ret, V::Ret>;
}

macro_rules! impl_client_type_for_tuple {
    ($($t:ident),+) => {
        impl<$($t: ClientType),+> ClientType for ($($t,)+) {
            type Arg = ClientArgByRef;
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
