extern crate proc_macro;

mod derive_fn;
mod derive_type;
mod map_type;
mod syn_ext;

use derive_fn::{derive_contract_function_set, derive_fn};
use derive_type::{derive_type_enum, derive_type_struct};

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use soroban_spec::wasm::GetSpecError;
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, DeriveInput, Error, ItemImpl, Visibility,
};

fn default_export() -> bool {
    true
}

#[derive(Debug, FromMeta)]
struct ContractImplArgs {
    #[darling(default = "default_export")]
    export: bool,
}

#[proc_macro_attribute]
pub fn contractimpl(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractImplArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
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
                ty,
                ident,
                &m.sig.inputs,
                &m.sig.output,
                args.export,
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

#[derive(Debug, FromMeta)]
struct ContractWasmArgs {
    #[darling(default)]
    wasm: String,
}

#[proc_macro_attribute]
pub fn contractwasm(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractWasmArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let consts = soroban_spec::wasm::generate_consts(&args.wasm);
    let input = parse_macro_input!(input as DeriveInput);
    quote! {
        #consts
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn contractclient(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // TODO: Implement client.
    quote! { #input }.into()
}

#[derive(Debug, FromMeta)]
struct ContractImportArgs {
    wasm: String,
}

#[proc_macro]
pub fn contractimport(metadata: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractImportArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let spec = match soroban_spec::wasm::get_spec(&args.wasm) {
        Ok(spec) => spec,
        Err(e) => {
            let err_str = match e {
                GetSpecError::Read(e) => e.to_string(),
                GetSpecError::LoadContract(e) => e.to_string(),
                GetSpecError::Parse(e) => e.to_string(),
                GetSpecError::NotFound => "spec not found".to_string(),
            };
            return Error::new(attr_args.first().unwrap().span(), err_str)
                .into_compile_error()
                .into();
        }
    };
    let types = soroban_spec::types::generate(&spec, &args.wasm);
    quote! { #types }.into()
}
