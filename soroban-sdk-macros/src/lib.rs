extern crate proc_macro;

mod derive_fn;
mod derive_type;
mod map_type;
mod syn_ext;

use derive_fn::{derive_contract_function_set, derive_fn};
use derive_type::{derive_type_enum, derive_type_struct};

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::{format_ident, quote};
use sha2::{Digest, Sha256};
use std::fs;
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, DeriveInput, Error, FnArg, ItemImpl,
    ItemTrait, Pat, Type, TypePath, Visibility,
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
    sha256: String,
}

#[proc_macro_attribute]
pub fn contractfile(metadata: TokenStream, input: TokenStream) -> TokenStream {
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
    if args.sha256 != sha256 {
        return Error::new(Span::call_site(), "sha256 does not match file".to_string())
            .into_compile_error()
            .into();
    }

    // Render bytes.
    let contents_lit = Literal::byte_string(&wasm);
    let input: proc_macro2::TokenStream = input.into();
    quote! {
        pub const WASM: &[u8] = #contents_lit;
        #input
    }
    .into()
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

    // Map the traits methods to methods for the Client.
    let mut errors = Vec::<Error>::new();
    let methods: Vec<_> = syn_ext::trait_methods(&trait_)
        .map(|m| {
            let fn_ident = &m.sig.ident;
            let fn_name = fn_ident.to_string();

            // Check for the Env argument.
            let env_input = m.sig.inputs.first().and_then(|a| match a {
                FnArg::Typed(pat_type) => {
                    let ty = &*pat_type.ty;
                    if let Type::Path(TypePath {
                        path: syn::Path { segments, .. },
                        ..
                    }) = ty
                    {
                        if segments.last().map_or(false, |s| s.ident == "Env") {
                            Some(a)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                FnArg::Receiver(_) => None,
            });

            // Map all remaining inputs.
            let (fn_input_names, fn_input_types): (Vec<_>, Vec<_>) = m.sig.inputs
                .iter()
                .skip(if env_input.is_some() { 1 } else { 0 })
                .enumerate().map(|(i, t)| {
                    match t {
                        FnArg::Typed(pat_type) => {
                            if let Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                                let ident = format_ident!("arg_{}", i);
                                let mut pat_type = pat_type.clone();
                                let mut pat_ident = pat_ident.clone();
                                pat_ident.ident = ident.clone();
                                pat_type.pat = Box::new(Pat::Ident(pat_ident));
                                (ident, FnArg::Typed(pat_type))
                            } else {
                                errors.push(Error::new(t.span(), "argument not supported"));
                                (format_ident!(""), t.clone())
                            }
                        }
                        _ => {
                            errors.push(Error::new(t.span(), "argument not supported"));
                            (format_ident!(""), t.clone())
                        }
                    }
                })
                .unzip();
            let fn_output = &m.sig.output;
            quote!{
                pub fn #fn_ident(env: &::soroban_sdk::Env, contract_id: &::soroban_sdk::BytesN<32>, #(#fn_input_types),*) #fn_output {
                    use ::soroban_sdk::IntoVal;
                    const FN_SYMBOL: ::soroban_sdk::Symbol = ::soroban_sdk::Symbol::from_str(#fn_name);
                    env.invoke_contract(contract_id, &FN_SYMBOL, ::soroban_sdk::vec![env, #(#fn_input_names.into_env_val(&env)),*])
                }
            }
        })
        .collect();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* }.into();
    }

    // Render the Client.
    let client_ident = format_ident!("{}", args.name);
    quote! {
        #trait_
        pub struct #client_ident;
        impl #client_ident { #(#methods)* }
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct ContractImportArgs {
    file: String,
    #[darling(default)]
    sha256: Option<String>,
}

#[proc_macro]
pub fn contractimport(metadata: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractImportArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    match soroban_spec::generate_from_file(&args.file, args.sha256.as_deref()) {
        Ok(code) => quote! { #code },
        Err(e) => Error::new(Span::call_site(), e.to_string()).into_compile_error(),
    }
    .into()
}
