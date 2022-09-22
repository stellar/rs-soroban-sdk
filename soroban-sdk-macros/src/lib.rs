extern crate proc_macro;

mod derive_client;
mod derive_enum;
mod derive_enum_int;
mod derive_error_enum_int;
mod derive_fn;
mod derive_struct;
mod map_type;
mod path;
mod syn_ext;

use derive_client::derive_client;
use derive_enum::derive_type_enum;
use derive_enum_int::derive_type_enum_int;
use derive_error_enum_int::derive_type_error_enum_int;
use derive_fn::{derive_contract_function_set, derive_fn};
use derive_struct::derive_type_struct;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::quote;
use sha2::{Digest, Sha256};
use std::fs;
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, DeriveInput, Error, ItemImpl, LitStr, Type,
    Visibility,
};

use self::derive_client::ClientItem;

use soroban_spec::gen::rust::{generate_from_wasm, GenerateFromFileError};

use soroban_env_common::Symbol;

#[proc_macro]
pub fn symbol(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    match Symbol::try_from_str(&input.value()) {
        Ok(_) => quote! {{
            const symbol: ::soroban_sdk::Symbol = ::soroban_sdk::Symbol::from_str(#input);
            symbol
        }}
        .into(),
        Err(e) => Error::new(input.span(), format!("{}", e))
            .to_compile_error()
            .into(),
    }
}

/// Exports the publicly accessible functions in the implementation.
///
/// Functions that are publicly accessible in the implementation are invocable
/// by other contracts, or directly by transactions, when deployed.
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
                #[::soroban_sdk::contractclient(name = #client_ident)]
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
    lib: Option<String>,
    export: Option<bool>,
}

/// Generates conversions from the struct/enum from/into a `RawVal`.
///
/// There are some constraints on the types that are supported:
/// - Enums with integer values must have an explicit integer literal for every
/// variant.
/// - Enums with unit variants are supported.
/// - Enums with tuple-like variants with a maximum of one tuple field are
/// supported. The tuple field must be of a type that is also convertible to and
/// from `RawVal`.
/// - Enums with struct-like variants are not supported.
/// - Structs are supported. All fields must be of a type that is also
/// convertible to and from `RawVal`.
/// - All variant names, field names, and type names must be 10-characters or
/// less in length.
///
/// Includes the type in the contract spec so that clients can generate bindings
/// for the type.
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
        syn::Data::Struct(s) => derive_type_struct(ident, s, gen_spec, &args.lib),
        syn::Data::Enum(e) => {
            let count_of_variants = e.variants.len();
            let count_of_int_variants = e
                .variants
                .iter()
                .filter(|v| v.discriminant.is_some())
                .count();
            if count_of_int_variants == 0 {
                derive_type_enum(ident, e, gen_spec, &args.lib)
            } else if count_of_int_variants == count_of_variants {
                derive_type_enum_int(ident, e, gen_spec, &args.lib)
            } else {
                Error::new(input.span(), "enums are supported as contract types only when all variants have an explicit integer literal, or when all variants are unit or single field")
                    .to_compile_error()
            }
        }
        syn::Data::Union(u) => Error::new(
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

/// Generates conversions from the repr(u32) enum from/into a `Status`.
///
/// There are some constraints on the types that are supported:
/// - Enum must derive `Copy`.
/// - Enum variants must have an explicit integer literal.
/// - Enum variants must have a value convertible to u32.
///
/// Includes the type in the contract spec so that clients can generate bindings
/// for the type.
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
        syn::Data::Enum(e) => {
            if e.variants.iter().all(|v| v.discriminant.is_some()) {
                derive_type_error_enum_int(ident, e, gen_spec, &args.lib)
            } else {
                Error::new(input.span(), "enums are supported as contract errors only when all variants have an explicit integer literal")
                    .to_compile_error()
            }
        }
        syn::Data::Struct(s) => Error::new(
            s.struct_token.span(),
            "structs are unsupported as contract errors",
        )
        .to_compile_error(),
        syn::Data::Union(u) => Error::new(
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

#[doc(hidden)]
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

#[doc(hidden)]
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

/// Import a contract from its WASM file.
///
/// Generates in the current module:
/// - A `Contract` trait that matches the contracts interface.
/// - A `ContractClient` struct that has functions for each function in the
/// contract.
/// - Types for all contract types defined in the contract.
///
/// ### Examples
///
/// ```ignore
/// use soroban_sdk::{BytesN, Env, Symbol};
///
/// mod contract_a {
///     soroban_sdk::contractimport!(file = "contract_a.wasm");
/// }
///
/// pub struct ContractB;
///
/// #[contractimpl]
/// impl ContractB {
///     pub fn add_with(env: Env, contract_id: BytesN<32>, x: u32, y: u32) -> u32 {
///         let client = contract_a::ContractClient::new(&env, contract_id);
///         client.add(&x, &y)
///     }
/// }
///
/// #[test]
/// fn test() {
///     let env = Env::default();
///
///     // Define IDs for contract A and B.
///     let contract_a_id = BytesN::from_array(&env, &[0; 32]);
///     let contract_b_id = BytesN::from_array(&env, &[1; 32]);
///
///     // Register contract A using the imported WASM.
///     env.register_contract_wasm(&contract_a_id, contract_a::WASM);
///
///     // Register contract B defined in this crate.
///     env.register_contract(&contract_b_id, ContractB);
///
///     // Create a client for calling contract B.
///     let client = ContractBClient::new(&env, &contract_b_id);
///
///     // Invoke contract B via its client.
///     let sum = client.add_with(&contract_a_id, &5, &7);
///     assert_eq!(sum, 12);
/// }
/// ```
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
