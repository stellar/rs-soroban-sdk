use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::{ScSymbol, StringM, SCSYMBOL_LIMIT};
use syn::TypeReference;
use syn::{
    ext::IdentExt as _, punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Error,
    FnArg, Ident, Pat, Path, ReturnType, Type, TypePath,
};

use crate::attribute::pass_through_attr_to_gen_code;
use crate::syn_ext::{self, ty_to_safe_ident_str};
use crate::{
    doc::docs_from_attrs,
    spec::{const_view_string, const_view_symbol, const_view_type_def},
};

pub fn derive_fns_spec<'a>(
    path: &Path,
    ty: &Type,
    fns: impl IntoIterator<Item = &'a syn_ext::Fn>,
    export: bool,
) -> Result<TokenStream2, TokenStream2> {
    fns.into_iter()
        .map(|f| derive_fn_spec(path, ty, &f.ident, &f.attrs, &f.inputs, &f.output, export))
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub fn derive_fn_spec(
    path: &Path,
    ty: &Type,
    ident: &Ident,
    attrs: &[Attribute],
    inputs: &Punctuated<FnArg, Comma>,
    output: &ReturnType,
    export: bool,
) -> Result<TokenStream2, TokenStream2> {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    // Prepare the env input.
    let env_input = inputs.first().and_then(|a| match a {
        FnArg::Typed(pat_type) => {
            let mut ty = &*pat_type.ty;
            if let Type::Reference(TypeReference { elem, .. }) = ty {
                ty = elem;
            }
            if let Type::Path(TypePath {
                path: syn::Path { segments, .. },
                ..
            }) = ty
            {
                if segments.last().map_or(false, |s| s.ident == "Env") {
                    Some(())
                } else {
                    None
                }
            } else {
                None
            }
        }
        FnArg::Receiver(_) => None,
    });

    // Prepare the argument inputs.
    let (spec_args, arg_types): (Vec<_>, Vec<_>) = inputs
        .iter()
        .skip(if env_input.is_some() { 1 } else { 0 })
        .map(|a| match a {
            FnArg::Typed(pat_type) => {
                let name = if let Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                    pat_ident.ident.unraw().to_string()
                } else {
                    errors.push(Error::new(a.span(), "argument not supported"));
                    "".to_string()
                };

                let ty = &*pat_type.ty;

                // Strip any underscore prefix characters. Implementations that do not use an
                // argument will prefix an underscore to the variable name to signal to the
                // compiler that the developer acknowledges they will not be using the parameter.
                // Keeping the underscore out of the spec ensures that the spec doesn't communicate
                // implementation details and doesn't change when implementations start or stop
                // using a variable in the implementation. It also ensures spec consistency between
                // implementations of the same trait even if some of those implementations do not
                // use all the inputs.
                let name = name.trim_start_matches("_");

                // The input's name, as the spec holds it. Its spec type comes from
                // the Rust type.
                let name = name.try_into().unwrap_or_else(|_| {
                    const MAX: u32 = 30;
                    errors.push(Error::new(
                        a.span(),
                        format!("argument name too long, max length {} characters", MAX),
                    ));
                    StringM::<MAX>::default()
                });
                (name, ty)
            }
            FnArg::Receiver(_) => {
                errors.push(Error::new(a.span(), "self argument not supported"));
                (StringM::default(), ty)
            }
        })
        .collect();

    // Prepare the output.
    let spec_result = match output {
        ReturnType::Type(_, ty) => Some(ty.as_ref()),
        ReturnType::Default => None,
    };

    // Generated code spec.
    let name = &ident.unraw().to_string();
    let fn_name: ScSymbol = name.try_into().unwrap_or_else(|_| {
        errors.push(Error::new(
            ident.span(),
            format!(
                "contract function name is too long: {}, max is {}",
                name.len(),
                SCSYMBOL_LIMIT,
            ),
        ));
        ScSymbol::default()
    });

    // The spec entry rendered as the equivalent const::ScSpecEntry, which the
    // contract crate encodes to XDR at compile time.
    let spec_entry = {
        let doc = const_view_string(path, &docs_from_attrs(attrs));
        let name = const_view_symbol(path, &fn_name);
        let inputs = spec_args
            .iter()
            .zip(arg_types.iter().copied())
            .map(|(input_name, rust)| {
                let doc = const_view_string(path, &StringM::<1024>::default());
                let name = const_view_string(path, input_name);
                let type_ = const_view_type_def(path, rust);
                quote!(#path::xdr::r#const::ScSpecFunctionInputV0 { doc: #doc, name: #name, type_: #type_ })
            });
        let outputs = spec_result.iter().map(|o| const_view_type_def(path, o));
        quote! {
            #path::xdr::r#const::ScSpecEntry::FunctionV0(#path::xdr::r#const::ScSpecFunctionV0 {
                doc: #doc,
                name: #name,
                inputs: #path::xdr::r#const::VecM::try_from_slice_or_panic(&[#(#inputs),*]),
                outputs: #path::xdr::r#const::VecM::try_from_slice_or_panic(&[#(#outputs),*]),
            })
        }
    };
    // Of the four idents derived from the fn name, only #spec_ident converts to upper case. There's
    // no reason for it to do that, other than historical. It has no consequence as nothing can
    // collide with it anyway, because it is only used inside #hidden_mod_ident. The other three
    // (spec_entry_ident, spec_fn_ident, hidden_mod_ident) are generated in a scope shared with
    // every other fn, so they need to retain the original case to be unique.
    let spec_ident = format_ident!("__SPEC_XDR_FN_{}", ident.unraw().to_string().to_uppercase());
    let spec_entry_ident = format_ident!("__SPEC_XDR_ENTRY_{}", ident);
    let spec_fn_ident = format_ident!("spec_xdr_{}", ident);

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return Err(quote! { #(#compile_errors)* });
    }

    // Filter attributes to those that should be passed through to the generated code.
    let attrs = attrs
        .iter()
        .filter(|attr| pass_through_attr_to_gen_code(attr))
        .collect::<Vec<_>>();

    let ty_str = ty_to_safe_ident_str(ty);
    let hidden_mod_ident = format_ident!("__{}__{}__spec", ty_str, ident);
    let exported = if export {
        Some(quote! {
            #[doc(hidden)]
            #(#attrs)*
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            mod #hidden_mod_ident {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[allow(non_upper_case_globals)]
                #[allow(dead_code)]
                #(#attrs)*
                #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
                static #spec_ident: [u8; super::#ty::#spec_fn_ident().len()] = super::#ty::#spec_fn_ident();
            }
        })
    } else {
        None
    };

    // Generated code.
    Ok(quote! {
        #exported

        impl #ty {
            #[allow(non_upper_case_globals)]
            #(#attrs)*
            const #spec_entry_ident: #path::xdr::r#const::ScSpecEntry = #spec_entry;

            #[allow(non_snake_case)]
            #(#attrs)*
            pub const fn #spec_fn_ident() -> [u8; #ty::#spec_entry_ident.const_xdr_len()] {
                const { #ty::#spec_entry_ident.const_to_xdr() }
            }
        }
    })
}
