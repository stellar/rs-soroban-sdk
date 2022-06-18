extern crate proc_macro;

mod wrap_and_spec_fn;
use wrap_and_spec_fn::wrap_and_spec_fn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemFn, ItemImpl, Visibility};

// TODO: Investigate how to make the multiple spec statics be joined into a
// variable length XDR array instead of being a stream of XDR values.

#[proc_macro_attribute]
#[allow(clippy::missing_panics_doc)]
pub fn contractfn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let ident = &func.sig.ident;
    let call = quote! { #ident };
    let wrap_and_spec = wrap_and_spec_fn(&call, ident, &func.sig.inputs, &func.sig.output);
    quote! {
        #func
        #wrap_and_spec
    }
    .into()
}

#[proc_macro_attribute]
#[allow(clippy::missing_panics_doc)]
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
