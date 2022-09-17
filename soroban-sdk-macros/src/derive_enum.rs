use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use soroban_env_common::Symbol;
use syn::{spanned::Spanned, DataEnum, Error, Ident};

use stellar_xdr::{
    ScSpecEntry, ScSpecTypeDef, ScSpecUdtUnionCaseV0, ScSpecUdtUnionV0, VecM, WriteXdr,
};

use crate::map_type::map_type;

pub fn derive_type_enum(
    enum_ident: &Ident,
    data: &DataEnum,
    spec: bool,
    lib: &Option<String>,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let variants = &data.variants;
    let (spec_cases, discriminant_consts, try_froms, intos, try_from_xdrs, into_xdrs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = variants
        .iter()
        .map(|v| {
            // TODO: Choose discriminant type based on repr type of enum.
            // TODO: Should we use variants explicit discriminant? Probably not.
            // Should have a separate derive for those types of enums that maps
            // to an integer type only.
            // TODO: Use attributes tagged on variant to control whether field is included.
            // TODO: Support multi-field enum variants.
            // TODO: Or, error on multi-field enum variants.
            // TODO: Handle field names longer than a symbol. Hash the name? Truncate the name?
            let ident = &v.ident;
            let name = &ident.to_string();
            if let Err(e) = Symbol::try_from_str(name) {
                errors.push(Error::new(ident.span(), format!("enum variant name {}", e)));
            }
            if v.fields.len() > 1 {
                errors.push(Error::new(v.fields.span(), "enum variant name {} has too many tuple values, max 1 supported"));
            }
            let field = v.fields.iter().next();
            let discriminant_const_sym_ident = format_ident!("DISCRIMINANT_SYM_{}", name.to_uppercase());
            let discriminant_const_u64_ident = format_ident!("DISCRIMINANT_U64_{}", name.to_uppercase());
            let discriminant_const_sym = quote! {
                const #discriminant_const_sym_ident: soroban_sdk::Symbol = soroban_sdk::symbol!(#name);
            };
            let discriminant_const_u64 = quote! {
                const #discriminant_const_u64_ident: u64 = #discriminant_const_sym_ident.to_raw().get_payload();
            };
            let discriminant_const = quote! {
                #discriminant_const_sym
                #discriminant_const_u64
            };
            if let Some(f) = field {
                let spec_case = ScSpecUdtUnionCaseV0 {
                    name: name.try_into().unwrap_or_else(|_| VecM::default()),
                    type_: Some(match map_type(&f.ty) {
                        Ok(t) => t,
                        Err(e) => {
                            errors.push(e);
                            ScSpecTypeDef::I32
                        }
                    }),
                };
                let try_from = quote! {
                    #discriminant_const_u64_ident => {
                        if iter.len() > 1 {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::#ident(iter.next().ok_or(soroban_sdk::ConversionError)??.try_into_val(env)?)
                    }
                };
                let into = quote! { #enum_ident::#ident(ref value) => (#discriminant_const_sym_ident, value).into_val(env) };
                let try_from_xdr = quote! {
                    #name => {
                        if iter.len() > 1 {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv: soroban_sdk::RawVal = iter.next().ok_or(soroban_sdk::xdr::Error::Invalid)?.try_into_val(env).map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::#ident(rv.try_into_val(env).map_err(|_| soroban_sdk::xdr::Error::Invalid)?)
                    }
                };
                let into_xdr = quote! { #enum_ident::#ident(value) => (#name, value).try_into().map_err(|_| soroban_sdk::xdr::Error::Invalid)? };
                (spec_case, discriminant_const, try_from, into, try_from_xdr, into_xdr)
            } else {
                let spec_case = ScSpecUdtUnionCaseV0 {
                    name: name.try_into().unwrap_or_else(|_| VecM::default()),
                    type_: None,
                };
                let try_from = quote! {
                    #discriminant_const_u64_ident => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::#ident
                    }
                };
                let into = quote! { #enum_ident::#ident => (#discriminant_const_sym_ident,).into_val(env) };
                let try_from_xdr = quote! {
                    #name => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        Self::#ident
                    }
                };
                let into_xdr = quote! { #enum_ident::#ident => (#name,).try_into().map_err(|_| soroban_sdk::xdr::Error::Invalid)? };
                (spec_case, discriminant_const, try_from, into, try_from_xdr, into_xdr)
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
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: enum_ident.to_string().try_into().unwrap(),
            cases: spec_cases.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr().unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_{}", enum_ident.to_string().to_uppercase());
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

    // Output.
    quote! {
        #spec_gen

        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::RawVal> for #enum_ident {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::RawVal) -> Result<Self, Self::Error> {
                use soroban_sdk::TryIntoVal;
                #(#discriminant_consts)*
                let vec: soroban_sdk::Vec<soroban_sdk::RawVal> = val.try_into_val(env)?;
                let mut iter = vec.iter();
                let discriminant = iter.next().ok_or(soroban_sdk::ConversionError)??;
                Ok(match discriminant.get_payload() {
                    #(#try_froms,)*
                    _ => Err(soroban_sdk::ConversionError{})?,
                })
            }
        }

        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #enum_ident> for soroban_sdk::RawVal {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#enum_ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        impl soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::RawVal> for #enum_ident {
            #[inline(always)]
            fn into_val(self, env: &soroban_sdk::Env) -> soroban_sdk::RawVal {
                #(#discriminant_consts)*
                match &self {
                    #(#intos,)*
                }
            }
        }

        impl soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::RawVal> for &#enum_ident {
            #[inline(always)]
            fn into_val(self, env: &soroban_sdk::Env) -> soroban_sdk::RawVal {
                #(#discriminant_consts)*
                match self {
                    #(#intos,)*
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for #enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::xdr::ScVec) -> Result<Self, Self::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;

                let vec = val;
                let mut iter = vec.iter();
                let discriminant: soroban_sdk::xdr::ScSymbol = iter.next().ok_or(soroban_sdk::xdr::Error::Invalid)?.clone().try_into().map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                let discriminant_name: &str = &discriminant.to_string()?;

                Ok(match discriminant_name {
                    #(#try_from_xdrs,)*
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #enum_ident> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#enum_ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScObject> for #enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::xdr::ScObject) -> Result<Self, Self::Error> {
                if let soroban_sdk::xdr::ScObject::Vec(vec) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #enum_ident> for soroban_sdk::xdr::ScObject {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#enum_ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for #enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::xdr::ScVal) -> Result<Self, Self::Error> {
                if let soroban_sdk::xdr::ScVal::Object(Some(obj)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, obj)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #enum_ident> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#enum_ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScVec> for &#enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVec, Self::Error> {
                extern crate alloc;
                Ok(match self {
                    #(#into_xdrs,)*
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScVec> for #enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVec, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScObject> for &#enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScObject, Self::Error> {
                Ok(soroban_sdk::xdr::ScObject::Vec(self.try_into()?))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScObject> for #enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScObject, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScVal> for &#enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, Self::Error> {
                Ok(soroban_sdk::xdr::ScVal::Object(Some(self.try_into()?)))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScVal> for #enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, Self::Error> {
                (&self).try_into()
            }
        }
    }
}
