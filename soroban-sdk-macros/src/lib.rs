extern crate proc_macro;

mod arbitrary;
mod attribute;
mod derive_args;
mod derive_client;
mod derive_enum;
mod derive_enum_int;
mod derive_error_enum_int;
mod derive_fn;
mod derive_spec_fn;
mod derive_struct;
mod derive_struct_tuple;
mod doc;
mod map_type;
mod path;
mod symbol;
mod syn_ext;

use derive_args::{derive_args_impl, derive_args_type};
use derive_client::{derive_client_impl, derive_client_type};
use derive_enum::derive_type_enum;
use derive_enum_int::derive_type_enum_int;
use derive_error_enum_int::derive_type_error_enum_int;
use derive_fn::{derive_contract_function_registration_ctor, derive_pub_fn};
use derive_spec_fn::derive_fn_spec;
use derive_struct::derive_type_struct;
use derive_struct_tuple::derive_type_struct_tuple;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use sha2::{Digest, Sha256};
use std::{fmt::Write, fs};
use syn::{
    parse_macro_input, parse_str, spanned::Spanned, Data, DeriveInput, Error, Fields, ItemImpl,
    ItemStruct, LitStr, Path, Type, Visibility,
};
use syn_ext::HasFnsItem;

use soroban_spec_rust::{generate_from_wasm, GenerateFromFileError};

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{Limits, ScMetaEntry, ScMetaV0, StringM, WriteXdr};

pub(crate) const DEFAULT_XDR_RW_LIMITS: Limits = Limits {
    depth: 500,
    len: 0x1000000,
};

#[proc_macro]
pub fn internal_symbol_short(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let crate_path: Path = syn::parse_str("crate").unwrap();
    symbol::short(&crate_path, &input).into()
}

#[proc_macro]
pub fn symbol_short(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let crate_path: Path = syn::parse_str("soroban_sdk").unwrap();
    symbol::short(&crate_path, &input).into()
}

fn default_crate_path() -> Path {
    parse_str("soroban_sdk").unwrap()
}

#[derive(Debug, FromMeta)]
struct ContractSpecArgs {
    name: Type,
    export: Option<bool>,
}

#[proc_macro_attribute]
pub fn contractspecfn(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractSpecArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let input2: TokenStream2 = input.clone().into();
    let item = parse_macro_input!(input as HasFnsItem);
    let methods: Vec<_> = item.fns();
    let export = args.export.unwrap_or(true);

    let derived: Result<proc_macro2::TokenStream, proc_macro2::TokenStream> = methods
        .iter()
        .map(|m| derive_fn_spec(&args.name, m.ident, m.attrs, m.inputs, m.output, export))
        .collect();

    match derived {
        Ok(derived_ok) => quote! {
            #input2
            #derived_ok
        }
        .into(),
        Err(derived_err) => quote! {
            #input2
            #derived_err
        }
        .into(),
    }
}

#[derive(Debug, FromMeta)]
struct ContractArgs {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
}

#[proc_macro_attribute]
pub fn contract(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let input2: TokenStream2 = input.clone().into();

    let item = parse_macro_input!(input as ItemStruct);

    let ty = &item.ident;
    let ty_str = quote!(#ty).to_string();

    let client_ident = format!("{ty_str}Client");
    let fn_set_registry_ident = format_ident!("__{}_fn_set_registry", ty_str.to_lowercase());
    let crate_path = &args.crate_path;
    let client = derive_client_type(&args.crate_path, &ty_str, &client_ident);
    let args_ident = format!("{ty_str}Args");
    let contract_args = derive_args_type(&ty_str, &args_ident);
    let mut output = quote! {
        #input2
        #contract_args
        #client
    };
    if cfg!(feature = "testutils") {
        output.extend(quote! {
            mod #fn_set_registry_ident {
                use super::*;

                extern crate std;
                use std::sync::Mutex;
                use std::collections::BTreeMap;

                pub type F = #crate_path::testutils::ContractFunctionF;

                static FUNCS: Mutex<BTreeMap<&'static str, &'static F>> = Mutex::new(BTreeMap::new());

                pub fn register(name: &'static str, func: &'static F) {
                    FUNCS.lock().unwrap().insert(name, func);
                }

                pub fn call(name: &str, env: #crate_path::Env, args: &[#crate_path::Val]) -> Option<#crate_path::Val> {
                    let fopt: Option<&'static F> = FUNCS.lock().unwrap().get(name).map(|f| f.clone());
                    fopt.map(|f| f(env, args))
                }
            }

            impl #crate_path::testutils::ContractFunctionRegister for #ty {
                fn register(name: &'static str, func: &'static #fn_set_registry_ident::F) {
                    #fn_set_registry_ident::register(name, func);
                }
            }

            #[doc(hidden)]
            impl #crate_path::testutils::ContractFunctionSet for #ty {
                fn call(&self, func: &str, env: #crate_path::Env, args: &[#crate_path::Val]) -> Option<#crate_path::Val> {
                    #fn_set_registry_ident::call(func, env, args)
                }
            }
        });
    }
    output.into()
}

