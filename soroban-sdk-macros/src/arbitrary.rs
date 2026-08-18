use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct, Fields, Ident, Path, Type, Visibility};

/// `proptest` implements `Strategy` for tuples of up to this many elements, so
/// strategies for prototypes with more fields are built from nested tuples.
const MAX_TUPLE_LEN: usize = 10;

/// A `proptest` strategy generating the prototype of a contract type's field.
fn field_strategy(path: &Path, field_type: &Type) -> TokenStream2 {
    quote! {
        <<#field_type as #path::testutils::arbitrary::SorobanArbitrary>::Prototype
            as #path::testutils::proptest::ProtoStrategy>::proto_strategy()
    }
}

/// Combines strategies into one strategy of a tuple of their values, returning
/// the strategy expression and a pattern destructuring the tuple into the given
/// bindings. Tuples longer than `MAX_TUPLE_LEN` are nested.
fn tuple_strategy(items: &[(TokenStream2, TokenStream2)]) -> (TokenStream2, TokenStream2) {
    assert!(!items.is_empty());
    if items.len() <= MAX_TUPLE_LEN {
        let strategies = items.iter().map(|(s, _)| s);
        let bindings = items.iter().map(|(_, b)| b);
        return (
            quote! { ( #(#strategies,)* ) },
            quote! { ( #(#bindings,)* ) },
        );
    }
    let chunks: Vec<(TokenStream2, TokenStream2)> =
        items.chunks(MAX_TUPLE_LEN).map(tuple_strategy).collect();
    tuple_strategy(&chunks)
}

/// A strategy generating a prototype value or variant with the given fields,
/// constructed by `ctor` from bindings named `field_0`, `field_1`, ….
///
/// The fields share the element budget, so that a prototype with many fields
/// generates values of the same size as one with few.
fn fields_strategy(path: &Path, fields: &Fields, ctor: TokenStream2) -> TokenStream2 {
    let items: Vec<(TokenStream2, TokenStream2)> = fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let binding = format_ident!("field_{}", i);
            (field_strategy(path, &field.ty), quote! { #binding })
        })
        .collect();
    if items.is_empty() {
        return quote! {
            #path::testutils::proptest::proptest::strategy::Strategy::boxed(
                #path::testutils::proptest::proptest::strategy::Just(#ctor)
            )
        };
    }
    let count = items.len();
    let (strategies, bindings) = tuple_strategy(&items);
    quote! {
        #path::testutils::proptest::with_fields(#count, || {
            #path::testutils::proptest::proptest::strategy::Strategy::boxed(
                #path::testutils::proptest::proptest::strategy::Strategy::prop_map(
                    #strategies,
                    |#bindings| #ctor,
                )
            )
        })
    }
}

/// The bindings `field_0`, `field_1`, … as the fields of `ident`. `named`
/// selects the brace or parenthesis form, and must match the prototype's
/// declaration.
fn fields_ctor(ident: TokenStream2, fields: &Fields, named: bool) -> TokenStream2 {
    if fields.is_empty() {
        // A unit struct or variant is named on its own, not called.
        return ident;
    }
    let bindings = fields
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("field_{}", i));
    if named {
        let names = fields.iter().map(|field| &field.ident);
        quote! { #ident { #(#names: #bindings,)* } }
    } else {
        quote! { #ident ( #(#bindings,)* ) }
    }
}

/// A strategy choosing between the variants of a prototype enum.
///
/// `Union` rather than `prop_oneof!`, which is limited to ten arms. Each variant
/// is built with the whole budget, since only one is generated.
fn union_strategy(path: &Path, arms: Vec<TokenStream2>) -> TokenStream2 {
    quote! {
        #path::testutils::proptest::proptest::strategy::Strategy::boxed(
            #path::testutils::proptest::proptest::strategy::Union::new(
                #path::testutils::arbitrary::std::vec![ #(#arms,)* ]
            )
        )
    }
}

pub fn derive_arbitrary_struct(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    data: &DataStruct,
) -> TokenStream2 {
    derive_arbitrary_struct_common(path, vis, ident, data, FieldType::Named)
}

