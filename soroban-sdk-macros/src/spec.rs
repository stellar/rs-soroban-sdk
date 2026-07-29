//! Generates the contract spec for a contract type.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::ScSpecEntry;
use syn::{ext::IdentExt as _, Ident, Path};

use crate::spec_ref;

/// Emits the contract spec for a contract type: the `spec_xdr` const fn holding
/// the spec entry's XDR, and the static that places its bytes in the contract's
/// spec section.
pub fn type_spec(path: &Path, ident: &Ident, entry: &ScSpecEntry) -> TokenStream2 {
    let spec_ref = spec_ref::spec_entry(path, entry);
    let spec_ident = format_ident!(
        "__SPEC_XDR_TYPE_{}",
        ident.unraw().to_string().to_uppercase()
    );
    quote! {
        #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
        pub static #spec_ident: [u8; #ident::__SPEC_XDR_REF.const_xdr_len()] = #ident::spec_xdr();

        impl #ident {
            #[doc(hidden)]
            pub const __SPEC_XDR_REF: #path::xdr::ScSpecEntryRef<'static> = #spec_ref;

            pub const fn spec_xdr() -> [u8; #ident::__SPEC_XDR_REF.const_xdr_len()] {
                #ident::__SPEC_XDR_REF.const_to_xdr()
            }
        }
    }
}
