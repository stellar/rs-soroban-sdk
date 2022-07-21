extern crate proc_macro;

mod derive_fn;
mod derive_type;
mod map_type;

use derive_fn::{derive_contract_function_set, derive_fn};
use derive_type::{derive_type_enum, derive_type_struct};

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, DeriveInput, Error, ImplItem,
    ImplItemMethod, ItemImpl, Visibility,
};

#[proc_macro]
pub fn contract(_input: TokenStream) -> TokenStream {
    quote! {
        #[cfg_attr(target_family = "wasm", link_section = "contractenvmetav0")]
        pub static __ENV_META_XDR: [u8; stellar_contract_sdk::meta::XDR.len()] = stellar_contract_sdk::meta::XDR;
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct ContractImplArgs {
    #[darling(default)]
    export_if: Option<String>,
    tests_if: Option<String>,
}

fn get_methods(imp: &ItemImpl) -> impl Iterator<Item = &ImplItemMethod> {
    imp.items.iter().filter_map(|i| match i {
        ImplItem::Method(m) => Some(m),
        _ => None,
    })
}

#[proc_macro_attribute]
pub fn contractimpl(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractImplArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let imp = parse_macro_input!(input as ItemImpl);
    let is_trait = imp.trait_.is_some();
    let ty = &imp.self_ty;
    let derived: Result<proc_macro2::TokenStream, proc_macro2::TokenStream> = get_methods(&imp)
        .filter(|m| is_trait || matches!(m.vis, Visibility::Public(_)))
        .map(|m| {
            let ident = &m.sig.ident;
            let call = quote! { <super::#ty>::#ident };
            let trait_ident = imp.trait_.as_ref().and_then(|x| x.1.get_ident());
            derive_fn(
                &call,
                ident,
                &m.sig.inputs,
                &m.sig.output,
                &args.export_if,
                &trait_ident,
            )
        })
        .collect();

    match derived {
        Ok(derived_ok) => {
            let cfs = derive_contract_function_set(ty, get_methods(&imp), &args.tests_if);
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
        #[derive(stellar_contract_sdk::ContractType)]
        #input
    }
    .into()
}

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
