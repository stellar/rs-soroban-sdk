extern crate proc_macro;

mod args;
mod derive_fn;
mod derive_type;
mod map_type;

use derive_fn::derive_fn;
use derive_type::{derive_type_enum, derive_type_struct};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, DeriveInput, Error, ImplItem, ItemImpl,
    Visibility,
};

#[proc_macro]
pub fn contract(_input: TokenStream) -> TokenStream {
    quote! {
        #[cfg_attr(target_family = "wasm", link_section = "contractenvmetav0")]
        pub static __ENV_META_XDR: [u8; stellar_contract_sdk::meta::XDR.len()] = stellar_contract_sdk::meta::XDR;
    }
    .into()
}

#[proc_macro_attribute]
pub fn contractimpl(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let feature = args::get_str(&args, "feature");
    let imp = parse_macro_input!(input as ItemImpl);
    let is_trait = imp.trait_.is_some();
    let ty = &imp.self_ty;
    let derived = imp
        .items
        .iter()
        .filter_map(|i| match i {
            ImplItem::Method(m) => Some(m),
            _ => None,
        })
        .filter(|m| is_trait || matches!(m.vis, Visibility::Public(_)))
        .map(|m| {
            let ident = &m.sig.ident;
            let call = quote! { <#ty>::#ident };
            derive_fn(&call, ident, &m.sig.inputs, &m.sig.output, &feature)
        });
    quote! {
        #imp
        #(#derived)*
    }
    .into()
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
