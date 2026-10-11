use super::*;

pub(crate) fn derive_contract_data(metadata: TokenStream, input: TokenStream) -> TokenStream {
    use syn::{parse::Parser, punctuated::Punctuated, Meta, Token};

    let mut item = parse_macro_input!(input as DeriveInput);
    let ident = &item.ident;
    let contracttype_path: Path = syn::parse_str("soroban_sdk").unwrap();
    let contracttype_derived = match is_mapped_type_udt(ident, &item.generics) {
        Ok(()) => match &item.data {
            Data::Struct(s) => match s.fields {
                Fields::Named(_) => {
                    derive_type_struct(&contracttype_path, &item.vis, ident, &item.attrs, s)
                }
                Fields::Unnamed(_) => {
                    derive_type_struct_tuple(&contracttype_path, &item.vis, ident, &item.attrs, s)
                }
                Fields::Unit => Error::new(
                    s.fields.span(),
                    "unit structs are not supported as contract data",
                )
                .to_compile_error(),
            },
            Data::Enum(e) => {
                let count_of_variants = e.variants.len();
                let count_of_int_variants = e
                    .variants
                    .iter()
                    .filter(|v| v.discriminant.is_some())
                    .count();
                if count_of_int_variants == 0 {
                    derive_type_enum(&contracttype_path, &item.vis, ident, &item.attrs, e)
                } else if count_of_int_variants == count_of_variants {
                    derive_type_enum_int(&contracttype_path, &item.vis, ident, &item.attrs, e)
                } else {
                    Error::new(item.span(), "enums are supported as contract data only when all variants have an explicit integer literal, or when all variants are unit or single field")
                        .to_compile_error()
                }
            }
            Data::Union(u) => Error::new(
                u.union_token.span(),
                "unions are unsupported as contract data",
            )
            .to_compile_error(),
        },
        Err(e) => return e.to_compile_error().into(),
    };
    let metas = match Punctuated::<Meta, Token![,]>::parse_terminated.parse(metadata) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };
    let mut storage = None;
    let mut key = None;
    let mut namespace = None;
    let mut ttl = quote!(soroban_sdk::TtlPolicy::None);
    let mut has_ttl = false;
    for meta in metas {
        let Meta::NameValue(value) = meta else {
            return Error::new(
                meta.span(),
                "expected `storage = ...`, `ttl = ...`, `key = \"...\"`, or `namespace = \"...\"`",
            )
            .to_compile_error()
            .into();
        };
        match value.path.get_ident().map(|i| i.to_string()).as_deref() {
            Some("storage") => {
                let Expr::Path(path) = value.value else {
                    return Error::new(
                        value.span(),
                        "storage must be persistent, temporary, or instance",
                    )
                    .to_compile_error()
                    .into();
                };
                storage = path.path.get_ident().map(|i| i.to_string());
            }
            Some("key") => {
                let Expr::Lit(lit) = value.value else {
                    return Error::new(value.span(), "key must be a string literal")
                        .to_compile_error()
                        .into();
                };
                if let syn::Lit::Str(s) = lit.lit {
                    key = Some(s);
                } else {
                    return Error::new(lit.span(), "key must be a string literal")
                        .to_compile_error()
                        .into();
                }
            }
            Some("namespace") => {
                let Expr::Lit(lit) = value.value else {
                    return Error::new(value.span(), "namespace must be a string literal")
                        .to_compile_error()
                        .into();
                };
                if let syn::Lit::Str(s) = lit.lit {
                    namespace = Some(s);
                } else {
                    return Error::new(lit.span(), "namespace must be a string literal")
                        .to_compile_error()
                        .into();
                }
            }
            Some("ttl") => {
                has_ttl = true;
                let Expr::Call(call) = value.value else {
                    return Error::new(
                        value.span(),
                        "ttl must be `bump_when_below(threshold, extend_to)`",
                    )
                    .to_compile_error()
                    .into();
                };
                let Expr::Path(function) = call.func.as_ref() else {
                    return Error::new(
                        call.func.span(),
                        "ttl must be `bump_when_below(threshold, extend_to)`",
                    )
                    .to_compile_error()
                    .into();
                };
                ttl = if function.path.is_ident("bump_when_below") && call.args.len() == 2 {
                    let threshold = &call.args[0];
                    let extend_to = &call.args[1];
                    quote! {
                        soroban_sdk::TtlPolicy::BumpWhenBelow {
                            threshold: #threshold,
                            extend_to: #extend_to,
                        }
                    }
                } else if function.path.is_ident("bump_to") && call.args.len() == 1 {
                    let extend_to = &call.args[0];
                    quote! { soroban_sdk::TtlPolicy::BumpTo { extend_to: #extend_to } }
                } else {
                    return Error::new(
                        call.span(),
                        "ttl must be `bump_to(extend_to)` or `bump_when_below(threshold, extend_to)`",
                    )
                    .to_compile_error()
                    .into();
                };
            }
            _ => {
                return Error::new(value.path.span(), "unsupported contractdata argument")
                    .to_compile_error()
                    .into()
            }
        }
    }
    let storage_is_instance = storage.as_deref() == Some("instance");
    let storage = match storage.as_deref() {
        Some("persistent") => quote!(soroban_sdk::StorageType::Persistent),
        Some("temporary") => quote!(soroban_sdk::StorageType::Temporary),
        Some("instance") => quote!(soroban_sdk::StorageType::Instance),
        _ => {
            return Error::new(
                ident.span(),
                "storage must be persistent, temporary, or instance",
            )
            .to_compile_error()
            .into()
        }
    };
    if storage_is_instance && has_ttl {
        return Error::new(
            ident.span(),
            "`ttl` is not supported with `storage = instance`; extend the contract instance TTL explicitly with `env.storage().instance().extend_ttl(...)`",
        )
        .to_compile_error()
        .into();
    }
    if key.is_some() && namespace.is_some() {
        return Error::new(
            ident.span(),
            "`key` and `namespace` cannot be used together",
        )
        .to_compile_error()
        .into();
    }
    let key = if namespace.is_none() {
        key.or_else(|| {
            Some(LitStr::new(
                &ident.unraw().to_string().to_snake_case(),
                ident.span(),
            ))
        })
    } else {
        key
    };
    let fixed_contract_data = if let Some(key) = key {
        quote! {
            impl soroban_sdk::FixedContractData for #ident {
                const KEY: &'static str = #key;
            }
        }
    } else {
        quote!()
    };
    let dynamic_contract_data = if let Some(namespace) = namespace {
        let syn::Data::Struct(data) = &item.data else {
            return Error::new(ident.span(), "namespace is supported only on structs")
                .to_compile_error()
                .into();
        };
        let syn::Fields::Named(fields) = &data.fields else {
            return Error::new(
                data.fields.span(),
                "namespace requires a struct with named fields",
            )
            .to_compile_error()
            .into();
        };
        let key_fields = fields
            .named
            .iter()
            .filter(|field| field.attrs.iter().any(|attr| attr.path().is_ident("key")))
            .collect::<Vec<_>>();
        if key_fields.is_empty() {
            return Error::new(
                ident.span(),
                "namespace requires at least one field marked #[key]",
            )
            .to_compile_error()
            .into();
        }
        let key_idents = key_fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap().clone())
            .collect::<Vec<_>>();
        let key_types = key_fields
            .iter()
            .map(|field| field.ty.clone())
            .collect::<Vec<_>>();
        let (args_type, borrowed_key_type, destructure_key_args, destructure_owned_args) =
            if key_fields.len() == 1 {
                let key_ident = &key_idents[0];
                let key_type = &key_types[0];
                (
                    quote!(#key_type),
                    quote!((soroban_sdk::Symbol, &'a #key_type)),
                    quote!(let #key_ident = args;),
                    quote!(let #key_ident = args;),
                )
            } else {
                (
                    quote!((#(#key_types,)*)),
                    quote!((soroban_sdk::Symbol, #(&'a #key_types,)*)),
                    quote!(let (#(#key_idents,)*) = args;),
                    quote!(let (#(#key_idents,)*) = args;),
                )
            };
        let data_fields = fields
            .named
            .iter()
            .filter(|field| !field.attrs.iter().any(|attr| attr.path().is_ident("key")))
            .collect::<Vec<_>>();
        let data_idents = data_fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap().clone())
            .collect::<Vec<_>>();
        let data_types = data_fields
            .iter()
            .map(|field| field.ty.clone())
            .collect::<Vec<_>>();
        let stored_ident = format_ident!("{}ContractDataStorage", ident);
        attribute::remove_attributes_from_item(&mut item.data, &["key"]);
        quote! {
            #[soroban_sdk::contracttype]
            struct #stored_ident {
                #(#data_idents: #data_types,)*
            }

            impl soroban_sdk::ContractDataKey for #ident {
                type Args = #args_type;
                type Key<'a> = #borrowed_key_type where Self: 'a;
                type Stored = #stored_ident;

                fn key<'a>(
                    env: &soroban_sdk::Env,
                    args: &'a Self::Args,
                ) -> Self::Key<'a> {
                    #destructure_key_args
                    (
                        soroban_sdk::Symbol::new(env, #namespace),
                        #(#key_idents,)*
                    )
                }

                fn key_from_value(&self, env: &soroban_sdk::Env) -> Self::Key<'_> {
                    (
                        soroban_sdk::Symbol::new(env, #namespace),
                        #(&self.#key_idents,)*
                    )
                }

                fn into_stored(&self) -> Self::Stored {
                    #stored_ident {
                        #(#data_idents: self.#data_idents.clone(),)*
                    }
                }

                fn from_stored(args: &Self::Args, stored: Self::Stored) -> Self {
                    #destructure_owned_args
                    Self {
                        #(#key_idents: #key_idents.clone(),)*
                        #(#data_idents: stored.#data_idents,)*
                    }
                }
            }
        }
    } else {
        quote!()
    };
    quote! {
        #item
        #contracttype_derived
        impl soroban_sdk::ContractData for #ident {
            const STORAGE: soroban_sdk::StorageType = #storage;
            const TTL: soroban_sdk::TtlPolicy = #ttl;
        }
        #fixed_contract_data
        #dynamic_contract_data
    }
    .into()
}
