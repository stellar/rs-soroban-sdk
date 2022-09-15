use core::fmt::Debug;

use soroban_env_host::EnvBase;

#[cfg(doc)]
use crate::{contracttype, Bytes, BytesN, Map};
use crate::{
    env::internal::{self},
    Env, IntoVal, RawVal, Vec,
};

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
pub struct Debugger(Env);

impl Debug for Debugger {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Debug")
    }
}

pub trait DebugArgs: IntoVal<Env, Vec<RawVal>> {}

macro_rules! impl_topics_for_tuple {
    ( $($typ:ident $idx:tt)* ) => {
        impl<$($typ),*> DebugArgs for ($($typ,)*)
        where
            $($typ: IntoVal<Env, RawVal>),*
        {
        }
    };
}

// 0 args
impl DebugArgs for () {}
// 1-4 args
impl_topics_for_tuple! { T0 0 }
impl_topics_for_tuple! { T0 0 T1 1 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 }
impl_topics_for_tuple! { T0 0 T1 1 T2 2 T3 3 }

impl Debugger {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Debugger {
        Debugger(env.clone())
    }

    /// Log a debug event.
    #[inline(always)]
    pub fn debug(&self, fmt: &'static str, arg: impl IntoVal<Env, RawVal>) {
        let env = self.env();
        let host = env.host();
        host.log_static_fmt_val(fmt, arg.into_val(env))
    }
}

#[cfg(feature = "testutils")]
use crate::{testutils, xdr, Bytes, TryIntoVal};

#[cfg(feature = "testutils")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl testutils::Debugger for Debugger {
    fn all(&self) -> Vec<(Bytes, RawVal)> {
        let env = self.env();
        let mut vec = Vec::new(env);
        self.env()
            .host()
            .get_events()
            .unwrap()
            .0
            .into_iter()
            .for_each(|e| {
                if let internal::events::HostEvent::Debug(internal::events::DebugEvent {
                    msg: Some(msg),
                    args,
                }) = e
                {
                    let msg: Bytes = msg.try_into_val(env).unwrap();
                    vec.push_back((
                        msg,
                        args.try_into_val(env).unwrap(),
                    ))
                }
            });
        vec
    }
}
