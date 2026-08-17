use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct, Ident, Path, Visibility};

pub fn derive_arbitrary_struct(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    data: &DataStruct,
    fuzzcheck: bool,
) -> TokenStream2 {
    derive_arbitrary_struct_common(path, vis, ident, data, FieldType::Named, fuzzcheck)
}

pub fn derive_arbitrary_struct_tuple(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    data: &DataStruct,
    fuzzcheck: bool,
) -> TokenStream2 {
    derive_arbitrary_struct_common(path, vis, ident, data, FieldType::Unnamed, fuzzcheck)
}

/// The `field_mutator` attribute that fuzzcheck's `DefaultMutator` derive
/// requires so that it uses the mutator of the field's contract type, rather
/// than looking for a `DefaultMutator` impl on the field's prototype type.
///
/// Returns nothing when the "fuzzcheck" feature is not enabled.
fn quote_field_mutator_attr(path: &Path, field_type: &syn::Type, fuzzcheck: bool) -> TokenStream2 {
    if !fuzzcheck || !cfg!(feature = "fuzzcheck") {
        return quote! {};
    }
    quote! {
        #[field_mutator(
            <#field_type as #path::testutils::fuzzcheck::SorobanFuzzcheck>::Mutator = {
                <#field_type as #path::testutils::fuzzcheck::SorobanFuzzcheck>::soroban_mutator()
            }
        )]
    }
}

enum FieldType {
    Named,
    Unnamed,
}

fn derive_arbitrary_struct_common(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    data: &DataStruct,
    field_type: FieldType,
    fuzzcheck: bool,
) -> TokenStream2 {
    let arbitrary_type_ident = format_ident!("Arbitrary{}", ident);

    let arbitrary_type_fields: Vec<TokenStream2> = data
        .fields
        .iter()
        .map(|field| {
            let field_type = &field.ty;
            let field_mutator_attr = quote_field_mutator_attr(path, field_type, fuzzcheck);
            match &field.ident {
                Some(ident) => {
                    quote! {
                        #field_mutator_attr
                        #ident: <#field_type as #path::testutils::arbitrary::SorobanArbitrary>::Prototype
                    }
                }
                None => {
                    quote! {
                        #field_mutator_attr
                        <#field_type as #path::testutils::arbitrary::SorobanArbitrary>::Prototype
                    }
                }
            }
        })
        .collect();

    let field_conversions: Vec<TokenStream2> = data
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| match &field.ident {
            Some(ident) => {
                quote! {
                    #ident: #path::IntoVal::into_val(&v.#ident, env)
                }
            }
            None => {
                let i = syn::Index::from(i);
                quote! {
                    #path::IntoVal::into_val(&v.#i, env)
                }
            }
        })
        .collect();

    let arbitrary_type_decl = match field_type {
        FieldType::Named => quote! {
            struct #arbitrary_type_ident {
                #(#arbitrary_type_fields,)*
            }
        },
        FieldType::Unnamed => quote! {
            struct #arbitrary_type_ident (
                #(#arbitrary_type_fields,)*
            );
        },
    };

    let arbitrary_ctor = match field_type {
        FieldType::Named => quote! {
            #ident {
                #(#field_conversions,)*
            }
        },
        FieldType::Unnamed => quote! {
            #ident (
                #(#field_conversions,)*
            )
        },
    };

    quote_arbitrary(
        path,
        vis,
        ident,
        arbitrary_type_ident,
        arbitrary_type_decl,
        arbitrary_ctor,
        fuzzcheck,
    )
}

