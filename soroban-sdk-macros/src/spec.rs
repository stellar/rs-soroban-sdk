//! Generates the contract spec for a contract type.

use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{ext::IdentExt as _, Ident};

/// Emits the contract spec for a contract type: the `spec_xdr` const fn holding
/// the spec entry's XDR, and the static that places its bytes in the contract's
/// spec section.
pub fn type_spec(ident: &Ident, spec_xdr: &[u8]) -> TokenStream2 {
    let spec_xdr_lit = Literal::byte_string(spec_xdr);
    let spec_xdr_len = spec_xdr.len();
    let spec_ident = format_ident!(
        "__SPEC_XDR_TYPE_{}",
        ident.unraw().to_string().to_uppercase()
    );
    quote! {
        #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
        pub static #spec_ident: [u8; #spec_xdr_len] = #ident::spec_xdr();

        impl #ident {
            pub const fn spec_xdr() -> [u8; #spec_xdr_len] {
                *#spec_xdr_lit
            }
        }
    }
}
