use core::fmt::Debug;

use paste::paste;

use crate::{env::internal, vec, Env, IntoVal, RawVal, Vec};

/// Events publishes events for the currently executing contract.
///
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
/// let events = env.events();
/// let sender = [0; 32];
/// let receiver = [1; 32];
/// let data = map![&env, (1u32, 2u32)];
/// events.publish_2((sender, receiver), data)
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
        write!(f, "ContractEvent")
    }
}

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
    /// One to four event topics can be provided by using:
    /// - [Events::publish_1]
    /// - [Events::publish_2]
    /// - [Events::publish_3]
    /// - [Events::publish_4]
    #[inline(always)]
    pub fn publish<D>(&self, data: D)
    where
        D: IntoVal<Env, RawVal>,
    {
        let env = self.env();
        internal::Env::contract_event(env, Vec::<RawVal>::new(env).to_object(), data.into_val(env));
    }
}

#[cfg(doc)]
use crate::{contracttype, Bytes, BytesN, Map};

macro_rules! publish_fn {
    ( $count:literal $($typ:ident $idx:tt)+ ) => {
        paste! {
            #[doc = "Publish an event with"]
            #[doc = stringify!($count)]
            #[doc = "topics"]
            ///
            /// Event data is specified in `data`. Data may be any value or
            /// type, including types defined by contracts using [contracttype].
            ///
            /// Event topics must not contain:
            /// - [Vec]
            /// - [Map]
            /// - [Bytes]/[BytesN] longer than 32 bytes
            /// - [contracttype]
            ///
            /// ### Panics
            ///
            /// When topics contain the types noted above that are disallowed.
            pub fn [<publish_ $count>]<$($typ),*, D>(&self, topics: ($($typ,)*), data: D)
            where
                $($typ: IntoVal<Env, RawVal>,)*
                D: IntoVal<Env, RawVal>,
            {
                let env = self.env();
                let topics = vec![
                    &env,
                    $(topics.$idx.into_val(env),)*
                ];
                internal::Env::contract_event(env, topics.to_object(), data.into_val(env));
            }
        }
    };
}

impl Events {
    publish_fn!(1 T0 0);
    publish_fn!(2 T0 0 T1 1);
    publish_fn!(3 T0 0 T1 1 T2 2);
    publish_fn!(4 T0 0 T1 1 T2 2 T3 3);
}
