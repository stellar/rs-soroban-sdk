use itertools::MultiUnzip;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{ext::IdentExt as _, Attribute, DataStruct, Error, Ident, Path, Visibility};

use stellar_xdr::{ScSpecTypeDef, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM};

use crate::{
    doc::docs_from_attrs,
    map_type::{const_view_string, const_view_type_def, map_type, spec_name_gen},
};

pub fn derive_type_struct_tuple(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    attrs: &[Attribute],
    data: &DataStruct,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let fields = &data.fields;
    let field_count_usize: usize = fields.len();

    let (field_specs, field_idx_lits, field_types, try_from_xdrs, try_into_xdrs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            // For tuple structs that have unnamed fields, use the field index
            // as the token to reference the field.
            let field_idx_lit = Literal::usize_unsuffixed(field_idx);
            let field_name = format!("{}", field_idx);
            let field_type = &field.ty;
            let field_spec = ScSpecUdtStructFieldV0 {
                doc: docs_from_attrs(&field.attrs),
                name: field_name.try_into().unwrap_or_else(|_| StringM::default()),
                type_: match map_type(&field.ty, false, false) {
                    Ok(t) => t,
                    Err(e) => {
                        errors.push(e);
                        ScSpecTypeDef::I32
                    }
                },
            };
            let try_from_xdr = quote! {
                #field_idx_lit: {
                    let rv: #path::Val = (&vec[#field_idx_lit].clone()).try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
                    rv.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?
                }
            };
            let try_into_xdr = quote! {
                (&val.#field_idx_lit).try_into().map_err(|_| #path::xdr::Error::Invalid)?
            };
            (field_spec, field_idx_lit, field_type, try_from_xdr, try_into_xdr)
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Build the spec entry once.
    let spec = ScSpecUdtStructV0 {
        doc: docs_from_attrs(attrs),
        // set to empty string always because the field is no longer used
        lib: StringM::default(),
        name: ident.unraw().to_string().try_into().unwrap(),
        fields: field_specs.try_into().unwrap(),
    };

    // The fully qualified name the spec knows this type by, emitted for every
    // type so that a reference to it from anywhere can reach it.
    let spec_name = spec_name_gen(ident, None, None, None);

    // Generated code spec. The spec entry is rendered as the equivalent const
    // ScSpecEntryView, which the contract crate encodes to XDR at compile time.
    let spec_gen = {
        let doc = const_view_string(path, &spec.doc);
        let lib = const_view_string(path, &spec.lib);
        let name = quote!(#path::xdr::StringMView::try_from_str_or_panic(#ident::spec_name()));
        // Each field's Rust type, so a reference to a user-defined type in a
        // field resolves to the name that type reports for itself.
        let fields = spec
            .fields
            .iter()
            .zip(field_types.iter().copied())
            .map(|(f, rust)| {
                let doc = const_view_string(path, &f.doc);
                let name = const_view_string(path, &f.name);
                let type_ = const_view_type_def(path, &f.type_, Some(rust));
                quote!(#path::xdr::ScSpecUdtStructFieldV0View { doc: #doc, name: #name, type_: #type_ })
            });
        let spec_view = quote! {
            #path::xdr::ScSpecEntryView::UdtStructV0(#path::xdr::ScSpecUdtStructV0View {
                doc: #doc,
                lib: #lib,
                name: #name,
                fields: #path::xdr::VecMView::try_from_slice_or_panic(&[#(#fields),*]),
            })
        };
        let spec_ident = format_ident!(
            "__SPEC_XDR_TYPE_{}",
            ident.unraw().to_string().to_uppercase()
        );
        quote! {
            #[doc(hidden)]
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #ident::spec_xdr_len()] = #ident::spec_xdr();

            impl #ident {
                const __SPEC_XDR_ENTRY: #path::xdr::ScSpecEntryView<'static> = #spec_view;

                pub const fn spec_xdr_len() -> usize {
                    const { #ident::__SPEC_XDR_ENTRY.const_xdr_len() }
                }

                pub const fn spec_xdr() -> [u8; #ident::spec_xdr_len()] {
                    const { #ident::__SPEC_XDR_ENTRY.const_to_xdr() }
                }
            }
        }
    };

    // Output.
    let mut output = quote! {
        #spec_name

        #spec_gen

        impl #path::TryFromVal<#path::Env, #path::Val> for #ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                use #path::{TryIntoVal,EnvBase,ConversionError,VecObject,Val};
                let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
                let mut vals: [Val; #field_count_usize] = [Val::VOID.to_val(); #field_count_usize];
                env.vec_unpack_to_slice(vec, &mut vals).map_err(|_| ConversionError)?;
                Ok(Self{
                    #(#field_idx_lits: vals[#field_idx_lits].try_into_val(env).map_err(|_| ConversionError)?),*
                })
            }
        }

        impl #path::TryFromVal<#path::Env, #ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#ident) -> Result<Self, #path::ConversionError> {
                use #path::{TryIntoVal,EnvBase,ConversionError,Val};
                let vals: [Val; #field_count_usize] = [
                    #((&val.#field_idx_lits).try_into_val(env).map_err(|_| ConversionError)?),*
                ];
                Ok(env.vec_new_from_slice(&vals).map_err(|_| ConversionError)?.into())
            }
        }

        impl #path::TryFromVal<#path::Env, &#ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &&#ident) -> Result<Self, #path::ConversionError> {
                <_ as #path::TryFromVal<#path::Env, #ident>>::try_from_val(env, *val)
            }
        }
    };

    // Additional output when testutils are enabled.
    if cfg!(feature = "testutils") {
        let arbitrary_tokens =
            crate::arbitrary::derive_arbitrary_struct_tuple(path, vis, ident, data);
        output.extend(quote! {
            impl #path::TryFromVal<#path::Env, #path::xdr::ScVec> for #ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from_val(env: &#path::Env, val: &#path::xdr::ScVec) -> Result<Self, #path::xdr::Error> {
                    use #path::xdr::Validate;
                    use #path::TryIntoVal;
                    let vec = val;
                    if vec.len() != #field_count_usize {
                        return Err(#path::xdr::Error::Invalid);
                    }
                    Ok(Self{
                        #(#try_from_xdrs,)*
                    })
                }
            }

            impl #path::TryFromVal<#path::Env, #path::xdr::ScVal> for #ident {
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

            impl TryFrom<&#ident> for #path::xdr::ScVec {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: &#ident) -> Result<Self, #path::xdr::Error> {
                    extern crate alloc;
                    use #path::TryFromVal;
                    Ok(#path::xdr::ScVec(alloc::vec![
                        #(#try_into_xdrs,)*
                    ].try_into()?))
                }
            }

            impl TryFrom<#ident> for #path::xdr::ScVec {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: #ident) -> Result<Self, #path::xdr::Error> {
                    (&val).try_into()
                }
            }

            impl TryFrom<&#ident> for #path::xdr::ScVal {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: &#ident) -> Result<Self, #path::xdr::Error> {
                    Ok(#path::xdr::ScVal::Vec(Some(val.try_into()?)))
                }
            }

            impl TryFrom<#ident> for #path::xdr::ScVal {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: #ident) -> Result<Self, #path::xdr::Error> {
                    (&val).try_into()
                }
            }

            #arbitrary_tokens
        });
    }
    output
}
