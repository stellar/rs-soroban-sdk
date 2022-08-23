use core::fmt::Debug;

use super::xdr::ScObjectType;

// TODO: consolidate with host::events::TOPIC_BYTES_LENGTH_LIMIT
const TOPIC_BYTES_LENGTH_LIMIT: u32 = 32;

use crate::{
    env::{internal, EnvObj},
    Bytes, Env, IntoVal, Object, RawVal, Vec,
};

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
/// let topics = (0u32, 1u32);
/// let data = map![&env, (1u32, 2u32)];
/// event.publish(topics, data)
/// #     }
/// # }
///
/// # #[cfg(feature = "testutils")]
/// # fn main() {
/// #     let env = Env::default();
/// #     let contract_id = BytesN::from_array(&env, [0; 32]);
/// #     env.register_contract(&contract_id, Contract);
/// #     f::invoke(&env, &contract_id);
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

pub trait IntoTopics {
    fn into_topics(self, env: &Env) -> Object;
}

// 0 topics
impl IntoTopics for () {
    fn into_topics(self, env: &Env) -> Object {
        Vec::<RawVal>::new(env).to_object()
    }
}

macro_rules! impl_for_tuple {
    ( $($typ:ident $idx:tt)* ) => {
        impl<$($typ),*> IntoTopics for ($($typ,)*)
        where
            $($typ: IntoVal<Env, RawVal> + Clone),*
        {
            fn into_topics(self, env: &Env) -> Object {
                $({
                    let ev = self.$idx.clone().into_env_val(env);
                    match TryInto::<EnvObj>::try_into(ev) {
                        Ok(obj) => {
                            if obj.as_object().is_obj_type(ScObjectType::Vec) {
                                panic!("topic cannot be a vec")
                            } else if obj.as_object().is_obj_type(ScObjectType::Map) {
                                panic!("topic cannot be a map")
                            } else if obj.as_object().is_obj_type(ScObjectType::Bytes) {
                                let bytes = unsafe { Bytes::unchecked_new(obj.clone()) };
                                if bytes.len() > TOPIC_BYTES_LENGTH_LIMIT {
                                    panic!("topic exceeds bytes length limit")
                                }
                            }
                        }
                        Err(_) => (),
                    }
                })*
                self.into_val(env)
            }
        }
    };
}

// 1-4 topics
impl_for_tuple! { T0 0 }
impl_for_tuple! { T0 0 T1 1 }
impl_for_tuple! { T0 0 T1 1 T2 2 }
impl_for_tuple! { T0 0 T1 1 T2 2 T3 3 }

impl Events {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Events {
        Events(env.clone())
    }

    /// Publishes `topics` and `data` into a contract event.
    /// `topics` is expected to have length <= 4 and cannot contain Vecs, Maps,
    /// or Binaries > 32 bytes
    #[inline(always)]
    pub fn publish<T, D>(&self, topics: T, data: D)
    where
        T: IntoTopics,
        D: IntoVal<Env, RawVal>,
    {
        let env = self.env();
        internal::Env::contract_event(env, topics.into_topics(env), data.into_val(env));
    }
}
