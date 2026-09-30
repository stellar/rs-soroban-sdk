use itertools::MultiUnzip;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    ext::IdentExt as _, spanned::Spanned, Attribute, DataEnum, Error, Fields, Ident, Path,
    Visibility,
};

use stellar_xdr::{StringM, SCSYMBOL_LIMIT};

use crate::{
    doc::docs_from_attrs,
    map_type::{const_view_string, const_view_type_def, spec_type_def_gen},
};

pub fn derive_type_enum(
    path: &Path,
    vis: &Visibility,
    enum_ident: &Ident,
    attrs: &[Attribute],
    data: &DataEnum,
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
    let (
        spec_cases,
        case_name_str_lits,
        variant_field_types,
        try_froms,
        try_intos,
        try_from_xdrs,
        into_xdrs,
    ): (Vec<_>, Vec<_>, Vec<Vec<_>>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = variants
        .iter()
        .enumerate()
        .map(|(case_num, variant)| {
            // TODO: Choose discriminant type based on repr type of enum.
            // TODO: Use attributes tagged on variant to control whether field is included.
            let case_ident = &variant.ident;
            let case_name = &case_ident.unraw().to_string();
            let case_name_str_lit = Literal::string(case_name);
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

            // Collect field types for SpecShakingMarker
            let field_types: Vec<_> = variant.fields.iter().map(|f| &f.ty).collect();

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
                    case_name,
                    case_ident,
                    &variant.attrs,
                    &variant.fields,
                );
                (
                    spec_case,
                    case_name_str_lit,
                    field_types,
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
                    case_name,
                    case_ident,
                    &variant.attrs,
                );
                (
                    spec_case,
                    case_name_str_lit,
                    field_types,
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

    // Generated code spec. The spec entry is rendered as the equivalent
    // const::ScSpecEntry, which the contract crate encodes to XDR at compile time.
    let spec_type_def = spec_type_def_gen(path, enum_ident, None, None, None);
    let spec_gen = {
        let doc = const_view_string(path, &docs_from_attrs(attrs));
        // Set to empty string always because the field is no longer used.
        let lib = const_view_string(path, &StringM::<80>::default());
        let cases = spec_cases
            .iter()
            .zip(&variant_field_types)
            .map(|(c, field_types)| {
                let doc = const_view_string(path, &c.doc);
                let name = const_view_string(path, &c.name);
                if !c.tuple {
                    quote!(#path::xdr::r#const::ScSpecUdtUnionCaseV0::VoidV0(
                        #path::xdr::r#const::ScSpecUdtUnionCaseVoidV0 { doc: #doc, name: #name }
                    ))
                } else {
                    let type_ = field_types
                        .iter()
                        .map(|rust| const_view_type_def(path, rust));
                    quote!(#path::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(
                        #path::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                            doc: #doc,
                            name: #name,
                            type_: #path::xdr::r#const::VecM::try_from_slice_or_panic(&[#(#type_),*]),
                        }
                    ))
                }
            });
        let spec_entry = quote! {
            #path::xdr::r#const::ScSpecEntry::UdtUnionV0(#path::xdr::r#const::ScSpecUdtUnionV0 {
                doc: #doc,
                lib: #lib,
                name: #path::xdr::r#const::StringM::try_from_str_or_panic(<#enum_ident as #path::SpecName>::SPEC_NAME),
                cases: #path::xdr::r#const::VecM::try_from_slice_or_panic(&[#(#cases),*]),
            })
        };
        let spec_ident = format_ident!(
            "__SPEC_XDR_TYPE_{}",
            enum_ident.unraw().to_string().to_uppercase()
        );
        quote! {
            #[doc(hidden)]
            #[allow(dead_code)]
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            static #spec_ident: [u8; #enum_ident::spec_xdr().len()] = #enum_ident::spec_xdr();

            impl #enum_ident {
                const __SPEC_XDR_ENTRY: #path::xdr::r#const::ScSpecEntry = #spec_entry;

                pub const fn spec_xdr() -> [u8; #enum_ident::__SPEC_XDR_ENTRY.const_xdr_len()] {
                    const { #enum_ident::__SPEC_XDR_ENTRY.const_to_xdr() }
                }
            }
        }
    };

    // Output.
    let mut output = quote! {
        #spec_type_def

        #spec_gen

        impl #path::TryFromVal<#path::Env, #path::Val> for #enum_ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                use #path::{EnvBase,TryIntoVal,TryFromVal};
                const CASES: &'static [&'static str] = &[#(#case_name_str_lits),*];
                let vec: #path::Vec<#path::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: #path::Symbol = iter.next().ok_or(#path::ConversionError)??.try_into_val(env).map_err(|_|#path::ConversionError)?;
                Ok(match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?) as usize {
                    #(#try_froms,)*
                    _ => Err(#path::ConversionError{})?,
                })
            }
        }

        impl #path::TryFromVal<#path::Env, #enum_ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#enum_ident) -> Result<Self, #path::ConversionError> {
                use #path::{TryIntoVal,TryFromVal};
                match val {
                    #(#try_intos,)*
                }
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
        let arbitrary_tokens = crate::arbitrary::derive_arbitrary_enum(path, vis, enum_ident, data);
        output.extend(quote! {
            impl #path::TryFromVal<#path::Env, #path::xdr::ScVec> for #enum_ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from_val(env: &#path::Env, val: &#path::xdr::ScVec) -> Result<Self, #path::xdr::Error> {
                    use #path::xdr::Validate;
                    use #path::TryIntoVal;

                    let vec = val;
                    let mut iter = vec.iter();
                    let discriminant: #path::xdr::ScSymbol = iter.next().ok_or(#path::xdr::Error::Invalid)?.clone().try_into().map_err(|_| #path::xdr::Error::Invalid)?;
                    let discriminant_name: &str = &discriminant.to_utf8_string()?;

                    Ok(match discriminant_name {
                        #(#try_from_xdrs,)*
                        _ => Err(#path::xdr::Error::Invalid)?,
                    })
                }
            }

            impl #path::TryFromVal<#path::Env, #path::xdr::ScVal> for #enum_ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from_val(env: &#path::Env, val: &#path::xdr::ScVal) -> Result<Self, #path::xdr::Error> {
                    if let #path::xdr::ScVal::Vec(Some(vec)) = val {
                        <_ as #path::TryFromVal<_, _>>::try_from_val(env, vec)
                    } else {
                        Err(#path::xdr::Error::Invalid)
                    }
                }
            }

            impl TryFrom<&#enum_ident> for #path::xdr::ScVec {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: &#enum_ident) -> Result<Self, #path::xdr::Error> {
                    extern crate alloc;
                    Ok(match val {
                        #(#into_xdrs,)*
                    })
                }
            }

            impl TryFrom<#enum_ident> for #path::xdr::ScVec  {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: #enum_ident) -> Result<Self, #path::xdr::Error> {
                    (&val).try_into()
                }
            }

            impl TryFrom<&#enum_ident> for #path::xdr::ScVal  {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: &#enum_ident) -> Result<Self, #path::xdr::Error> {
                    Ok(#path::xdr::ScVal::Vec(Some(val.try_into()?)))
                }
            }

            impl TryFrom<#enum_ident> for #path::xdr::ScVal  {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: #enum_ident) -> Result<Self, #path::xdr::Error> {
                    (&val).try_into()
                }
            }

            #arbitrary_tokens
        });
    }
    output
}