pub fn derive_arbitrary_struct_tuple(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    data: &DataStruct,
) -> TokenStream2 {
    derive_arbitrary_struct_common(path, vis, ident, data, FieldType::Unnamed)
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
) -> TokenStream2 {
    let arbitrary_type_ident = format_ident!("Arbitrary{}", ident);

    let arbitrary_type_fields: Vec<TokenStream2> = data
        .fields
        .iter()
        .map(|field| {
            let field_type = &field.ty;
            match &field.ident {
                Some(ident) => {
                    quote! {
                        #ident: <#field_type as #path::testutils::arbitrary::SorobanArbitrary>::Prototype
                    }
                }
                None => {
                    quote! {
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

    let proptest_strategy = fields_strategy(
        path,
        &data.fields,
        fields_ctor(
            quote! { #arbitrary_type_ident },
            &data.fields,
            matches!(field_type, FieldType::Named),
        ),
    );

    quote_arbitrary(
        path,
        vis,
        ident,
        arbitrary_type_ident,
        arbitrary_type_decl,
        arbitrary_ctor,
        proptest_strategy,
    )
}

pub fn derive_arbitrary_enum(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    data: &DataEnum,
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
                    match &field.ident {
                        Some(ident) => {
                            field_types = Some(FieldType::Named);
                            quote! {
                                #ident: <#field_type as #path::testutils::arbitrary::SorobanArbitrary>::Prototype
                            }
                        }
                        None => {
                            field_types = Some(FieldType::Unnamed);
                            quote! {
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

    let proptest_arms: Vec<TokenStream2> = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            fields_strategy(
                path,
                &variant.fields,
                fields_ctor(
                    quote! { #arbitrary_type_ident::#variant_ident },
                    &variant.fields,
                    matches!(variant.fields, Fields::Named(_)),
                ),
            )
        })
        .collect();
    let proptest_strategy = union_strategy(path, proptest_arms);

    quote_arbitrary(
        path,
        vis,
        ident,
        arbitrary_type_ident,
        arbitrary_type_decl,
        arbitrary_ctor,
        proptest_strategy,
    )
}

pub fn derive_arbitrary_enum_int(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    data: &DataEnum,
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

    let proptest_arms: Vec<TokenStream2> = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            fields_strategy(
                path,
                &variant.fields,
                quote! { #arbitrary_type_ident::#variant_ident },
            )
        })
        .collect();
    let proptest_strategy = union_strategy(path, proptest_arms);

    quote_arbitrary(
        path,
        vis,
        ident,
        arbitrary_type_ident,
        arbitrary_type_decl,
        arbitrary_ctor,
        proptest_strategy,
    )
}

#[allow(clippy::too_many_arguments)]
fn quote_arbitrary(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    arbitrary_type_ident: Ident,
    arbitrary_type_decl: TokenStream2,
    arbitrary_ctor: TokenStream2,
    proptest_strategy: TokenStream2,
) -> TokenStream2 {
    quote! {
        // This allows us to create a scope to import std and arbitrary, while
        // also keeping everything from the current scope. This is better than a
        // module because: modules inside functions have surprisingly
        // inconsistent scoping rules and visibility management is harder.
        const _: () = {
            // derive(Arbitrary) expects these two to be in scope
            use #path::testutils::arbitrary::std;
            use #path::testutils::arbitrary::arbitrary;

            #[derive(#path::testutils::arbitrary::arbitrary::Arbitrary)]
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            #vis #arbitrary_type_decl

            impl #path::testutils::arbitrary::SorobanArbitrary for #ident {
                type Prototype = #arbitrary_type_ident;
            }

            impl #path::TryFromVal<#path::Env, #arbitrary_type_ident> for #ident {
                type Error = #path::ConversionError;
                fn try_from_val(env: &#path::Env, v: &#arbitrary_type_ident) -> std::result::Result<Self, Self::Error> {
                    Ok(#arbitrary_ctor)
                }
            }

            impl #path::testutils::proptest::ProtoStrategy for #arbitrary_type_ident {
                fn proto_strategy() -> #path::testutils::proptest::proptest::strategy::BoxedStrategy<Self> {
                    #proptest_strategy
                }
            }

            impl #path::testutils::proptest::proptest::arbitrary::Arbitrary for #arbitrary_type_ident {
                type Parameters = ();
                type Strategy = #path::testutils::proptest::proptest::strategy::BoxedStrategy<Self>;
                fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
                    <Self as #path::testutils::proptest::ProtoStrategy>::proto_strategy()
                }
            }
        };
    }
}
