extern crate proc_macro;

mod derive_client;
mod derive_enum;
mod derive_enum_int;
mod derive_error_enum_int;
mod derive_fn;
mod derive_struct;
mod derive_struct_tuple;
mod map_type;
mod path;
mod syn_ext;

use derive_client::derive_client;
use derive_enum::derive_type_enum;
use derive_enum_int::derive_type_enum_int;
use derive_error_enum_int::derive_type_error_enum_int;
use derive_fn::{derive_contract_function_set, derive_fn};
use derive_struct::derive_type_struct;
use derive_struct_tuple::derive_type_struct_tuple;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::quote;
use sha2::{Digest, Sha256};
use std::fs;
use syn::{
    parse_macro_input, parse_str, spanned::Spanned, AttributeArgs, Data, DeriveInput, Error,
    Fields, ItemImpl, LitStr, Path, Type, Visibility,
};

use self::derive_client::ClientItem;

use soroban_spec::gen::rust::{generate_from_wasm, GenerateFromFileError};

use soroban_env_common::Symbol;

fn default_crate_path() -> Path {
    parse_str("soroban_sdk").unwrap()
}

#[proc_macro]
pub fn symbol(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    match Symbol::try_from_str(&input.value()) {
        Ok(_) => quote! {{
            const symbol: soroban_sdk::Symbol = soroban_sdk::Symbol::from_str(#input);
            symbol
        }}
        .into(),
        Err(e) => Error::new(input.span(), format!("{}", e))
            .to_compile_error()
            .into(),
    }
}

#[proc_macro_attribute]
pub fn contractimpl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let imp = parse_macro_input!(input as ItemImpl);
    let ty = &imp.self_ty;

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
            let trait_ident = imp.trait_.as_ref().and_then(|x| x.1.get_ident());
            derive_fn(
                &call,
                ty,
                ident,
                &m.attrs,
                &m.sig.inputs,
                &m.sig.output,
                trait_ident,
                &client_ident,
            )
        })
        .collect();

    match derived {
        Ok(derived_ok) => {
            let cfs = derive_contract_function_set(ty, pub_methods.into_iter());
            quote! {
                #[soroban_sdk::contractclient(name = #client_ident)]
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

#[derive(Debug, FromMeta)]
struct ContractTypeArgs {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    lib: Option<String>,
    export: Option<bool>,
}

#[proc_macro_attribute]
pub fn contracttype(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractTypeArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    // If the export argument has a value, do as it instructs regarding
    // exporting. If it does not have a value, export if the type is pub.
    let gen_spec = if let Some(export) = args.export {
        export
    } else {
        matches!(input.vis, Visibility::Public(_))
    };
    let derived = match &input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(_) => derive_type_struct(&args.crate_path, ident, s, gen_spec, &args.lib),
            Fields::Unnamed(_) => {
                derive_type_struct_tuple(&args.crate_path, ident, s, gen_spec, &args.lib)
            }
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
                derive_type_enum(&args.crate_path, ident, e, gen_spec, &args.lib)
            } else if count_of_int_variants == count_of_variants {
                derive_type_enum_int(&args.crate_path, ident, e, gen_spec, &args.lib)
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
    let args = parse_macro_input!(metadata as AttributeArgs);
    let args = match ContractTypeArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
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
                derive_type_error_enum_int(&args.crate_path, ident, e, gen_spec, &args.lib)
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
    let args = parse_macro_input!(metadata as AttributeArgs);
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
    let input2: TokenStream2 = input.clone().into();
    let item = parse_macro_input!(input as ClientItem);
    let methods: Vec<_> = item.fns();
    let client = derive_client(&args.name, &methods);
    quote! {
        #input2
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
