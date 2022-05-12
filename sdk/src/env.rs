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
pub use env::EnvValConvertible;
pub use env::OrAbort;
pub use env::RawVal;
pub use env::Status;
pub use env::Symbol;
pub use env::TagObject;
pub use env::TaggedVal;
pub use env::Val;

pub type EnvVal<V> = env::EnvVal<Env, V>;

pub type Obj = TaggedVal<TagObject>;
pub type EnvObj = EnvVal<Obj>;

pub trait EnvRawValConvertible: EnvValConvertible<Env, RawVal> {}
impl<C> EnvRawValConvertible for C where C: EnvValConvertible<Env, RawVal> {}
