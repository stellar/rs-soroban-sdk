//! Convert values to and from [Bytes].
//!
//! All types that are convertible to and from [Val] implement the
//! [ToXdr] and [FromXdr] traits, and serialize to the ScVal XDR form.
//!
//! ### Examples
//!
//! ```
//! use soroban_sdk::{
//!     xdr::{FromXdr, ToXdr},
//!     Env, Bytes, IntoVal, TryFromVal,
//! };
//!
//! let env = Env::default();
//!
//! let value: u32 = 5;
//!
//! let bytes = value.to_xdr(&env);
//! assert_eq!(bytes.len(), 8);
//!
//! let roundtrip = u32::from_xdr(&env, &bytes);
//! assert_eq!(roundtrip, Ok(value));
//! ```

use crate::{
    env::internal::Env as _, unwrap::UnwrapInfallible, Bytes, Env, IntoVal, TryFromVal, Val,
};

// Re-export all the XDR from the environment.
pub use crate::env::xdr::*;

/// Implemented by types that can be serialized to [Bytes] as XDR.
///
/// All types that are convertible to [Val] implement this trait. The value is
/// first converted to a [Val], then serialized to XDR in its [ScVal] form.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{xdr::ToXdr, Env};
///
/// let env = Env::default();
///
/// let value: u32 = 5;
/// let bytes = value.to_xdr(&env);
/// assert_eq!(bytes.len(), 8);
/// ```
pub trait ToXdr {
    /// Serializes the value to XDR as [Bytes].
    fn to_xdr(self, env: &Env) -> Bytes;
}

/// Implemented by types that can be deserialized from [Bytes] containing XDR.
///
/// All types that are convertible from [Val] implement this trait. The bytes
/// are deserialized from their [ScVal] XDR form into a [Val], then converted
/// to the target type.
///
/// ### Errors
///
/// Returns an error if the [Val] cannot be converted into the target type.
///
/// ### Panics
///
/// Panics if the provided bytes are not valid XDR for an [ScVal].
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{xdr::{ToXdr, FromXdr}, Env};
///
/// let env = Env::default();
///
/// let value: u32 = 5;
/// let bytes = value.to_xdr(&env);
///
/// let roundtrip = u32::from_xdr(&env, &bytes);
/// assert_eq!(roundtrip, Ok(5));
/// ```
pub trait FromXdr: Sized {
    /// The error type returned if the [Val] cannot be converted into the
    /// target type.
    type Error;
    /// Deserializes the value from XDR [Bytes].
    ///
    /// ### Errors
    ///
    /// Returns an error if the [Val] cannot be converted into the target
    /// type.
    ///
    /// ### Panics
    ///
    /// Panics if the provided bytes are not valid XDR for an [ScVal].
    fn from_xdr(env: &Env, b: &Bytes) -> Result<Self, Self::Error>;
}

impl<T> ToXdr for T
where
    T: IntoVal<Env, Val>,
{
    fn to_xdr(self, env: &Env) -> Bytes {
        let val: Val = self.into_val(env);
        let bin = env.serialize_to_bytes(val).unwrap_infallible();
        unsafe { Bytes::unchecked_new(env.clone(), bin) }
    }
}

impl<T> FromXdr for T
where
    T: TryFromVal<Env, Val>,
{
    type Error = T::Error;

    fn from_xdr(env: &Env, b: &Bytes) -> Result<Self, Self::Error> {
        let t = env.deserialize_from_bytes(b.into()).unwrap_infallible();
        T::try_from_val(env, &t)
    }
}
