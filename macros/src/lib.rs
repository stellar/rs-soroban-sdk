extern crate proc_macro;

mod derive_and_spec_fn;
mod derive_type;
mod map_type;

use derive_and_spec_fn::derive_and_spec_fn;
use derive_type::{derive_type_enum, derive_type_struct};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, DeriveInput, Error, ImplItem, ItemFn, ItemImpl, Visibility,
};

#[proc_macro_attribute]
pub fn contractfn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut errors = Vec::<Error>::new();
    let func = parse_macro_input!(input as ItemFn);
    if !matches!(func.vis, Visibility::Public(_)) {
        errors.push(Error::new(func.span(), "contract functions must be public"));
    }
    let ident = &func.sig.ident;
    let call = quote! { #ident };
    let derive_and_spec = derive_and_spec_fn(&call, ident, &func.sig.inputs, &func.sig.output);
    let compile_errors = errors.iter().map(Error::to_compile_error);
    quote! {
        #func
        #(#compile_errors)*
        #derive_and_spec
    }
    .into()
}

#[proc_macro_attribute]
pub fn contractimpl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let imp = parse_macro_input!(input as ItemImpl);
    let is_trait = imp.trait_.is_some();
    let ty = &imp.self_ty;
    let derive_and_specs = imp
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
            derive_and_spec_fn(&call, ident, &m.sig.inputs, &m.sig.output)
        });
    quote! {
        #imp
        #(#derive_and_specs)*
    }
    .into()
}

#[proc_macro_attribute]
pub fn contracttype(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let struct_or_enum = parse_macro_input!(input as DeriveInput);
    // let spec = if matches!(strct.vis, Visibility::Public(_)) {
    //     let spec = spec_type(&strct.ident, &strct.fields);
    //     Some(spec)
    // } else {
    //     None
    // };
    quote! {
        #[derive(stellar_contract_sdk::IntoTryFromVal)]
        #struct_or_enum
        // #spec
    }
    .into()
}

#[proc_macro_derive(IntoTryFromVal)]
pub fn derive_into_try_from_val(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let derived = match &input.data {
        syn::Data::Struct(s) => derive_type_struct(ident, s),
        syn::Data::Enum(e) => derive_type_enum(ident, e),
        syn::Data::Union(_) => unimplemented!(),
    };
    quote! { #derived }.into()
}
