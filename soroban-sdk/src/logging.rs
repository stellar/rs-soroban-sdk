use core::fmt::Debug;

#[cfg(doc)]
use crate::{contracttype, Bytes, BytesN, Map};
use crate::{env::internal::EnvBase, Env, IntoVal, RawVal};

/// Log a debug event.
///
/// The first argument in the list must be a reference to an [Env].
///
/// The following arguments may be any value that are convertible to [`RawVal`].
///
/// ### Examples
///
/// Log a string:
///
/// ```
/// use soroban_sdk::{log, Env};
///
/// let env = Env::default();
///
/// log!(&env, "a log entry");
/// ```
///
/// Log a string with values:
///
/// ```
/// use soroban_sdk::{log, symbol, Env};
///
/// let env = Env::default();
///
/// let value = 5;
/// log!(&env, "a log entry: {}, {}", value, symbol!("another"));
/// ```
///
/// Assert on logs in tests:
///
/// ```
/// use soroban_sdk::{log, symbol, Env};
///
/// let env = Env::default();
///
/// let value = 5;
/// log!(&env, "a log entry: {}, {}, {}", value, symbol!("another"), (1, 2, 1));
///
/// use soroban_sdk::testutils::Logger;
///
/// assert_eq!(
///     env.logger().all(),
///     std::vec![
///         "a log entry: I32(5), Symbol(another), Object(Vec(3))".to_string(),
///     ],
/// );
/// ```
#[macro_export]
macro_rules! log {
    ($env:expr, $fmt:literal $(,)?) => {
        if cfg!(debug_assertions) {
            $env.logger().log_raw($fmt, &[]);
        }
    };
    ($env:expr, $fmt:literal, $($args:expr),* $(,)?) => {
        if cfg!(debug_assertions) {
            $env.logger().log_raw($fmt, &[
                $(
                    <_ as $crate::IntoVal<Env, $crate::RawVal>>::into_val($args, $env)
                ),*
            ]);
        }
    };
}

/// Logger logs debug events.
///
/// See [`log`] for how to conveniently log debug events.

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
// 1-4 arg tuples
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
        if cfg!(debug_assertions) {
            let env = self.env();
            self.log_raw(fmt, &args.into_val(env));
        }
    }

    #[inline(always)]
    #[doc(hidden)]
    pub fn log_raw(&self, fmt: &'static str, args: &[RawVal]) {
        if cfg!(debug_assertions) {
            let env = self.env();
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
