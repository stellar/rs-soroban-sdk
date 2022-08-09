extern crate proc_macro;

mod derive_fn;
mod derive_type;
mod map_type;
mod syn_ext;

use std::env;

use derive_fn::{derive_contract_function_set, derive_fn};
use derive_type::{derive_type_enum, derive_type_struct};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Error, ItemImpl, Visibility};

#[proc_macro_attribute]
pub fn contractimpl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    // Export the functions in the wasm and in the contract spec if the package
    // is being built as the primary package. If the crate is imported to
    // another package to reuse types, export will be false. We require that any
    // crate containing a contract must export their types and functions
    // themselves so that dependency crates do not accidentally export their
    // contract implementations into a developers contract.
    let export = env::var("CARGO_PRIMARY_PACKAGE").is_ok();

    let imp = parse_macro_input!(input as ItemImpl);
    let ty = &imp.self_ty;
    let pub_methods: Vec<_> = syn_ext::impl_pub_methods(&imp).collect();
    let derived: Result<proc_macro2::TokenStream, proc_macro2::TokenStream> = pub_methods
        .iter()
        .map(|m| {
            let ident = &m.sig.ident;
            let call = quote! { <super::#ty>::#ident };
            let trait_ident = imp.trait_.as_ref().and_then(|x| x.1.get_ident());
            derive_fn(
                &call,
                ident,
                &m.sig.inputs,
                &m.sig.output,
                export,
                &trait_ident,
            )
        })
        .collect();

    match derived {
        Ok(derived_ok) => {
            let cfs = derive_contract_function_set(ty, pub_methods.into_iter());
            quote! {
                #imp
                #derived_ok
                #cfs
            }
            .into()
        }
        Err(derived_err) => quote! {
            #imp
            #derived_err
        }
        .into(),
    }
}

#[proc_macro_attribute]
pub fn contracttype(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    quote! {
        #[derive(soroban_sdk::ContractType)]
        #input
    }
    .into()
}

#[doc(hidden)]
#[proc_macro_derive(ContractType)]
pub fn derive_contract_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let gen_spec = matches!(input.vis, Visibility::Public(_));
    let derived = match &input.data {
        syn::Data::Struct(s) => derive_type_struct(ident, s, gen_spec),
        syn::Data::Enum(e) => derive_type_enum(ident, e, gen_spec),
        syn::Data::Union(u) => Error::new(
            u.union_token.span(),
            "unions are unsupported as contract types",
        )
        .to_compile_error(),
    };
    quote! { #derived }.into()
}
