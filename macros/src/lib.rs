extern crate proc_macro;

mod map_type;
mod wrap_and_spec_fn;

use wrap_and_spec_fn::wrap_and_spec_fn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemFn, ItemImpl, Visibility, Error, spanned::Spanned};

#[proc_macro_attribute]
pub fn contractfn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut errors = Vec::<Error>::new();
    let func = parse_macro_input!(input as ItemFn);
    if !matches!(func.vis, Visibility::Public(_)) {
        errors.push(Error::new(
            func.span(),
            "contract functions must be public",
        ));
    }
    let ident = &func.sig.ident;
    let call = quote! { #ident };
    let wrap_and_spec = wrap_and_spec_fn(&call, ident, &func.sig.inputs, &func.sig.output);
    let compile_errors = errors.iter().map(Error::to_compile_error);
    quote! {
        #func
        #(#compile_errors)*
        #wrap_and_spec
    }
    .into()
}

#[proc_macro_attribute]
pub fn contractimpl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let imp = parse_macro_input!(input as ItemImpl);
    let is_trait = imp.trait_.is_some();
    let ty = &imp.self_ty;
    let wrap_and_specs = imp
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
            wrap_and_spec_fn(&call, ident, &m.sig.inputs, &m.sig.output)
        });
    quote! {
        #imp
        #(#wrap_and_specs)*
    }
    .into()
}

#[proc_macro_attribute]
pub fn contracttype(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    todo!()
}
