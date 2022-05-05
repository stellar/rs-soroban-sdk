pub use stellar_contract_env_host::xdr;

pub use stellar_contract_env_host::OrAbort;
pub use stellar_contract_env_host::BitSet;
pub use stellar_contract_env_host::EnvObj;
pub use stellar_contract_env_host::EnvVal;
pub use stellar_contract_env_host::EnvValType;
pub use stellar_contract_env_host::HasEnv;
pub use stellar_contract_env_host::Status;
pub use stellar_contract_env_host::Symbol;
pub use stellar_contract_env_host::Val;
pub use stellar_contract_env_host::ValType;

pub type Host = stellar_contract_env_host::Host;
pub type Env = stellar_contract_env_host::WeakHost;
pub type Object = EnvObj<Env>;
