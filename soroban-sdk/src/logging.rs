//! Logging contains types for logging debug events.
//!
//! See [`log`][crate::log] for how to conveniently log debug events.
use core::fmt::Debug;

use crate::{
    env::internal::{self, EnvBase},
    Bytes, Env, IntoVal, RawVal, Vec,
};

/// Log a debug event.
///
/// Takes a [Env], and a literal format string that containing `{}` for each
/// additional argument. Arguments may be any value that are convertible to
/// [`RawVal`].
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
/// [custom profiles]: https://doc.rust-lang.org/cargo/reference/profiles.html#custom-profiles
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
/// # #[cfg(feature = "testutils")]
/// # {
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
///         "a log entry: I32(5), Symbol(another), Object(Vec(#4))".to_string(),
///     ],
/// );
/// # }
/// ```
#[macro_export]
macro_rules! log {
    ($env:expr, $fmt:literal $(,)?) => {
        if cfg!(debug_assertions) {
            $env.logger().log($fmt, &[]);
        }
    };
    ($env:expr, $fmt:literal, $($args:expr),* $(,)?) => {
        if cfg!(debug_assertions) {
            $env.logger().log($fmt, &[
                $(
                    <_ as $crate::IntoVal<Env, $crate::RawVal>>::into_val($args, $env)
                ),*
            ]);
        }
    };
}

/// Logger logs debug events.
///
/// See [`log`][crate::log] for how to conveniently log debug events.
#[derive(Clone)]
pub struct Logger(Env);

impl Debug for Logger {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Logger")
    }
}

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
    ///
    /// Takes a literal format string that containing `{}` for each argument in
    /// the args slice.
    ///
    /// See [`log`][crate::log] for how to conveniently log debug events.
    #[inline(always)]
    pub fn log(&self, fmt: &'static str, args: &[RawVal]) {
        if cfg!(debug_assertions) {
            let env = self.env();
            // If building for WASM logging will be transmitted through the
            // guest interface via host types. This is very inefficient. When
            // building on non-WASM environments where the environment Host is
            // directly available, use the log static variants.
            if cfg!(target_family = "wasm") {
                let fmt: Bytes = fmt.into_val(env);
                let args: Vec<RawVal> = Vec::from_slice(env, args);
                internal::Env::log_fmt_values(env, fmt.to_object(), args.to_object());
            } else {
                env.log_static_fmt_general(fmt, args, &[]).unwrap();
            }
        }
    }
}

#[cfg(any(test, feature = "testutils"))]
use crate::{env::internal::events::HostEvent, testutils};

#[cfg(any(test, feature = "testutils"))]
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

    fn print(&self) {
        std::println!("{}", self.all().join("\n"))
    }
}
