use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataEnum, DataStruct, Ident};

// TODO: Replace use of vecs with maps.
// TODO: Replace use of index integers with symbols specified on fields.
// TODO: Add field attribute for including/excluding fields in types.

pub fn derive_type_struct(_ident: &Ident, _data: &DataStruct) -> TokenStream2 {
    todo!()
}

pub fn derive_type_enum(ident: &Ident, data: &DataEnum) -> TokenStream2 {
    let variants = &data.variants;
    let (try_froms, intos): (Vec<_>, Vec<_>) = variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let i: u32 = i.try_into().unwrap();
            // TODO: Choose discriminant type based on repr type of enum.
            // TODO: Should we use variants explicit discriminant? Probably not.
            // Should have a separate derive for those types of enums that maps
            // to an integer type only.
            // TODO: Use attributes tagged on variant to control whether field is included.
            // TODO: Support multi-field enum variants.
            let ident = &v.ident;
            let field = v.fields.iter().next();
            if field.is_some() {
                let try_from = quote! { #i => Self::#ident(value.try_into()?) };
                let into = quote! { Self::#ident(value) => (#i, value).into_env_val(env) };
                (try_from, into)
            } else {
                let try_from = quote! { #i => Self::#ident };
                let into = quote! { Self::#ident => (#i, ()).into_env_val(env) };
                (try_from, into)
            }
        })
        .multiunzip();
    quote! {
        impl TryFrom<EnvVal> for #ident {
            type Error = ();
            #[inline(always)]
            fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
                let (discriminant, value): (u32, EnvVal) = ev.try_into()?;
                let v = match discriminant {
                    #(#try_froms,)*
                    _ => Err(())?
                };
                Ok(v)
            }
        }
        impl IntoEnvVal<Env, RawVal> for #ident {
            #[inline(always)]
            fn into_env_val(self, env: &Env) -> EnvVal {
                match self {
                    #(#intos,)*
                }
            }
        }
    }
}
