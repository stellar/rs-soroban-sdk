//! Logging contains types for logging debug events.
//!
//! See [`log`][crate::log] for how to conveniently log debug events.
use core::fmt::Debug;

use crate::{env::internal::EnvBase, Env, Val};

/// Log a debug event.
///
/// Takes a [Env], a literal string, and an optional trailing sequence of
/// arguments that may be any value that are convertible to [`Val`]. The
/// string and arguments are appended as-is to the log, as the body of a
/// structured diagnostic event. Such events may be emitted from the host as
/// auxiliary diagnostic XDR, or converted to strings later for debugging.
///
/// `log!` statements are only enabled in non optimized builds that have
/// `debug-assertions` enabled. To enable `debug-assertions` add the following
/// lines to `Cargo.toml`, then build with the profile specified, `--profile
/// release-with-logs`. See the cargo docs for how to use [custom profiles].
///
/// ```toml
/// [profile.release-with-logs]
/// inherits = "release"
/// debug-assertions = true
/// ```
///
/// [custom profiles]:
///     https://doc.rust-lang.org/cargo/reference/profiles.html#custom-profiles
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
/// use soroban_sdk::{log, symbol_short, Symbol, Env};
///
/// let env = Env::default();
///
/// let value = 5;
/// log!(&env, "a log entry", value, symbol_short!("another"));
/// ```
///
/// Assert on logs in tests:
///
/// ```
/// # #[cfg(feature = "testutils")]
/// # {
/// use soroban_sdk::{log, symbol_short, Symbol, Env};
///
/// let env = Env::default();
///
/// let value = 5;
/// log!(&env, "a log entry", value, symbol_short!("another"));
///
/// use soroban_sdk::testutils::Logs;
/// let logentry = env.logs().all().last().unwrap().clone();
/// assert!(logentry.contains("[\"a log entry\", 5, another]"));
/// # }
/// ```
#[macro_export]
macro_rules! log {
    ($env:expr, $fmt:literal $(,)?) => {
        if cfg!(debug_assertions) {
            $env.logs().add($fmt, &[]);
        }
    };
    ($env:expr, $fmt:literal, $($args:expr),* $(,)?) => {
        if cfg!(debug_assertions) {
            $env.logs().add($fmt, &[
                $(
                    <_ as $crate::IntoVal<Env, $crate::Val>>::into_val(&$args, $env)
                ),*
            ]);
        }
    };
}

/// Logs logs debug events.
///
/// See [`log`][crate::log] for how to conveniently log debug events.
#[derive(Clone)]
pub struct Logs(Env);

impl Debug for Logs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Logs")
    }
}

impl Logs {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Logs {
        Logs(env.clone())
    }

    #[deprecated(note = "use [Logs::add]")]
    #[inline(always)]
    pub fn log(&self, msg: &'static str, args: &[Val]) {
        self.add(msg, args);
    }

    /// Log a debug event.
    ///
    /// Takes a literal string and a sequence of trailing values to add
    /// as a log entry in the diagnostic event stream.
    ///
    /// See [`log`][crate::log] for how to conveniently log debug events.
    #[inline(always)]
    pub fn add(&self, msg: &'static str, args: &[Val]) {
        if cfg!(debug_assertions) {
            let env = self.env();
            env.log_from_slice(msg, args).unwrap();
        }
    }
}

#[cfg(any(test, feature = "testutils"))]
use crate::testutils;

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl testutils::Logs for Logs {
    fn all(&self) -> std::vec::Vec<String> {
        use crate::xdr::{
            ContractEventBody, ContractEventType, ScSymbol, ScVal, ScVec, StringM, VecM,
        };
        let env = self.env();
        let log_sym = ScSymbol(StringM::try_from("log").unwrap());
        let log_topics = ScVec(VecM::try_from(vec![ScVal::Symbol(log_sym)]).unwrap());
        env.host()
            .get_events()
            .unwrap()
            .0
            .into_iter()
            .filter_map(|e| match (&e.event.type_, &e.event.body) {
                (ContractEventType::Diagnostic, ContractEventBody::V0(ce))
                    if &ce.topics == &log_topics.0 =>
                {
                    Some(format!("{}", &e))
                }
                _ => None,
            })
            .collect::<std::vec::Vec<_>>()
    }

    fn print(&self) {
        std::println!("{}", self.all().join("\n"))
    }
}
