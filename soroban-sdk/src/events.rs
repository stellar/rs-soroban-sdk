//! Events contains types for publishing contract events.
use core::fmt::Debug;

#[cfg(doc)]
use crate::{contracttype, Bytes, Map};
use crate::{env::internal, unwrap::UnwrapInfallible, Env, IntoVal, Val, Vec};

// TODO: consolidate with host::events::TOPIC_BYTES_LENGTH_LIMIT
const TOPIC_BYTES_LENGTH_LIMIT: u32 = 32;

/// Events publishes events for the currently executing contract.
///
/// ```
/// use soroban_sdk::Env;
///
/// # use soroban_sdk::{contract, contractimpl, vec, map, Val, BytesN};
/// #
/// # #[contract]
/// # pub struct Contract;
/// #
/// # #[contractimpl]
/// # impl Contract {
/// #     pub fn f(env: Env) {
/// let event = env.events();
/// let data = map![&env, (1u32, 2u32)];
/// // topics can be represented with tuple up to a certain length
/// let topics0 = ();
/// let topics1 = (0u32,);
/// let topics2 = (0u32, 1u32);
/// let topics3 = (0u32, 1u32, 2u32);
/// let topics4 = (0u32, 1u32, 2u32, 3u32);
/// // topics can also be represented with a `Vec` with no length limit
/// let topics5 = vec![&env, 4u32, 5u32, 6u32, 7u32, 8u32];
/// event.publish(topics0, data.clone());
/// event.publish(topics1, data.clone());
/// event.publish(topics2, data.clone());
/// event.publish(topics3, data.clone());
/// event.publish(topics4, data.clone());
/// event.publish(topics5, data.clone());
/// #     }
/// # }
///
/// # #[cfg(feature = "testutils")]
/// # fn main() {
/// #     let env = Env::default();
/// #     let contract_id = env.register(Contract, ());
/// #     ContractClient::new(&env, &contract_id).f();
/// # }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
#[derive(Clone)]
pub struct Events(Env);

impl Debug for Events {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Events")
    }
}

#[cfg(any(test, feature = "testutils"))]
use crate::{testutils, xdr, Address, TryIntoVal};

pub trait Event {
    fn topics(&self, env: &Env) -> Vec<Val>;
    fn data(&self, env: &Env) -> Val;

    fn publish(&self, env: &Env) {
        env.events().publish_event(self);
    }

    /// Convert this event into a [`testutils::ContractEvent`] object for testing purposes.
    #[cfg(any(test, feature = "testutils"))]
    fn to_contract_event(
        &self,
        env: &Env,
        contract_id: &crate::Address,
    ) -> testutils::ContractEvent {
        testutils::ContractEvent::new(env, contract_id.clone(), self.topics(env), self.data(env))
    }
}

pub trait Topics: IntoVal<Env, Vec<Val>> {}

impl<T> Topics for Vec<T> {}

impl Events {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Events {
        Events(env.clone())
    }

    /// Publish an event defined using the [`contractevent`][crate::contractevent] macro.
    #[inline(always)]
    pub fn publish_event(&self, e: &(impl Event + ?Sized)) {
        let env = self.env();
        internal::Env::contract_event(env, e.topics(env).to_object(), e.data(env))
            .unwrap_infallible();
    }

    /// Publish an event.
    ///
    /// Consider using [`contractevent`][crate::contractevent] instead of this function.
    ///
    /// Event data is specified in `data`. Data may be any value or
    /// type, including types defined by contracts using [contracttype].
    ///
    /// Event topics must not contain:
    ///
    /// - [Vec]
    /// - [Map]
    /// - [Bytes]/[BytesN][crate::BytesN] longer than 32 bytes
    /// - [contracttype]
    #[deprecated(note = "use the #[contractevent] macro on a contract event type")]
    #[inline(always)]
    pub fn publish<T, D>(&self, topics: T, data: D)
    where
        T: Topics,
        D: IntoVal<Env, Val>,
    {
        let env = self.env();
        internal::Env::contract_event(env, topics.into_val(env).to_object(), data.into_val(env))
            .unwrap_infallible();
    }
}

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl testutils::Events for Events {
    fn all(&self) -> Vec<(crate::Address, Vec<Val>, Val)> {
        let env = self.env();
        let mut vec = Vec::new(&env);
        self.env()
            .host()
            .get_events()
            .unwrap()
            .0
            .into_iter()
            .for_each(|e| {
                if e.failed_call {
                    return;
                }
                if let xdr::ContractEvent {
                    type_: xdr::ContractEventType::Contract,
                    contract_id: Some(contract_id),
                    body: xdr::ContractEventBody::V0(xdr::ContractEventV0 { topics, data }),
                    ..
                } = e.event
                {
                    vec.push_back((
                        Address::from_contract_id(env, contract_id.0 .0),
                        topics.try_into_val(env).unwrap(),
                        data.try_into_val(env).unwrap(),
                    ))
                }
            });
        vec
    }

    fn contract_events(&self) -> std::vec::Vec<testutils::ContractEvent> {
        let env = self.env();
        self.env()
            .host()
            .get_events()
            .unwrap()
            .0
            .into_iter()
            .filter_map(|e| {
                if e.failed_call {
                    None
                } else if let xdr::ContractEvent {
                    type_: xdr::ContractEventType::Contract,
                    contract_id: Some(contract_id),
                    body: xdr::ContractEventBody::V0(xdr::ContractEventV0 { topics, data }),
                    ..
                } = e.event
                {
                    Some(testutils::ContractEvent::new(
                        &env,
                        Address::from_contract_id(env, contract_id.0 .0),
                        topics.try_into_val(env).unwrap(),
                        data.try_into_val(env).unwrap(),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    fn contract_events_for(
        &self,
        contract_id: &crate::Address,
    ) -> std::vec::Vec<testutils::ContractEvent> {
        self.contract_events()
            .into_iter()
            .filter(|e| &e.contract_id == contract_id)
            .collect()
    }
}
