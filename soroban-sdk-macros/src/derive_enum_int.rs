use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::{ScSpecUdtEnumV0, StringM};
use syn::{
    ext::IdentExt as _, spanned::Spanned, Attribute, DataEnum, Error, ExprLit, Ident, Lit, Path,
    Visibility,
};

use stellar_xdr::{ScSpecEntry, ScSpecUdtEnumCaseV0, WriteXdr};

use crate::{doc::docs_from_attrs, map_type::const_ref_string, shaking, DEFAULT_XDR_RW_LIMITS};

// TODO: Add conversions to/from ScVal types.

pub fn derive_type_enum_int(
    path: &Path,
    vis: &Visibility,
    enum_ident: &Ident,
    attrs: &[Attribute],
    data: &DataEnum,
    spec: bool,
    lib: &Option<String>,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let variants = &data.variants;
    let (spec_cases, try_froms, try_intos): (Vec<_>, Vec<_>, Vec<_>) = variants
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
            let spec_case = ScSpecUdtEnumCaseV0 {
                doc: docs_from_attrs(&v.attrs),
                name: name.try_into().unwrap_or_else(|_| StringM::default()),
                value: discriminant,
            };
            let try_from = quote! { #discriminant => Self::#ident };
            let try_into = quote! { #enum_ident::#ident => #discriminant.into() };
            (spec_case, try_from, try_into)
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Build the spec entry once if spec is enabled.
    let spec_entry = if spec {
        Some(ScSpecUdtEnumV0 {
            doc: docs_from_attrs(attrs),
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: enum_ident.unraw().to_string().try_into().unwrap(),
            cases: spec_cases.try_into().unwrap(),
        })
    } else {
        None
    };

    // Generated code spec. The spec entry is rendered as the equivalent const
    // ScSpecEntryRef, which the contract crate encodes to XDR at compile time.
    let spec_gen = spec_entry.as_ref().map(|spec_entry| {
        let doc = const_ref_string(path, &spec_entry.doc);
        let lib = const_ref_string(path, &spec_entry.lib);
        let name = const_ref_string(path, &spec_entry.name);
        let cases = spec_entry.cases.iter().map(|c| {
            let doc = const_ref_string(path, &c.doc);
            let name = const_ref_string(path, &c.name);
            let value = c.value;
            quote!(#path::xdr::ScSpecUdtEnumCaseV0Ref { doc: #doc, name: #name, value: #value })
        });
        let spec_ref = quote! {
            #path::xdr::ScSpecEntryRef::UdtEnumV0(#path::xdr::ScSpecUdtEnumV0Ref {
                doc: #doc,
                lib: #lib,
                name: #name,
                cases: #path::xdr::VecMRef::new(&[#(#cases),*]),
            })
        };
        let spec_ident = format_ident!(
            "__SPEC_XDR_TYPE_{}",
            enum_ident.unraw().to_string().to_uppercase()
        );
        quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #enum_ident::__SPEC_XDR_REF.const_xdr_len()] = #enum_ident::spec_xdr();

            impl #enum_ident {
                const __SPEC_XDR_REF: #path::xdr::ScSpecEntryRef<'static> = #spec_ref;

                pub const fn spec_xdr() -> [u8; #enum_ident::__SPEC_XDR_REF.const_xdr_len()] {
                    #enum_ident::__SPEC_XDR_REF.const_to_xdr()
                }
            }
        }
    });

    // SpecShakingMarker impl - only generated when spec is true and the
    // experimental_spec_shaking_v2 feature is enabled.
    let spec_shaking_impl = if cfg!(feature = "experimental_spec_shaking_v2") {
        spec_entry.as_ref().map(|spec_entry| {
            let spec_xdr = ScSpecEntry::UdtEnumV0(spec_entry.clone())
                .to_xdr(DEFAULT_XDR_RW_LIMITS)
                .unwrap();
            shaking::generate_marker_impl(
                path,
                quote!(#enum_ident),
                &spec_xdr,
                std::iter::empty(),
                None,
                None,
                None,
            )
        })
    } else {
        None
    };

    // Output.
    let mut output = quote! {
        #spec_gen

        #spec_shaking_impl

        impl #path::TryFromVal<#path::Env, #path::Val> for #enum_ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                use #path::TryIntoVal;
                let discriminant: u32 = val.try_into_val(env)?;
                Ok(match discriminant {
                    #(#try_froms,)*
                    _ => Err(#path::ConversionError{})?,
                })
            }
        }

        impl #path::TryFromVal<#path::Env, #enum_ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#enum_ident) -> Result<Self, #path::ConversionError> {
                Ok(match val {
                    #(#try_intos,)*
                })
            }
        }

        impl #path::TryFromVal<#path::Env, &#enum_ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &&#enum_ident) -> Result<Self, #path::ConversionError> {
                <_ as #path::TryFromVal<#path::Env, #enum_ident>>::try_from_val(env, *val)
            }
        }
    };

    // Additional output when testutils are enabled.
    if cfg!(feature = "testutils") {
        let arbitrary_tokens =
            crate::arbitrary::derive_arbitrary_enum_int(path, vis, enum_ident, data);
        output.extend(quote! {
            impl #path::TryFromVal<#path::Env, #path::xdr::ScVal> for #enum_ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from_val(env: &#path::Env, val: &#path::xdr::ScVal) -> Result<Self, #path::xdr::Error> {
                    if let #path::xdr::ScVal::U32(discriminant) = val {
                        Ok(match *discriminant {
                            #(#try_froms,)*
                            _ => Err(#path::xdr::Error::Invalid)?,
                        })
                    } else {
                        Err(#path::xdr::Error::Invalid)
                    }
                }
            }

            impl TryInto<#path::xdr::ScVal> for &#enum_ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_into(self) -> Result<#path::xdr::ScVal, #path::xdr::Error> {
                    Ok(match self {
                        #(#try_intos,)*
                    })
                }
            }

            impl TryInto<#path::xdr::ScVal> for #enum_ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_into(self) -> Result<#path::xdr::ScVal, #path::xdr::Error> {
                    Ok(match self {
                        #(#try_intos,)*
                    })
                }
            }

            #arbitrary_tokens
        });
    }
    output
}
