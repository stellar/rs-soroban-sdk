#[cfg(target_family = "wasm")]
pub use guest::*;

#[cfg(target_family = "wasm")]
mod guest {
    pub use stellar_contract_env_guest::xdr;

    pub use stellar_contract_env_guest::BitSet;
    pub use stellar_contract_env_guest::Env as EnvTrait;
    pub use stellar_contract_env_guest::EnvBase;
    pub use stellar_contract_env_guest::EnvValType;
    pub use stellar_contract_env_guest::OrAbort;
    pub use stellar_contract_env_guest::RawObj;
    pub use stellar_contract_env_guest::RawVal;
    pub use stellar_contract_env_guest::RawValType;
    pub use stellar_contract_env_guest::Status;
    pub use stellar_contract_env_guest::Symbol;

    pub type Env = stellar_contract_env_guest::Guest;
    pub type EnvVal = stellar_contract_env_guest::EnvVal<Env>;
    pub type EnvObj = stellar_contract_env_guest::EnvObj<Env>;
}

#[cfg(not(target_family = "wasm"))]
pub use host::*;

#[cfg(not(target_family = "wasm"))]
mod host {
    pub use stellar_contract_env_host::xdr;

    pub use stellar_contract_env_host::BitSet;
    pub use stellar_contract_env_host::Env as EnvTrait;
    pub use stellar_contract_env_host::EnvBase;
    pub use stellar_contract_env_host::EnvValType;
    pub use stellar_contract_env_host::OrAbort;
    pub use stellar_contract_env_host::RawObj;
    pub use stellar_contract_env_host::RawVal;
    pub use stellar_contract_env_host::RawValType;
    pub use stellar_contract_env_host::Status;
    pub use stellar_contract_env_host::Symbol;

    pub type Env = stellar_contract_env_host::Host;
    pub type EnvVal = stellar_contract_env_host::EnvVal<Env>;
    pub type EnvObj = stellar_contract_env_host::EnvObj<Env>;
}
