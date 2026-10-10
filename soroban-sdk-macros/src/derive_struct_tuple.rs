use itertools::{izip, MultiUnzip};
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{ext::IdentExt as _, Attribute, DataStruct, Ident, Path, Visibility};

use stellar_xdr::StringM;

use crate::{
    doc::docs_from_attrs,
    spec::{const_view_string, const_view_type_def, spec_anchor_impl, spec_type_def_gen},
};

pub fn derive_type_struct_tuple(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    attrs: &[Attribute],
    data: &DataStruct,
) -> TokenStream2 {
    let fields = &data.fields;
    let field_count_usize: usize = fields.len();

    let (field_docs, field_names, field_idx_lits, field_types, try_from_xdrs, try_into_xdrs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            // For tuple structs that have unnamed fields, use the field index
            // as the token to reference the field.
            let field_idx_lit = Literal::usize_unsuffixed(field_idx);
            let field_name = format!("{}", field_idx);
            let field_type = &field.ty;
            // The field's doc and name, as the spec holds them. Its type in the
            // spec comes from the Rust type.
            let field_doc = docs_from_attrs(&field.attrs);
            let field_name = StringM::<30>::try_from(field_name).unwrap_or_default();
            let try_from_xdr = quote! {
                #field_idx_lit: {
                    let rv: #path::Val = (&vec[#field_idx_lit].clone()).try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
                    rv.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?
                }
            };
            let try_into_xdr = quote! {
                (&val.#field_idx_lit).try_into().map_err(|_| #path::xdr::Error::Invalid)?
            };
            (field_doc, field_name, field_idx_lit, field_type, try_from_xdr, try_into_xdr)
        })
        .multiunzip();

    // Generated code spec. The spec entry is rendered as the equivalent
    // const::ScSpecEntry, which the contract crate encodes to XDR at compile time.
    let spec_type_def = spec_type_def_gen(path, ident, None, None, None);
    let spec_anchor = spec_anchor_impl(path, ident);
    let spec_gen = {
        let doc = const_view_string(path, &docs_from_attrs(attrs));
        // Set to empty string always because the field is no longer used.
        let lib = const_view_string(path, &StringM::<80>::default());
        let fields = izip!(&field_docs, &field_names, &field_types).map(
            |(field_doc, field_name, rust)| {
                let doc = const_view_string(path, field_doc);
                let name = const_view_string(path, field_name);
                let type_ = const_view_type_def(path, rust);
                quote!(#path::xdr::r#const::ScSpecUdtStructFieldV0 { doc: #doc, name: #name, type_: #type_ })
            });
        let spec_entry = quote! {
            #path::xdr::r#const::ScSpecEntry::UdtStructV0(#path::xdr::r#const::ScSpecUdtStructV0 {
                doc: #doc,
                lib: #lib,
                name: #path::xdr::r#const::StringM::try_from_str_or_panic(<#ident as #path::SpecName>::SPEC_NAME),
                fields: #path::xdr::r#const::VecM::try_from_slice_or_panic(&[#(#fields),*]),
            })
        };
        let spec_ident = format_ident!(
            "__SPEC_XDR_TYPE_{}",
            ident.unraw().to_string().to_uppercase()
        );
        quote! {
            #[doc(hidden)]
            #[allow(dead_code)]
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            static #spec_ident: [u8; #ident::spec_xdr().len()] = #ident::spec_xdr();

            impl #ident {
                const __SPEC_XDR_ENTRY: #path::xdr::r#const::ScSpecEntry = #spec_entry;

                pub const fn spec_xdr() -> [u8; #ident::__SPEC_XDR_ENTRY.const_xdr_len()] {
                    const { #ident::__SPEC_XDR_ENTRY.const_to_xdr() }
                }
            }
        }
    };

    // Output.
    let mut output = quote! {
        #spec_type_def

        #spec_gen

        #spec_anchor

        impl #path::TryFromVal<#path::Env, #path::Val> for #ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                <#ident as #path::SpecAnchor>::spec_anchor();
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
                <#ident as #path::SpecAnchor>::spec_anchor();
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