#[derive(Debug, FromMeta)]
struct ContractImplArgs {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
}

#[proc_macro_attribute]
pub fn contractimpl(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractImplArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let crate_path = &args.crate_path;
    let crate_path_str = quote!(#crate_path).to_string();

    let imp = parse_macro_input!(input as ItemImpl);
    let trait_ident = imp.trait_.as_ref().and_then(|x| x.1.get_ident());
    let ty = &imp.self_ty;
    let ty_str = quote!(#ty).to_string();

    // TODO: Use imp.trait_ in generating the args ident, to create a unique
    // args for each trait impl for a contract, to avoid conflicts.
    let args_ident = if let Type::Path(path) = &**ty {
        path.path
            .segments
            .last()
            .map(|name| format!("{}Args", name.ident))
    } else {
        None
    }
    .unwrap_or_else(|| "Args".to_string());

    // TODO: Use imp.trait_ in generating the client ident, to create a unique
    // client for each trait impl for a contract, to avoid conflicts.
    let client_ident = if let Type::Path(path) = &**ty {
        path.path
            .segments
            .last()
            .map(|name| format!("{}Client", name.ident))
    } else {
        None
    }
    .unwrap_or_else(|| "Client".to_string());

    let pub_methods: Vec<_> = syn_ext::impl_pub_methods(&imp).collect();
    let derived: Result<proc_macro2::TokenStream, proc_macro2::TokenStream> = pub_methods
        .iter()
        .map(|m| {
            let ident = &m.sig.ident;
            let call = quote! { <super::#ty>::#ident };
            derive_pub_fn(
                crate_path,
                &call,
                ident,
                &m.attrs,
                &m.sig.inputs,
                trait_ident,
                &client_ident,
            )
        })
        .collect();

    match derived {
        Ok(derived_ok) => {
            let mut output = quote! {
                #[#crate_path::contractargs(name = #args_ident, impl_only = true)]
                #[#crate_path::contractclient(crate_path = #crate_path_str, name = #client_ident, impl_only = true)]
                #[#crate_path::contractspecfn(name = #ty_str)]
                #imp
                #derived_ok
            };
            let cfs = derive_contract_function_registration_ctor(
                crate_path,
                ty,
                trait_ident,
                pub_methods.into_iter(),
            );
            output.extend(quote! { #cfs });
            output.into()
        }
        Err(derived_err) => quote! {
            #imp
            #derived_err
        }
        .into(),
    }
}

#[proc_macro]
pub fn contractmetabuiltin(_metadata: TokenStream) -> TokenStream {
    // The following two lines assume that the soroban-sdk-macros crate always
    // has the same version as the soroban-sdk, and lives in the same
    // repository.
    let rustc_version = env!("RUSTC_VERSION");
    let sdk_pkg_version = env!("CARGO_PKG_VERSION");
    let sdk_git_revision = env!("GIT_REVISION");
    let sdk_version = format!("{sdk_pkg_version}#{sdk_git_revision}");
    quote! {
        contractmeta!(
            // Rustc version.
            key = "rsver",
            val = #rustc_version,
        );
        contractmeta!(
            // Rust Soroban SDK version.
            key = "rssdkver",
            val = #sdk_version,
        );
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct MetadataArgs {
    key: String,
    val: String,
}

#[proc_macro]
pub fn contractmeta(metadata: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match MetadataArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let gen = {
        let key: StringM = match args.key.clone().try_into() {
            Ok(k) => k,
            Err(e) => {
                return Error::new(Span::call_site(), e.to_string())
                    .into_compile_error()
                    .into()
            }
        };

        let val: StringM = match args.val.try_into() {
            Ok(k) => k,
            Err(e) => {
                return Error::new(Span::call_site(), e.to_string())
                    .into_compile_error()
                    .into()
            }
        };

        let meta_v0 = ScMetaV0 { key, val };
        let meta_entry = ScMetaEntry::ScMetaV0(meta_v0);
        let metadata_xdr: Vec<u8> = match meta_entry.to_xdr(DEFAULT_XDR_RW_LIMITS) {
            Ok(v) => v,
            Err(e) => {
                return Error::new(Span::call_site(), e.to_string())
                    .into_compile_error()
                    .into()
            }
        };

        let metadata_xdr_lit = proc_macro2::Literal::byte_string(metadata_xdr.as_slice());
        let metadata_xdr_len = metadata_xdr.len();

        let ident = format_ident!(
            "__CONTRACT_KEY_{}",
            args.key.as_bytes().iter().fold(String::new(), |mut s, b| {
                let _ = write!(s, "{b:02x}");
                s
            })
        );
        quote! {
            #[doc(hidden)]
            #[cfg_attr(target_family = "wasm", link_section = "contractmetav0")]
            static #ident: [u8; #metadata_xdr_len] = *#metadata_xdr_lit;
        }
    };

    quote! {
        #gen
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct ContractTypeArgs {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    lib: Option<String>,
    export: Option<bool>,
}

#[proc_macro_attribute]
pub fn contracttype(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractTypeArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let input = parse_macro_input!(input as DeriveInput);
    let vis = &input.vis;
    let ident = &input.ident;
    let attrs = &input.attrs;
    // If the export argument has a value, do as it instructs regarding
    // exporting. If it does not have a value, export if the type is pub.
    let gen_spec = if let Some(export) = args.export {
        export
    } else {
        matches!(input.vis, Visibility::Public(_))
    };
    let derived = match &input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(_) => {
                derive_type_struct(&args.crate_path, vis, ident, attrs, s, gen_spec, &args.lib)
            }
            Fields::Unnamed(_) => derive_type_struct_tuple(
                &args.crate_path,
                vis,
                ident,
                attrs,
                s,
                gen_spec,
                &args.lib,
            ),
            Fields::Unit => Error::new(
                s.fields.span(),
                "unit structs are not supported as contract types",
            )
            .to_compile_error(),
        },
        Data::Enum(e) => {
            let count_of_variants = e.variants.len();
            let count_of_int_variants = e
                .variants
                .iter()
                .filter(|v| v.discriminant.is_some())
                .count();
            if count_of_int_variants == 0 {
                derive_type_enum(&args.crate_path, vis, ident, attrs, e, gen_spec, &args.lib)
            } else if count_of_int_variants == count_of_variants {
                derive_type_enum_int(&args.crate_path, vis, ident, attrs, e, gen_spec, &args.lib)
            } else {
                Error::new(input.span(), "enums are supported as contract types only when all variants have an explicit integer literal, or when all variants are unit or single field")
                    .to_compile_error()
            }
        }
        Data::Union(u) => Error::new(
            u.union_token.span(),
            "unions are unsupported as contract types",
        )
        .to_compile_error(),
    };
    quote! {
        #input
        #derived
    }
    .into()
}

#[proc_macro_attribute]
pub fn contracterror(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractTypeArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let attrs = &input.attrs;
    // If the export argument has a value, do as it instructs regarding
    // exporting. If it does not have a value, export if the type is pub.
    let gen_spec = if let Some(export) = args.export {
        export
    } else {
        matches!(input.vis, Visibility::Public(_))
    };
    let derived = match &input.data {
        Data::Enum(e) => {
            if e.variants.iter().all(|v| v.discriminant.is_some()) {
                derive_type_error_enum_int(&args.crate_path, ident, attrs, e, gen_spec, &args.lib)
            } else {
                Error::new(input.span(), "enums are supported as contract errors only when all variants have an explicit integer literal")
                    .to_compile_error()
            }
        }
        Data::Struct(s) => Error::new(
            s.struct_token.span(),
            "structs are unsupported as contract errors",
        )
        .to_compile_error(),
        Data::Union(u) => Error::new(
            u.union_token.span(),
            "unions are unsupported as contract errors",
        )
        .to_compile_error(),
    };
    quote! {
        #input
        #derived
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct ContractFileArgs {
    file: String,
    sha256: darling::util::SpannedValue<String>,
}

#[proc_macro]
pub fn contractfile(metadata: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractFileArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    // Read WASM from file.
    let file_abs = path::abs_from_rel_to_manifest(&args.file);
    let wasm = match fs::read(file_abs) {
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
struct ContractArgsArgs {
    name: String,
    #[darling(default)]
    impl_only: bool,
}

#[proc_macro_attribute]
pub fn contractargs(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractArgsArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let input2: TokenStream2 = input.clone().into();
    let item = parse_macro_input!(input as HasFnsItem);
    let methods: Vec<_> = item.fns();
    let args_type = (!args.impl_only).then(|| derive_args_type(&item.name(), &args.name));
    let args_impl = derive_args_impl(&args.name, &methods);
    quote! {
        #input2
        #args_type
        #args_impl
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct ContractClientArgs {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    name: String,
    #[darling(default)]
    impl_only: bool,
}

#[proc_macro_attribute]
pub fn contractclient(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractClientArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let input2: TokenStream2 = input.clone().into();
    let item = parse_macro_input!(input as HasFnsItem);
    let methods: Vec<_> = item.fns();
    let client_type =
        (!args.impl_only).then(|| derive_client_type(&args.crate_path, &item.name(), &args.name));
    let client_impl = derive_client_impl(&args.crate_path, &args.name, &methods);
    quote! {
        #input2
        #client_type
        #client_impl
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
    let args = match NestedMeta::parse_meta_list(metadata.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractImportArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    // Read WASM from file.
    let file_abs = path::abs_from_rel_to_manifest(&args.file);
    let wasm = match fs::read(file_abs) {
        Ok(wasm) => wasm,
        Err(e) => {
            return Error::new(Span::call_site(), e.to_string())
                .into_compile_error()
                .into()
        }
    };

    // Generate.
    match generate_from_wasm(&wasm, &args.file, args.sha256.as_deref()) {
        Ok(code) => quote! { #code },
        Err(e @ GenerateFromFileError::VerifySha256 { .. }) => {
            Error::new(args.sha256.span(), e.to_string()).into_compile_error()
        }
        Err(e) => Error::new(Span::call_site(), e.to_string()).into_compile_error(),
    }
    .into()
}
