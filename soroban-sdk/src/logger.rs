use core::fmt::Debug;

#[cfg(doc)]
use crate::{contracttype, Bytes, BytesN, Map};
use crate::{env::internal::EnvBase, Env, IntoVal, RawVal};

/// Logger logs debug events.
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
pub struct Logger(Env);

impl Debug for Logger {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Logger")
    }
}

pub trait LogArgs<'a, const N: usize>: IntoVal<Env, [RawVal; N]> {}

macro_rules! impl_log_args_for_tuple {
    ( $count:literal $($typ:ident $idx:tt)* ) => {
        impl<'a, $($typ),*> LogArgs<'a, $count> for ($($typ,)*)
        where
            $($typ: IntoVal<Env, RawVal>),*
        {
        }
    };
}

// 0 args
impl<'a> LogArgs<'a, 0> for () {}
// 1-4 args
impl_log_args_for_tuple! { 1 T0 0 }
impl_log_args_for_tuple! { 2 T0 0 T1 1 }
impl_log_args_for_tuple! { 3 T0 0 T1 1 T2 2 }
impl_log_args_for_tuple! { 4 T0 0 T1 1 T2 2 T3 3 }

impl Logger {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Logger {
        Logger(env.clone())
    }

    /// Log a debug event.
    #[inline(always)]
    pub fn log<'a, const N: usize>(&self, fmt: &'static str, args: impl LogArgs<'a, N>) {
        if !cfg!(debug_assertions) {
            let env = self.env();
            let args = args.into_val(env);
            env.log_static_fmt_general(fmt, &args, &[]);
        }
    }
}

#[cfg(feature = "testutils")]
use crate::{env::internal::events::HostEvent, testutils};

#[cfg(feature = "testutils")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl testutils::Logger for Logger {
    fn all(&self) -> std::vec::Vec<String> {
        let env = self.env();
        env.host()
            .get_events()
            .unwrap()
            .0
            .into_iter()
            .filter_map(|e| match e {
                HostEvent::Debug(de) => Some(format!("{}", de)),
                _ => None,
            })
            .collect::<std::vec::Vec<_>>()
    }
}
