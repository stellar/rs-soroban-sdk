use itertools::MultiUnzip;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Attribute, DataEnum, Error, Fields, Ident, Path, Visibility};

use stellar_xdr::{
    Error as XdrError, ScSpecEntry, ScSpecTypeDef, ScSpecUdtUnionCaseTupleV0, ScSpecUdtUnionCaseV0,
    ScSpecUdtUnionCaseVoidV0, ScSpecUdtUnionV0, StringM, VecM, WriteXdr, SCSYMBOL_LIMIT,
};

use crate::{doc::docs_from_attrs, map_type::map_type};

pub fn derive_type_enum(
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
    if variants.is_empty() {
        errors.push(Error::new(
            enum_ident.span(),
            format!("enum {} must have variants", enum_ident),
        ));
    }
    let (spec_cases, case_name_str_lits, try_froms, try_intos, try_from_xdrs, into_xdrs): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = variants
        .iter()
        .enumerate()
        .map(|(case_num, variant)| {
            // TODO: Choose discriminant type based on repr type of enum.
            // TODO: Use attributes tagged on variant to control whether field is included.
            let case_ident = &variant.ident;
            let case_name = &case_ident.to_string();
            let case_name_str_lit = Literal::string(&case_name);
            let case_num_lit = Literal::usize_unsuffixed(case_num);
            if case_name.len() > SCSYMBOL_LIMIT as usize {
                errors.push(Error::new(
                    case_ident.span(),
                    format!(
                        "enum field name is too long: {}, max is {}",
                        case_name.len(),
                        SCSYMBOL_LIMIT
                    ),
                ));
            }

            match variant.fields {
                Fields::Named(_) => {
                    errors.push(Error::new(
                        variant.fields.span(),
                        format!("enum variant {} has unsupported named fields", case_ident),
                    ));
                }
                Fields::Unnamed(_) if variant.fields.is_empty() => {
                    // Empty tuples are unsupported because it would require extra complexity
                    // to distinguish them from unit-style variants.
                    errors.push(Error::new(
                        variant.fields.span(),
                        format!("enum variant {} is unsupported 0-element tuple", case_ident),
                    ));
                }
                _ => {}
            }
            let is_unit_variant = variant.fields == Fields::Unit;
            if !is_unit_variant {
                let VariantTokens {
                    spec_case,
                    try_from,
                    try_into,
                    try_from_xdr,
                    into_xdr,
                } = map_tuple_variant(
                    path,
                    enum_ident,
                    &case_num_lit,
                    &case_name_str_lit,
                    &case_name,
                    case_ident,
                    &variant.attrs,
                    &variant.fields,
                    &mut errors,
                );
                (
                    spec_case,
                    case_name_str_lit,
                    try_from,
                    try_into,
                    try_from_xdr,
                    into_xdr,
                )
            } else {
                let VariantTokens {
                    spec_case,
                    try_from,
                    try_into,
                    try_from_xdr,
                    into_xdr,
                } = map_empty_variant(
                    path,
                    enum_ident,
                    &case_num_lit,
                    &case_name_str_lit,
                    &case_name,
                    case_ident,
                    &variant.attrs,
                );
                (
                    spec_case,
                    case_name_str_lit,
                    try_from,
                    try_into,
                    try_from_xdr,
                    into_xdr,
                )
            }
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code spec.
    let spec_gen = if spec {
        let spec_entry = ScSpecEntry::UdtUnionV0(ScSpecUdtUnionV0 {
            doc: docs_from_attrs(attrs).try_into().unwrap(), // TODO: Truncate docs, or display friendly compile error.
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: enum_ident.to_string().try_into().unwrap(),
            cases: spec_cases.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr().unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_TYPE_{}", enum_ident.to_string().to_uppercase());
        Some(quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #spec_xdr_len] = #enum_ident::spec_xdr();

            impl #enum_ident {
                pub const fn spec_xdr() -> [u8; #spec_xdr_len] {
                    *#spec_xdr_lit
                }
            }
        })
    } else {
        None
    };

    let arbitrary_tokens = crate::arbitrary::derive_arbitrary_enum(path, vis, enum_ident, data);

    // Output.
    quote! {
        #spec_gen

        impl #path::TryFromVal<#path::Env, #path::RawVal> for #enum_ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::RawVal) -> Result<Self, Self::Error> {
                use #path::{EnvBase,TryIntoVal,TryFromVal};
                const CASES: &'static [&'static str] = &[#(#case_name_str_lits),*];
                let vec: #path::Vec<#path::RawVal> = val.try_into_val(env)?;
                let mut iter = vec.iter();
                let discriminant: #path::Symbol = iter.next().ok_or(#path::ConversionError)??.try_into_val(env).map_err(|_|#path::ConversionError)?;
                Ok(match u32::from(env.symbol_index_in_strs(discriminant.to_val(), CASES)?) as usize {
                    #(#try_froms,)*
                    _ => Err(#path::ConversionError{})?,
                })
            }
        }

        impl #path::TryFromVal<#path::Env, #enum_ident> for #path::RawVal {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#enum_ident) -> Result<Self, Self::Error> {
                use #path::{TryIntoVal,TryFromVal};
                match val {
                    #(#try_intos,)*
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryFromVal<#path::Env, #path::xdr::ScVec> for #enum_ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::xdr::ScVec) -> Result<Self, Self::Error> {
                use #path::xdr::Validate;
                use #path::TryIntoVal;

                let vec = val;
                let mut iter = vec.iter();
                let discriminant: #path::xdr::ScSymbol = iter.next().ok_or(#path::xdr::Error::Invalid)?.clone().try_into().map_err(|_| #path::xdr::Error::Invalid)?;
                let discriminant_name: &str = &discriminant.to_string()?;

                Ok(match discriminant_name {
                    #(#try_from_xdrs,)*
                    _ => Err(#path::xdr::Error::Invalid)?,
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryFromVal<#path::Env, #path::xdr::ScVal> for #enum_ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::xdr::ScVal) -> Result<Self, Self::Error> {
                if let #path::xdr::ScVal::Vec(Some(vec)) = val {
                    <_ as #path::TryFromVal<_, _>>::try_from_val(env, vec)
                } else {
                    Err(#path::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<&#enum_ident> for #path::xdr::ScVec {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from(val: &#enum_ident) -> Result<Self, Self::Error> {
                extern crate alloc;
                Ok(match val {
                    #(#into_xdrs,)*
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<#enum_ident> for #path::xdr::ScVec  {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from(val: #enum_ident) -> Result<Self, Self::Error> {
                (&val).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<&#enum_ident> for #path::xdr::ScVal  {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from(val: &#enum_ident) -> Result<Self, Self::Error> {
                Ok(#path::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<#enum_ident> for #path::xdr::ScVal  {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from(val: #enum_ident) -> Result<Self, Self::Error> {
                (&val).try_into()
            }
        }

        #arbitrary_tokens
    }
}

struct VariantTokens {
    spec_case: ScSpecUdtUnionCaseV0,
    try_from: TokenStream2,
    try_into: TokenStream2,
    try_from_xdr: TokenStream2,
    into_xdr: TokenStream2,
}

fn map_empty_variant(
    path: &Path,
    enum_ident: &Ident,
    case_num_lit: &Literal,
    case_name_str_lit: &Literal,
    case_name: &str,
    case_ident: &Ident,
    attrs: &[Attribute],
) -> VariantTokens {
    let spec_case = ScSpecUdtUnionCaseV0::VoidV0(ScSpecUdtUnionCaseVoidV0 {
        doc: docs_from_attrs(attrs).try_into().unwrap(), // TODO: Truncate docs, or display friendly compile error.
        name: case_name.try_into().unwrap_or_else(|_| StringM::default()),
    });
    let try_from = quote! {
        #case_num_lit => {
            if iter.len() > 0 {
                return Err(#path::ConversionError);
            }
            Self::#case_ident
        }
    };
    let try_into = quote! {
        #enum_ident::#case_ident => {
            let tup: (#path::RawVal,) = (#path::Symbol::try_from_val(env, &#case_name_str_lit)?.to_raw(),);
            tup.try_into_val(env)
        }
    };
    let try_from_xdr = quote! {
        #case_name => {
            if iter.len() > 0 {
                return Err(#path::xdr::Error::Invalid);
            }
            Self::#case_ident
        }
    };
    let into_xdr = quote! { #enum_ident::#case_ident => (#case_name,).try_into().map_err(|_| #path::xdr::Error::Invalid)? };

    VariantTokens {
        spec_case,
        try_from,
        try_into,
        try_from_xdr,
        into_xdr,
    }
}

fn map_tuple_variant(
    path: &Path,
    enum_ident: &Ident,
    case_num_lit: &Literal,
    case_name_str_lit: &Literal,
    case_name: &str,
    case_ident: &Ident,
    attrs: &[Attribute],
    fields: &Fields,
    errors: &mut Vec<Error>,
) -> VariantTokens {
    let spec_case = {
        let field_types = fields
            .iter()
            .map(|f| match map_type(&f.ty) {
                Ok(t) => t,
                Err(e) => {
                    errors.push(e);
                    ScSpecTypeDef::I32
                }
            })
            .collect::<Vec<_>>();
        let field_types = match VecM::try_from(field_types) {
            Ok(t) => t,
            Err(e) => {
                let v = VecM::default();
                let max_len = v.max_len();
                match e {
                    XdrError::LengthExceedsMax => {
                        errors.push(Error::new(
                            fields.span(),
                            format!(
                                "enum variant name {} has too many tuple values, max {} supported",
                                case_ident, max_len
                            ),
                        ));
                    }
                    e => {
                        errors.push(Error::new(fields.span(), format!("{e}")));
                    }
                }
                v
            }
        };
        ScSpecUdtUnionCaseV0::TupleV0(ScSpecUdtUnionCaseTupleV0 {
            doc: docs_from_attrs(attrs).try_into().unwrap(), // TODO: Truncate docs, or display friendly compile error.
            name: case_name.try_into().unwrap_or_else(|_| StringM::default()),
            type_: field_types.try_into().unwrap(),
        })
    };

    let num_fields = fields.iter().len();
    let try_from = {
        let field_convs = fields
            .iter()
            .enumerate()
            .map(|(_i, _f)| {
                quote! {
                    iter.next().ok_or(#path::ConversionError)??.try_into_val(env)?
                }
            })
            .collect::<Vec<_>>();
        quote! {
            #case_num_lit => {
                if iter.len() > #num_fields {
                    return Err(#path::ConversionError);
                }
                Self::#case_ident( #(#field_convs,)* )
            }
        }
    };
    let try_into = {
        let fragments = fields
            .iter()
            .enumerate()
            .map(|(i, _f)| {
                let binding_name = format_ident!("value{i}");
                let field_conv = quote! {
                    #binding_name.try_into_val(env)?
                };
                let tup_elem_type = quote! {
                    #path::RawVal
                };
                (binding_name, field_conv, tup_elem_type)
            })
            .multiunzip();
        let (binding_names, field_convs, tup_elem_types): (Vec<_>, Vec<_>, Vec<_>) = fragments;
        quote! {
            #enum_ident::#case_ident(#(ref #binding_names,)* ) => {
                let tup: (#path::RawVal, #(#tup_elem_types,)* ) = (#path::Symbol::try_from_val(env, &#case_name_str_lit)?.to_raw(), #(#field_convs,)* );
                tup.try_into_val(env)
            }
        }
    };
    let try_from_xdr = {
        let fragments = fields.iter().enumerate().map(|(i, _f)| {
            let rawval_name = format_ident!("rv{i}");
            let rawval_binding = quote! {
                let #rawval_name: #path::RawVal = iter.next().ok_or(#path::xdr::Error::Invalid)?.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
            };
            let into_field = quote! {
                #rawval_name.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?
            };
            (rawval_binding, into_field)
        }).multiunzip();
        let (rawval_bindings, into_fields): (Vec<_>, Vec<_>) = fragments;
        quote! {
            #case_name => {
                if iter.len() > #num_fields {
                    return Err(#path::xdr::Error::Invalid);
                }
                #(#rawval_bindings)*
                Self::#case_ident( #(#into_fields,)* )
            }
        }
    };
    let into_xdr = {
        let binding_names = fields
            .iter()
            .enumerate()
            .map(|(i, _f)| format_ident!("value{i}"))
            .collect::<Vec<_>>();
        quote! {
            #enum_ident::#case_ident( #(#binding_names,)* ) => (#case_name, #(#binding_names,)* ).try_into().map_err(|_| #path::xdr::Error::Invalid)?
        }
    };

    VariantTokens {
        spec_case,
        try_from,
        try_into,
        try_from_xdr,
        into_xdr,
    }
}
