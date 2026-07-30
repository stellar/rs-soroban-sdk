use itertools::MultiUnzip;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{ext::IdentExt as _, Attribute, DataStruct, Error, Ident, Path, Type, Visibility};

use stellar_xdr::{
    ScSpecEntry, ScSpecTypeDef, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM, WriteXdr,
};

use crate::{
    doc::docs_from_attrs,
    map_type::{
        const_ref_string, const_ref_type_def, const_ref_type_def_canonical, map_type,
        spec_type_id_gen,
    },
    shaking, DEFAULT_XDR_RW_LIMITS,
};

pub fn derive_type_struct_tuple(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    attrs: &[Attribute],
    data: &DataStruct,
    spec: bool,
    lib: &Option<String>,
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

    // Build the spec entry. Built even when spec is not enabled, because the
    // type's identity is derived from it and other types' references to this
    // type need that identity regardless.
    let spec_entry = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
        doc: docs_from_attrs(attrs),
        lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
        name: ident.unraw().to_string().try_into().unwrap(),
        fields: field_specs.try_into().unwrap(),
    });
    let ScSpecEntry::UdtStructV0(spec_struct) = &spec_entry else {
        unreachable!()
    };
    // The spec entry rendered as the equivalent const ScSpecEntryRef, which the
    // contract crate encodes to XDR at compile time. `field_type` renders each
    // field's type, and is all that differs between the exported form and the
    // canonical form the type's identity is computed over.
    let entry_ref = |field_type: &dyn Fn(&ScSpecTypeDef, &Type) -> TokenStream2| {
        let doc = const_ref_string(path, &spec_struct.doc);
        let lib = const_ref_string(path, &spec_struct.lib);
        let name = const_ref_string(path, &spec_struct.name);
        let fields = spec_struct
            .fields
            .iter()
            .zip(field_types.iter().copied())
            .map(|(f, rust)| {
            let doc = const_ref_string(path, &f.doc);
            let name = const_ref_string(path, &f.name);
            let type_ = field_type(&f.type_, rust);
            quote!(#path::xdr::ScSpecUdtStructFieldV0Ref { doc: #doc, name: #name, type_: #type_ })
        });
        quote! {
            #path::xdr::ScSpecEntryRef::UdtStructV0(#path::xdr::ScSpecUdtStructV0Ref {
                doc: #doc,
                lib: #lib,
                name: #name,
                fields: #path::xdr::VecMRef::new(&[#(#fields),*]),
            })
        }
    };
    let spec_id_gen = spec_type_id_gen(
        path,
        ident,
        &entry_ref(&|t, _| const_ref_type_def_canonical(path, t)),
    );

    let spec_gen = spec.then(|| {
        let spec_ref = entry_ref(&|t, rust| const_ref_type_def(path, t, Some(rust)));
        let spec_ident = format_ident!(
            "__SPEC_XDR_TYPE_{}",
            ident.unraw().to_string().to_uppercase()
        );
        quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #ident::__SPEC_XDR_REF.const_xdr_len()] = #ident::spec_xdr();

            impl #ident {
                const __SPEC_XDR_REF: #path::xdr::ScSpecEntryRef<'static> = #spec_ref;

                pub const fn spec_xdr() -> [u8; #ident::__SPEC_XDR_REF.const_xdr_len()] {
                    #ident::__SPEC_XDR_REF.const_to_xdr()
                }
            }
        }
    });

    // SpecShakingMarker impl - only generated when spec is true and the
    // experimental_spec_shaking_v2 feature is enabled.
    let spec_shaking_impl = (spec && cfg!(feature = "experimental_spec_shaking_v2")).then(|| {
        let spec_xdr = spec_entry.to_xdr(DEFAULT_XDR_RW_LIMITS).unwrap();
        shaking::generate_marker_impl(
            path,
            quote!(#ident),
            &spec_xdr,
            field_types.iter().cloned(),
            None,
            None,
            None,
        )
    });

    // Output.
    let mut output = quote! {
        #spec_gen

        #spec_id_gen

        #spec_shaking_impl

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
