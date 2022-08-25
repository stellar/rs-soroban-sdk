#[cfg(doc)]
use crate::Symbol;

/// Create a [Symbol] with the given string.
///
/// The [Symbol] is generated at compile time and returned as a const.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{sym, Symbol};
///
/// let symbol = sym!("a_str");
/// assert_eq!(symbol, Symbol::from_str("a_str"));
/// ```
///
/// ```
/// use soroban_sdk::{sym, Symbol};
///
/// const symbol: Symbol = sym!("a_str");
/// assert_eq!(symbol, Symbol::from_str("a_str"));
/// ```
#[macro_export]
macro_rules! sym {
    ($str:literal) => {{
        const symbol: $crate::Symbol = $crate::Symbol::from_str($str);
        symbol
    }};
}
