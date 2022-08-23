use core::fmt::Debug;

use crate::{env::internal, Env, IntoVal, RawVal, Vec};

/// ### Examples
///
/// ```
/// use soroban_sdk::Env;
///
/// # use soroban_sdk::{contractimpl, vec, map, RawVal, BytesN};
/// #
/// # pub struct Contract;
/// #
/// # #[contractimpl]
/// # impl Contract {
/// #     pub fn f(env: Env) {
/// let event = env.contract_event();
/// let topics = vec![&env, 0u32, 1u32];
/// let data = map![&env, (1u32, 2u32)];
/// event.publish(topics, data)
/// #     }
/// # }
///
/// # #[cfg(feature = "testutils")]
/// # fn main() {
/// #     let env = Env::default();
/// #     let contract_id = BytesN::from_array(&env, &[0; 32]);
/// #     env.register_contract(&contract_id, Contract);
/// #     f::invoke(&env, &contract_id);
/// # }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
#[derive(Clone)]
pub struct ContractEvent(Env);

impl Debug for ContractEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ContractEvent")
    }
}

impl ContractEvent {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> ContractEvent {
        ContractEvent(env.clone())
    }

    /// Publishes `topics` and `data` into a contract event.
    /// `topics` is expected to have length <= 4 and cannot contain Vecs, Maps,
    /// or Binaries > 32 bytes
    #[inline(always)]
    pub fn publish<T, D>(&self, topics: Vec<T>, data: D)
    where
        T: IntoVal<Env, RawVal>,
        D: IntoVal<Env, RawVal>,
    {
        let env = self.env();
        internal::Env::contract_event(env, topics.to_object(), data.into_val(env));
    }
}
