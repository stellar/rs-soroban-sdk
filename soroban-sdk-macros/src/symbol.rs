use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, LitStr, Path};

use soroban_env_common::{Symbol, SymbolError};

/// Generates code that renders the Symbol as a compile-time const Symbol if
/// small enough, otherwise generates a compile error.
///
/// Also generates a compile error if the string cannot be represented as a
/// Symbol.
pub fn short(crate_path: &Path, s: &LitStr) -> TokenStream {
    match Symbol::try_from_small_str(&s.value()) {
        Ok(_) => quote! {{
            #[allow(deprecated)]
            const SYMBOL: #crate_path::Symbol = #crate_path::Symbol::short(#s);
            SYMBOL
        }},
        Err(e) => Error::new(s.span(), format!("{e}")).to_compile_error(),
    }
}

/// Generates code that renders the Symbol as a compile-time const Symbol if
/// small enough, otherwise as a Symbol::new construction.
///
/// Also generates a compile error if the string cannot be represented as a
/// Symbol.
pub fn short_or_long(crate_path: &Path, env: TokenStream, s: &LitStr) -> TokenStream {
    match Symbol::try_from_small_str(&s.value()) {
        Ok(_) => quote! {{
            #[allow(deprecated)]
            const SYMBOL: #crate_path::Symbol = #crate_path::Symbol::short(#s);
            SYMBOL
        }},
        Err(SymbolError::TooLong(_)) => quote! {{
            #crate_path::Symbol::new(#env, #s)
        }},
        Err(e) => Error::new(s.span(), format!("{e}")).to_compile_error(),
    }
}