/// A union case's doc and name, as the spec holds them, and whether it is a
/// tuple case. The spec types of a tuple case's values come from the Rust types
/// of its fields.
struct SpecCase {
    doc: StringM<1024>,
    name: StringM<60>,
    tuple: bool,
}

struct VariantTokens {
    spec_case: SpecCase,
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
    let spec_case = SpecCase {
        doc: docs_from_attrs(attrs),
        name: case_name.try_into().unwrap_or_else(|_| StringM::default()),
        tuple: false,
    };
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
            let tup: (#path::Val,) = (#path::Symbol::try_from_val(env, &#case_name_str_lit)?.to_val(),);
            tup.try_into_val(env).map_err(Into::into)
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
    let into_xdr = quote! {
        #enum_ident::#case_ident => {
            let symbol = #path::xdr::ScSymbol(#case_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?);
            let val = #path::xdr::ScVal::Symbol(symbol);
            (val,).try_into().map_err(|_| #path::xdr::Error::Invalid)?
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

fn map_tuple_variant(
    path: &Path,
    enum_ident: &Ident,
    case_num_lit: &Literal,
    case_name_str_lit: &Literal,
    case_name: &str,
    case_ident: &Ident,
    attrs: &[Attribute],
    fields: &Fields,
) -> VariantTokens {
    let spec_case = SpecCase {
        doc: docs_from_attrs(attrs),
        name: case_name.try_into().unwrap_or_else(|_| StringM::default()),
        tuple: true,
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
                    #path::Val
                };
                (binding_name, field_conv, tup_elem_type)
            })
            .multiunzip();
        let (binding_names, field_convs, tup_elem_types): (Vec<_>, Vec<_>, Vec<_>) = fragments;
        quote! {
            #enum_ident::#case_ident(#(ref #binding_names,)* ) => {
                let tup: (#path::Val, #(#tup_elem_types,)* ) = (#path::Symbol::try_from_val(env, &#case_name_str_lit)?.to_val(), #(#field_convs,)* );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    };
    let try_from_xdr = {
        let fragments = fields.iter().enumerate().map(|(i, _f)| {
            let val_name = format_ident!("rv{i}");
            let val_binding = quote! {
                let #val_name: #path::Val = iter.next().ok_or(#path::xdr::Error::Invalid)?.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
            };
            let into_field = quote! {
                #val_name.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?
            };
            (val_binding, into_field)
        }).multiunzip();
        let (val_bindings, into_fields): (Vec<_>, Vec<_>) = fragments;
        quote! {
            #case_name => {
                if iter.len() > #num_fields {
                    return Err(#path::xdr::Error::Invalid);
                }
                #(#val_bindings)*
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
            #enum_ident::#case_ident( #(#binding_names,)* ) => (
                #path::xdr::ScSymbol(#case_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?),
                #(#binding_names,)*
            ).try_into().map_err(|_| #path::xdr::Error::Invalid)?
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
