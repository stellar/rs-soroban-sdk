pub use stellar_contract_env_guest::xdr;

pub use stellar_contract_env_guest::OrAbort;
pub use stellar_contract_env_guest::BitSet;
pub use stellar_contract_env_guest::EnvObj;
pub use stellar_contract_env_guest::EnvVal;
pub use stellar_contract_env_guest::EnvValType;
pub use stellar_contract_env_guest::HasEnv;
pub use stellar_contract_env_guest::Status;
pub use stellar_contract_env_guest::Symbol;
pub use stellar_contract_env_guest::Val;
pub use stellar_contract_env_guest::ValType;

pub type Env = stellar_contract_env_guest::Guest;
pub type Object = stellar_contract_env_guest::EnvObj<Env>;
