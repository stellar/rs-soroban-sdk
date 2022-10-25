use itertools::MultiUnzip;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{spanned::Spanned, DataStruct, Error, Ident, Path};

use stellar_xdr::{
    ScSpecEntry, ScSpecTypeDef, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM, WriteXdr,
};

use crate::map_type::map_type;

pub fn derive_type_struct_tuple(
    path: &Path,
    ident: &Ident,
    data: &DataStruct,
    spec: bool,
    lib: &Option<String>,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let fields = &data.fields;
    let field_count_usize: usize = fields.len();
    let field_count_u32: u32 = fields.len().try_into().unwrap_or_else(|_| {
        errors.push(Error::new(
            data.struct_token.span(),
            "struct has too many fields exceeding u32::MAX",
        ));
        0
    });
    let (spec_fields, try_froms, intos, try_from_xdrs, into_xdrs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            // For tuple structs that have unnamed fields, use the field index
            // as the token to reference the field.
            let ident = Literal::usize_unsuffixed(i);
            let name = format!("{}", i);
            let spec_field = ScSpecUdtStructFieldV0 {
                name: name.try_into().unwrap_or_else(|_| StringM::default()),
                type_: match map_type(&f.ty) {
                    Ok(t) => t,
                    Err(e) => {
                        errors.push(e);
                        ScSpecTypeDef::I32
                    }
                },
            };
            let try_from = quote! {
                #ident: if let Some(Ok(val)) = vec.get(#ident) {
                    val.try_into_val(env)?
                } else {
                    Err(#path::ConversionError)?
                }
            };
            let into = quote! { vec.push_back((&self.#ident).into_val(env)) };
            let try_from_xdr = quote! {
                #ident: {
                    let rv: #path::RawVal = (&vec[#ident].clone()).try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
                    rv.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?
                }
            };
            let into_xdr = quote! {
                (&self.#ident).try_into().map_err(|_| #path::xdr::Error::Invalid)?
            };
            (spec_field, try_from, into, try_from_xdr, into_xdr)
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code spec.
    let spec_gen = if spec {
        let spec_entry = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: ident.to_string().try_into().unwrap(),
            fields: spec_fields.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr().unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_{}", ident.to_string().to_uppercase());
        Some(quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #spec_xdr_len] = #ident::spec_xdr();

            impl #ident {
                pub const fn spec_xdr() -> [u8; #spec_xdr_len] {
                    *#spec_xdr_lit
                }
            }
        })
    } else {
        None
    };

    // Output.
    quote! {
        #spec_gen

        impl #path::TryFromVal<#path::Env, #path::RawVal> for #ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: #path::RawVal) -> Result<Self, Self::Error> {
                use #path::TryIntoVal;
                let vec: #path::Vec<#path::RawVal> = val.try_into_val(env)?;
                if vec.len() != #field_count_u32 {
                    return Err(#path::ConversionError);
                }
                Ok(Self{
                    #(#try_froms,)*
                })
            }
        }

        impl #path::TryIntoVal<#path::Env, #ident> for #path::RawVal {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_into_val(self, env: &#path::Env) -> Result<#ident, Self::Error> {
                <_ as #path::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        impl #path::IntoVal<#path::Env, #path::RawVal> for #ident {
            #[inline(always)]
            fn into_val(self, env: &#path::Env) -> #path::RawVal {
                let mut vec = #path::Vec::<#path::RawVal>::new(env);
                #(#intos;)*
                vec.into()
            }
        }

        impl #path::IntoVal<#path::Env, #path::RawVal> for &#ident {
            #[inline(always)]
            fn into_val(self, env: &#path::Env) -> #path::RawVal {
                let mut vec = #path::Vec::<#path::RawVal>::new(env);
                #(#intos;)*
                vec.into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryFromVal<#path::Env, #path::xdr::ScVec> for #ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: #path::xdr::ScVec) -> Result<Self, Self::Error> {
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

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryIntoVal<#path::Env, #ident> for #path::xdr::ScVec {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &#path::Env) -> Result<#ident, Self::Error> {
                <_ as #path::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryFromVal<#path::Env, #path::xdr::ScObject> for #ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: #path::xdr::ScObject) -> Result<Self, Self::Error> {
                if let #path::xdr::ScObject::Vec(map) = val {
                    <_ as #path::TryFromVal<_, _>>::try_from_val(env, map)
                } else {
                    Err(#path::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryIntoVal<#path::Env, #ident> for #path::xdr::ScObject {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &#path::Env) -> Result<#ident, Self::Error> {
                <_ as #path::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryFromVal<#path::Env, #path::xdr::ScVal> for #ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: #path::xdr::ScVal) -> Result<Self, Self::Error> {
                if let #path::xdr::ScVal::Object(Some(obj)) = val {
                    <_ as #path::TryFromVal<_, _>>::try_from_val(env, obj)
                } else {
                    Err(#path::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryIntoVal<#path::Env, #ident> for #path::xdr::ScVal {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &#path::Env) -> Result<#ident, Self::Error> {
                <_ as #path::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<#path::xdr::ScVec> for &#ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<#path::xdr::ScVec, Self::Error> {
                extern crate alloc;
                use #path::TryFromVal;
                Ok(#path::xdr::ScVec(alloc::vec![
                    #(#into_xdrs,)*
                ].try_into()?))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<#path::xdr::ScVec> for #ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<#path::xdr::ScVec, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<#path::xdr::ScObject> for &#ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<#path::xdr::ScObject, Self::Error> {
                Ok(#path::xdr::ScObject::Vec(self.try_into()?))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<#path::xdr::ScObject> for #ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<#path::xdr::ScObject, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<#path::xdr::ScVal> for &#ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<#path::xdr::ScVal, Self::Error> {
                Ok(#path::xdr::ScVal::Object(Some(self.try_into()?)))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<#path::xdr::ScVal> for #ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<#path::xdr::ScVal, Self::Error> {
                (&self).try_into()
            }
        }
    }
}
