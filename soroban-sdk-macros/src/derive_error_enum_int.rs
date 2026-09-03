use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::{ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0, StringM};
use syn::{
    ext::IdentExt as _, spanned::Spanned, Attribute, DataEnum, Error, ExprLit, Ident, Lit, Path,
};

use crate::{
    doc::docs_from_attrs,
    map_type::{const_view_string, spec_name_gen},
    shaking,
};

pub fn derive_type_error_enum_int(
    path: &Path,
    enum_ident: &Ident,
    attrs: &[Attribute],
    data: &DataEnum,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let variants = &data.variants;
    let (spec_cases, try_froms, into_errors, into_invoke_errors): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let name = &ident.unraw().to_string();
            let discriminant: u32 = if let syn::Expr::Lit(ExprLit {
                lit: Lit::Int(ref lit_int),
                ..
            }) = v.discriminant.as_ref().unwrap().1
            {
                lit_int.base10_parse().unwrap_or_else(|_| {
                    errors.push(Error::new(
                        lit_int.span(),
                        "unsupported discriminant value on enum variant, must be parseable as u32",
                    ));
                    0
                })
            } else {
                errors.push(Error::new(
                    v.discriminant.as_ref().unwrap().1.span(),
                    "unsupported discriminant value on enum variant",
                ));
                0
            };
            let spec_case = ScSpecUdtErrorEnumCaseV0 {
                doc: docs_from_attrs(&v.attrs),
                name: name.try_into().unwrap_or_else(|_| StringM::default()),
                value: discriminant,
            };
            let try_from = quote! { #discriminant => Self::#ident };
            let into_error =
                quote! { #enum_ident::#ident => #path::Error::from_contract_error(#discriminant) };
            let into_invoke_error =
                quote! { #enum_ident::#ident => #path::InvokeError::Contract(#discriminant) };
            (spec_case, try_from, into_error, into_invoke_error)
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Build the spec entry once.
    let spec = ScSpecUdtErrorEnumV0 {
        doc: docs_from_attrs(attrs),
        // set to empty string always because the field is no longer used
        lib: StringM::default(),
        name: enum_ident.unraw().to_string().try_into().unwrap(),
        cases: spec_cases.try_into().unwrap(),
    };

    // Generated code spec. The spec entry is rendered as the equivalent const
    // ScSpecEntryView, which the contract crate encodes to XDR at compile time.
    // The fully qualified name the spec knows this type by, emitted for every
    // type so that a reference to it from anywhere can reach it.
    let spec_name = spec_name_gen(enum_ident, None, None, None);

    let spec_gen = {
        let doc = const_view_string(path, &spec.doc);
        let lib = const_view_string(path, &spec.lib);
        let name = quote!(#path::xdr::StringMView::try_from_str_or_panic(#enum_ident::spec_name()));
        let cases = spec.cases.iter().map(|c| {
            let doc = const_view_string(path, &c.doc);
            let name = const_view_string(path, &c.name);
            let value = c.value;
            quote!(#path::xdr::ScSpecUdtErrorEnumCaseV0View { doc: #doc, name: #name, value: #value })
        });
        let spec_view = quote! {
            #path::xdr::ScSpecEntryView::UdtErrorEnumV0(#path::xdr::ScSpecUdtErrorEnumV0View {
                doc: #doc,
                lib: #lib,
                name: #name,
                cases: #path::xdr::VecMView::try_from_slice_or_panic(&[#(#cases),*]),
            })
        };
        let spec_ident = format_ident!(
            "__SPEC_XDR_TYPE_{}",
            enum_ident.unraw().to_string().to_uppercase()
        );
        quote! {
            #[doc(hidden)]
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #enum_ident::spec_xdr_len()] = #enum_ident::spec_xdr();

            impl #enum_ident {
                const __SPEC_XDR_ENTRY: #path::xdr::ScSpecEntryView<'static> = #spec_view;

                pub const fn spec_xdr_len() -> usize {
                    const { #enum_ident::__SPEC_XDR_ENTRY.const_xdr_len() }
                }

                pub const fn spec_xdr() -> [u8; #enum_ident::spec_xdr_len()] {
                    const { #enum_ident::__SPEC_XDR_ENTRY.const_to_xdr() }
                }
            }
        }
    };

    // SpecShakingMarker impl.
    let spec_shaking_impl = shaking::generate_marker_impl(
        path,
        quote!(#enum_ident),
        quote!(#enum_ident::spec_xdr()),
        std::iter::empty(),
        None,
        None,
        None,
    );

    // Output.
    quote! {
        #spec_name

        #spec_gen

        #spec_shaking_impl

        impl TryFrom<#path::Error> for #enum_ident {
            type Error = #path::Error;
            #[inline(always)]
            fn try_from(error: #path::Error) -> Result<Self, #path::Error> {
                if error.is_type(#path::xdr::ScErrorType::Contract) {
                    let discriminant = error.get_code();
                    Ok(match discriminant {
                        #(#try_froms,)*
                        _ => return Err(error),
                    })
                } else {
                    Err(error)
                }
            }
        }

        impl TryFrom<&#path::Error> for #enum_ident {
            type Error = #path::Error;
            #[inline(always)]
            fn try_from(error: &#path::Error) -> Result<Self, #path::Error> {
                <_ as TryFrom<#path::Error>>::try_from(*error)
            }
        }

        impl From<#enum_ident> for #path::Error {
            #[inline(always)]
            fn from(val: #enum_ident) -> #path::Error {
                <_ as From<&#enum_ident>>::from(&val)
            }
        }

        impl From<&#enum_ident> for #path::Error {
            #[inline(always)]
            fn from(val: &#enum_ident) -> #path::Error {
                match val {
                    #(#into_errors,)*
                }
            }
        }

        impl TryFrom<#path::InvokeError> for #enum_ident {
            type Error = #path::InvokeError;
            #[inline(always)]
            fn try_from(error: #path::InvokeError) -> Result<Self, #path::InvokeError> {
                match error {
                    #path::InvokeError::Abort => Err(error),
                    #path::InvokeError::Contract(code) => Ok(match code {
                        #(#try_froms,)*
                        _ => return Err(error),
                    }),
                }
            }
        }

        impl TryFrom<&#path::InvokeError> for #enum_ident {
            type Error = #path::InvokeError;
            #[inline(always)]
            fn try_from(error: &#path::InvokeError) -> Result<Self, #path::InvokeError> {
                <_ as TryFrom<#path::InvokeError>>::try_from(*error)
            }
        }

        impl From<#enum_ident> for #path::InvokeError {
            #[inline(always)]
            fn from(val: #enum_ident) -> #path::InvokeError {
                <_ as From<&#enum_ident>>::from(&val)
            }
        }

        impl From<&#enum_ident> for #path::InvokeError {
            #[inline(always)]
            fn from(val: &#enum_ident) -> #path::InvokeError {
                match val {
                    #(#into_invoke_errors,)*
                }
            }
        }

        impl #path::TryFromVal<#path::Env, #path::Val> for #enum_ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                use #path::TryIntoVal;
                let error: #path::Error = val.try_into_val(env)?;
                error.try_into().map_err(|_| #path::ConversionError)
            }
        }
        impl #path::TryFromVal<#path::Env, #enum_ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#enum_ident) -> Result<Self, #path::ConversionError> {
                let error: #path::Error = val.into();
                Ok(error.into())
            }
        }

        impl #path::TryFromVal<#path::Env, &#enum_ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &&#enum_ident) -> Result<Self, #path::ConversionError> {
                <_ as #path::TryFromVal<#path::Env, #enum_ident>>::try_from_val(env, *val)
            }
        }
    }
}
