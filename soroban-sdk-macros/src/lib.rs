extern crate proc_macro;

mod derive_client;
mod derive_fn;
mod derive_type;
mod map_type;
mod syn_ext;

use derive_client::derive_client;
use derive_fn::{derive_contract_function_set, derive_fn};
use derive_type::{derive_type_enum, derive_type_struct};

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use sha2::{Digest, Sha256};
use soroban_spec::gen::rust::{generate_from_file, GenerateFromFileError};
use std::fs;
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, DeriveInput, Error, ItemImpl, ItemTrait,
    Visibility,
};

#[derive(Debug, FromMeta)]
struct ContractImplArgs {
    #[darling(default = "contractimpl_args_default_export")]
    export: bool,
}

fn contractimpl_args_default_export() -> bool {
    true
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
struct ContractFileArgs {
    file: String,
    sha256: darling::util::SpannedValue<String>,
}

#[proc_macro]
pub fn contractfile(metadata: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractFileArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    // Read WASM from file.
    let wasm = match fs::read(&args.file) {
        Ok(wasm) => wasm,
        Err(e) => {
            return Error::new(Span::call_site(), e.to_string())
                .into_compile_error()
                .into()
        }
    };

    // Verify SHA256 hash.
    let sha256 = Sha256::digest(&wasm);
    let sha256 = format!("{:x}", sha256);
    if *args.sha256 != sha256 {
        return Error::new(
            args.sha256.span(),
            format!("sha256 does not match, expected: {}", sha256),
        )
        .into_compile_error()
        .into();
    }

    // Render bytes.
    let contents_lit = Literal::byte_string(&wasm);
    quote! { #contents_lit }.into()
}

#[derive(Debug, FromMeta)]
struct ContractClientArgs {
    name: String,
}

#[proc_macro_attribute]
pub fn contractclient(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractClientArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let trait_ = parse_macro_input!(input as ItemTrait);
    let methods: Vec<_> = syn_ext::trait_methods(&trait_).collect();
    let client = derive_client(&args.name, &methods);
    quote! {
        #trait_
        #client
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct ContractImportArgs {
    file: String,
    #[darling(default)]
    sha256: darling::util::SpannedValue<Option<String>>,
}

#[proc_macro]
pub fn contractimport(metadata: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractImportArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    match generate_from_file(&args.file, args.sha256.as_deref()) {
        Ok(code) => quote! { #code },
        Err(e @ GenerateFromFileError::VerifySha256 { .. }) => {
            Error::new(args.sha256.span(), e.to_string()).into_compile_error()
        }
        Err(e) => Error::new(Span::call_site(), e.to_string()).into_compile_error(),
    }
    .into()
}
