//! Serialize and deserialize values to and from [Bytes].
//!
//! All types that are convertible to and from [RawVal] implement the
//! [Serialize] and [Deserialize] traits, and serialize to the ScVal XDR form.
//!
//! ### Examples
//!
//! ```
//! use soroban_sdk::{
//!     serde::{Deserialize, Serialize},
//!     Env, Bytes, IntoVal, TryFromVal,
//! };
//!
//! let env = Env::default();
//!
//! let value: u32 = 5;
//!
//! let bytes = value.serialize(&env);
//! assert_eq!(bytes.len(), 8);
//!
//! let roundtrip = u32::deserialize(&env, &bytes);
//! assert_eq!(roundtrip, Ok(value));
//! ```

use crate::{env::internal::Env as _, Bytes, Env, IntoVal, RawVal, TryFromVal};

/// Implemented by types that can be serialized to [Bytes].
///
/// All types that are convertible to [RawVal] are implemented.
pub trait Serialize {
    fn serialize(self, env: &Env) -> Bytes;
}

/// Implemented by types that can be deserialized from [Bytes].
///
/// All types that are convertible from [RawVal] are implemented.
pub trait Deserialize: Sized {
    type Error;
    fn deserialize(env: &Env, b: &Bytes) -> Result<Self, Self::Error>;
}

impl<T> Serialize for T
where
    T: IntoVal<Env, RawVal>,
{
    fn serialize(self, env: &Env) -> Bytes {
        let val: RawVal = self.into_val(env);
        let bin = env.serialize_to_bytes(val);
        unsafe { Bytes::unchecked_new(env.clone(), bin) }
    }
}

impl<T> Deserialize for T
where
    T: TryFromVal<Env, RawVal>,
{
    type Error = T::Error;

    fn deserialize(env: &Env, b: &Bytes) -> Result<Self, Self::Error> {
        let t = env.deserialize_from_bytes(b.into());
        T::try_from_val(env, t)
    }
}
