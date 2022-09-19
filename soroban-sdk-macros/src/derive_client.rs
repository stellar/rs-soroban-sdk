use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    AngleBracketedGenericArguments, Attribute, Error, FnArg, Ident, ItemImpl, ItemTrait,
    PathArguments, PathSegment, ReturnType, Token, Type, TypePath,
};

use crate::syn_ext;

pub enum ClientItem {
    Trait(ItemTrait),
    Impl(ItemImpl),
}

impl ClientItem {
    pub fn fns(&'_ self) -> Vec<ClientFn> {
        match self {
            ClientItem::Trait(t) => syn_ext::trait_methods(t)
                .map(|m| ClientFn {
                    ident: &m.sig.ident,
                    inputs: &m.sig.inputs,
                    output: &m.sig.output,
                })
                .collect(),
            ClientItem::Impl(i) => syn_ext::impl_pub_methods(i)
                .map(|m| ClientFn {
                    ident: &m.sig.ident,
                    inputs: &m.sig.inputs,
                    output: &m.sig.output,
                })
                .collect(),
        }
    }
}

impl Parse for ClientItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.call(Attribute::parse_outer);
        _ = input.parse::<Token![pub]>();
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![trait]) {
            input.parse().map(ClientItem::Trait)
        } else if lookahead.peek(Token![impl]) {
            input.parse().map(ClientItem::Impl)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for ClientItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ClientItem::Trait(t) => t.to_tokens(tokens),
            ClientItem::Impl(i) => i.to_tokens(tokens),
        }
    }
}

pub struct ClientFn<'a> {
    pub ident: &'a Ident,
    pub inputs: &'a Punctuated<FnArg, Comma>,
    pub output: &'a ReturnType,
}

impl<'a> ClientFn<'a> {
    pub fn output(&self) -> Type {
        match self.output {
            ReturnType::Default => Type::Verbatim(quote!(())),
            ReturnType::Type(_, typ) => {
                let typ = *typ.clone();
                let unwrapped_type = match &typ {
                    Type::Path(TypePath { path, .. }) => {
                        if let Some(PathSegment {
                            ident,
                            arguments:
                                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                    args,
                                    ..
                                }),
                        }) = path.segments.last()
                        {
                            let args = args.iter().collect::<Vec<_>>();
                            match (&ident.to_string()[..], args.as_slice()) {
                                ("Result", [t, _]) => Some(Type::Verbatim(quote!(#t))),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some(unwrapped_type) = unwrapped_type {
                    unwrapped_type
                } else {
                    Type::Verbatim(quote!(#typ))
                }
            }
        }
    }
    pub fn try_output(&self) -> Type {
        match self.output {
            ReturnType::Default => Type::Verbatim(quote!(Result<(), Result<Status, Status>)),
            ReturnType::Type(_, typ) => {
                let typ = *typ.clone();
                let result_type = match &typ {
                    Type::Path(TypePath { path, .. }) => {
                        if let Some(PathSegment {
                            ident,
                            arguments:
                                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                    args,
                                    ..
                                }),
                        }) = path.segments.last()
                        {
                            let args = args.iter().collect::<Vec<_>>();
                            match (&ident.to_string()[..], args.as_slice()) {
                                ("Result", [t, e]) => Some(Type::Verbatim(quote! {
                                    Result<
                                        Result<
                                            #t,
                                            <#t as ::soroban_sdk::TryFromVal<::soroban_sdk::Env, ::soroban_sdk::RawVal>>::Error
                                        >,
                                        Result<
                                            #e,
                                            <#e as TryFrom<::soroban_sdk::Status>>::Error
                                        >
                                    >
                                })),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some(result_type) = result_type {
                    result_type
                } else {
                    Type::Verbatim(quote! {
                        Result<
                            Result<
                                #typ,
                                <#typ as ::soroban_sdk::TryFromVal<::soroban_sdk::Env, ::soroban_sdk::RawVal>>::Error
                            >,
                            Result<
                                ::soroban_sdk::Status,
                                <Status as TryFrom<::soroban_sdk::Status>>::Error
                            >
                        >
                    })
                }
            }
        }
        // Result<
        //     Result<
        //         #fn_output_type,
        //         <#fn_output_type as ::soroban_sdk::TryFromVal<::soroban_sdk::Env, ::soroban_sdk::RawVal>>::Error
        //     >,
        //     Result<
        //         #fn_output_type,
        //         <#fn_output_type as ::soroban_sdk::TryFromVal<::soroban_sdk::Env, ::soroban_sdk::RawVal>>::Error
        //     >,
        //     ::soroban_sdk::Status
        // >,
    }
}

pub fn derive_client(name: &str, fns: &[ClientFn]) -> TokenStream {
    // Map the traits methods to methods for the Client.
    let mut errors = Vec::<Error>::new();
    let fns: Vec<_> = fns
        .iter()
        .map(|f| {
            let fn_ident = &f.ident;
            let fn_try_ident = format_ident!("try_{}", &f.ident);
            let fn_name = fn_ident.to_string();

            // Check for the Env argument.
            let env_input = f.inputs.first().and_then(|a| match a {
                FnArg::Typed(pat_type) => {
                    let ty = &*pat_type.ty;
                    if let Type::Path(TypePath {
                        path: syn::Path { segments, .. },
                        ..
                    }) = ty
                    {
                        if segments.last().map_or(false, |s| s.ident == "Env") {
                            Some(a)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                FnArg::Receiver(_) => None,
            });

            // Map all remaining inputs.
            let (fn_input_names, fn_input_types): (Vec<_>, Vec<_>) = f
                .inputs
                .iter()
                .skip(if env_input.is_some() { 1 } else { 0 })
                .map(|t| {
                    let ident = match syn_ext::fn_arg_ident(t) {
                        Ok(ident) => ident,
                        Err(_) => {
                            errors.push(Error::new(t.span(), "argument not supported"));
                            format_ident!("")
                        }
                    };
                    (ident, syn_ext::fn_arg_make_ref(t))
                })
                .unzip();
            let fn_output = f.output();
            let fn_try_output = f.try_output();
            quote! {
                pub fn #fn_ident(&self, #(#fn_input_types),*) -> #fn_output {
                    use ::soroban_sdk::IntoVal;
                    self.env.invoke_contract(
                        &self.contract_id,
                        &::soroban_sdk::symbol!(#fn_name),
                        ::soroban_sdk::vec![&self.env, #(#fn_input_names.into_val(&self.env)),*],
                    )
                }

                pub fn #fn_try_ident(&self, #(#fn_input_types),*) -> #fn_try_output {
                    use ::soroban_sdk::IntoVal;
                    self.env.try_invoke_contract(
                        &self.contract_id,
                        &::soroban_sdk::symbol!(#fn_name),
                        ::soroban_sdk::vec![&self.env, #(#fn_input_names.into_val(&self.env)),*],
                    )
                }
            }
        })
        .collect();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Render the Client.
    let client_ident = format_ident!("{}", name);
    quote! {
        pub struct #client_ident {
            env: ::soroban_sdk::Env,
            contract_id: ::soroban_sdk::BytesN<32>,
        }
        impl #client_ident {
            pub fn new(env: &::soroban_sdk::Env, contract_id: impl ::soroban_sdk::IntoVal<::soroban_sdk::Env, ::soroban_sdk::BytesN<32>>) -> Self {
                Self {
                    env: env.clone(),
                    contract_id: contract_id.into_val(env),
                }
            }

            #(#fns)*
        }
    }
}