pub fn derive_arbitrary_enum(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    data: &DataEnum,
    fuzzcheck: bool,
) -> TokenStream2 {
    let arbitrary_type_ident = format_ident!("Arbitrary{}", ident);

    let arbitrary_type_variants: Vec<TokenStream2> = data
        .variants
        .iter()
        .map(|variant| {
            let mut field_types = None;
            let variant_ident = &variant.ident;
            let fields: Vec<TokenStream2> = variant
                .fields
                .iter()
                .map(|field| {
                    let field_type = &field.ty;
                    let field_mutator_attr = quote_field_mutator_attr(path, field_type, fuzzcheck);
                    match &field.ident {
                        Some(ident) => {
                            field_types = Some(FieldType::Named);
                            quote! {
                                #field_mutator_attr
                                #ident: <#field_type as #path::testutils::arbitrary::SorobanArbitrary>::Prototype
                            }
                        }
                        None => {
                            field_types = Some(FieldType::Unnamed);
                            quote! {
                                #field_mutator_attr
                                <#field_type as #path::testutils::arbitrary::SorobanArbitrary>::Prototype
                            }
                        }
                    }
                })
                .collect();
            match field_types {
                None => {
                    quote! {
                        #variant_ident
                    }
                },
                Some(FieldType::Named) => {
                    quote! {
                        #variant_ident { #(#fields,)* }
                    }
                }
                Some(FieldType::Unnamed) => {
                    quote! {
                        #variant_ident ( #(#fields,)* )
                    }
                }
            }
        })
        .collect();

    let variant_conversions: Vec<TokenStream2> = data
        .variants
        .iter()
        .map(|variant| {
            let mut field_types = None;
            let variant_ident = &variant.ident;
            let fields: Vec<TokenStream2> = variant
                .fields
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    match &field.ident {
                        Some(ident) => {
                            quote! {
                                #ident
                            }
                        }
                        None => {
                            let ident = format_ident!("field_{}", i);
                            quote! {
                                #ident
                            }
                        }
                    }
                })
                .collect();
            let field_conversions: Vec<TokenStream2> = variant
                .fields
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    match &field.ident {
                       Some(ident) => {
                            field_types = Some(FieldType::Named);
                            quote! {
                                #ident: #path::IntoVal::into_val(#ident, env)
                            }
                        }
                        None => {
                            field_types = Some(FieldType::Unnamed);
                            let ident = format_ident!("field_{}", i);
                            quote! {
                                #path::IntoVal::into_val(#ident, env)
                            }
                        }
                    }
                })
                .collect();
            match field_types {
                None => {
                    quote! {
                        #arbitrary_type_ident::#variant_ident => #ident::#variant_ident
                    }
                },
                Some(FieldType::Named) => {
                    quote! {
                        #arbitrary_type_ident::#variant_ident { #(#fields,)* } => #ident::#variant_ident { #(#field_conversions,)* }
                    }
                }
                Some(FieldType::Unnamed) => {
                    quote! {
                        #arbitrary_type_ident::#variant_ident ( #(#fields,)* ) => #ident::#variant_ident ( #(#field_conversions,)* )
                    }
                }
            }
        })
        .collect();

    let arbitrary_type_decl = quote! {
        enum #arbitrary_type_ident {
            #(#arbitrary_type_variants,)*
        }
    };
    let arbitrary_ctor = quote! {
        match v {
            #(#variant_conversions,)*
        }
    };

    quote_arbitrary(
        path,
        vis,
        ident,
        arbitrary_type_ident,
        arbitrary_type_decl,
        arbitrary_ctor,
        fuzzcheck,
    )
}

pub fn derive_arbitrary_enum_int(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    data: &DataEnum,
    fuzzcheck: bool,
) -> TokenStream2 {
    let arbitrary_type_ident = format_ident!("Arbitrary{}", ident);

    let arbitrary_type_variants: Vec<TokenStream2> = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            quote! {
                #variant_ident
            }
        })
        .collect();

    let variant_conversions: Vec<TokenStream2> = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            quote! {
                #arbitrary_type_ident::#variant_ident => #ident::#variant_ident
            }
        })
        .collect();

    let arbitrary_type_decl = quote! {
        enum #arbitrary_type_ident {
            #(#arbitrary_type_variants,)*
        }
    };
    let arbitrary_ctor = quote! {
        match v {
            #(#variant_conversions,)*
        }
    };

    quote_arbitrary(
        path,
        vis,
        ident,
        arbitrary_type_ident,
        arbitrary_type_decl,
        arbitrary_ctor,
        fuzzcheck,
    )
}

fn quote_arbitrary(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    arbitrary_type_ident: Ident,
    arbitrary_type_decl: TokenStream2,
    arbitrary_ctor: TokenStream2,
    fuzzcheck: bool,
) -> TokenStream2 {
    // Additional output when fuzzcheck is enabled. The code fuzzcheck's
    // derive(DefaultMutator) generates expects the fuzzcheck crate to be in
    // scope.
    let (fuzzcheck_use, fuzzcheck_derive, fuzzcheck_impl) = if fuzzcheck
        && cfg!(feature = "fuzzcheck")
    {
        (
            quote! {
                use #path::testutils::fuzzcheck::fuzzcheck;
            },
            quote! {
                #[derive(#path::testutils::fuzzcheck::fuzzcheck::DefaultMutator)]
            },
            quote! {
                impl #path::testutils::fuzzcheck::SorobanFuzzcheck for #ident {
                    type Mutator = <#arbitrary_type_ident as #path::testutils::fuzzcheck::DefaultMutator>::Mutator;
                    fn soroban_mutator() -> Self::Mutator {
                        <#arbitrary_type_ident as #path::testutils::fuzzcheck::DefaultMutator>::default_mutator()
                    }
                }
            },
        )
    } else {
        (quote! {}, quote! {}, quote! {})
    };

    quote! {
        // This allows us to create a scope to import std and arbitrary, while
        // also keeping everything from the current scope. This is better than a
        // module because: modules inside functions have surprisingly
        // inconsistent scoping rules and visibility management is harder.
        const _: () = {
            // derive(Arbitrary) expects these two to be in scope
            use #path::testutils::arbitrary::std;
            use #path::testutils::arbitrary::arbitrary;
            #fuzzcheck_use

            #[derive(#path::testutils::arbitrary::arbitrary::Arbitrary)]
            #fuzzcheck_derive
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            #vis #arbitrary_type_decl

            impl #path::testutils::arbitrary::SorobanArbitrary for #ident {
                type Prototype = #arbitrary_type_ident;
            }

            #fuzzcheck_impl

            impl #path::TryFromVal<#path::Env, #arbitrary_type_ident> for #ident {
                type Error = #path::ConversionError;
                fn try_from_val(env: &#path::Env, v: &#arbitrary_type_ident) -> std::result::Result<Self, Self::Error> {
                    Ok(#arbitrary_ctor)
                }
            }
        };
    }
}
