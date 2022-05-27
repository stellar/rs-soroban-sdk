#[cfg(target_family = "wasm")]
mod env {
    pub use stellar_contract_env_guest::{Env as EnvTrait, *};
    pub type Env = Guest;
}

#[cfg(not(target_family = "wasm"))]
mod env {
    pub use stellar_contract_env_host::{Env as EnvTrait, *};
    pub type Env = Host;
}

pub use env::xdr;
pub use env::BitSet;
pub use env::Env;
pub use env::EnvBase;
pub use env::EnvTrait;
pub use env::IntoEnvVal;
pub use env::IntoVal;
pub use env::RawVal;
pub use env::Status;
pub use env::Symbol;
pub use env::TagObject;
pub use env::TaggedVal;
pub use env::TryFromVal;
pub use env::Val;

pub type EnvVal<V> = env::EnvVal<Env, V>;

pub type Obj = TaggedVal<TagObject>;
pub type EnvObj = EnvVal<Obj>;

pub trait EnvRawValConvertible: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> {}
impl<C> EnvRawValConvertible for C where C: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> {}
