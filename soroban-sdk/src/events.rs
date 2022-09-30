//! Events contains types for publishing contract events.
use core::fmt::Debug;

#[cfg(doc)]
use crate::{contracttype, Bytes, BytesN, Map};
use crate::{
    env::internal::{self},
    Env, IntoVal, RawVal, Vec,
};

// TODO: consolidate with host::events::TOPIC_BYTES_LENGTH_LIMIT
const TOPIC_BYTES_LENGTH_LIMIT: u32 = 32;

/// Events publishes events for the currently executing contract.
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
/// let event = env.events();
/// let data = map![&env, (1u32, 2u32)];
/// let topics0 = ();
/// let topics1 = (0u32,);
/// let topics2 = (0u32, 1u32);
/// let topics3 = (0u32, 1u32, 2u32);
/// let topics4 = (0u32, 1u32, 2u32, 3u32);
/// event.publish(topics0, data.clone());
/// event.publish(topics1, data.clone());
/// event.publish(topics2, data.clone());
/// event.publish(topics3, data.clone());
/// event.publish(topics4, data.clone());
/// #     }
/// # }
///
/// # #[cfg(feature = "testutils")]
/// # fn main() {
/// #     let env = Env::default();
/// #     let contract_id = BytesN::from_array(&env, &[0; 32]);
/// #     env.register_contract(&contract_id, Contract);
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

pub trait Topics: IntoVal<Env, Vec<RawVal>> {}

impl IntoVal<Env, Vec<RawVal>> for () {
    fn into_val(self, env: &Env) -> Vec<RawVal> {
        Vec::<RawVal>::new(env)
    }
}

macro_rules! impl_topics_for_tuple {
    ( $($typ:ident $idx:tt)* ) => {
        impl<$($typ),*> Topics for ($($typ,)*)
        where
            $($typ: IntoVal<Env, RawVal>),*
        {
        }
    };
}

// 0 topics
impl Topics for () {}
// 1-4 topics
impl_topics_for_tuple! { T0 0 }
impl_topics_for_tuple! { T0 0 T1 1 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 }

impl Events {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Events {
        Events(env.clone())
    }

    /// Publish an event.
    ///
    /// Event data is specified in `data`. Data may be any value or
    /// type, including types defined by contracts using [contracttype].
    ///
    /// Event topics must not contain:
    ///
    /// - [Vec]
    /// - [Map]
    /// - [Bytes]/[BytesN] longer than 32 bytes
    /// - [contracttype]
    #[inline(always)]
    pub fn publish<T, D>(&self, topics: T, data: D)
    where
        T: Topics,
        D: IntoVal<Env, RawVal>,
    {
        let env = self.env();
        internal::Env::contract_event(env, topics.into_val(env).to_object(), data.into_val(env));
    }
}

#[cfg(any(test, feature = "testutils"))]
use crate::{testutils, xdr, TryIntoVal};

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl testutils::Events for Events {
    fn all(&self) -> Vec<(crate::BytesN<32>, Vec<RawVal>, RawVal)> {
        let env = self.env();
        let mut vec = Vec::new(env);
        self.env()
            .host()
            .get_events()
            .unwrap()
            .0
            .into_iter()
            .for_each(|e| {
                if let internal::events::HostEvent::Contract(xdr::ContractEvent {
                    type_: xdr::ContractEventType::Contract,
                    contract_id: Some(contract_id),
                    body: xdr::ContractEventBody::V0(xdr::ContractEventV0 { topics, data }),
                    ..
                }) = e
                {
                    vec.push_back((
                        contract_id.0.into_val(env),
                        topics.try_into_val(env).unwrap(),
                        data.try_into_val(env).unwrap(),
                    ))
                }
            });
        vec
    }
}
