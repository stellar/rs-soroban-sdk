use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, FnArg, Ident, LitStr, Path, Type, TypePath, TypeReference};

use syn::ext::IdentExt as _;

use crate::{
    attribute::{is_attr_doc, pass_through_attr_to_gen_code},
    symbol, syn_ext,
};

/// Emits the `client::Type` impl for a user-defined type, which a client takes as
/// a borrow and returns as itself.
pub fn client_type_gen(path: &Path, ident: &Ident) -> TokenStream {
    quote! {
        impl #path::client::Type for #ident {
            type Arg = #path::client::ArgByBorrow;
            type Ret = Self;
        }
    }
}

pub fn derive_client_type(crate_path: &Path, ty: &str, name: &str) -> TokenStream {
    let ty_str = quote!(#ty).to_string();
    // Render the Client.
    let client_doc = format!("{name} is a client for calling the contract defined in {ty_str}.");
    let client_ident = format_ident!("{}", name);
    if cfg!(not(feature = "testutils")) {
        quote! {
            #[doc = #client_doc]
            pub struct #client_ident<'a> {
                pub env: #crate_path::Env,
                pub address: #crate_path::Address,
                #[doc(hidden)]
                _phantom: core::marker::PhantomData<&'a ()>,
            }

            impl<'a> #client_ident<'a> {
                pub fn new(env: &#crate_path::Env, address: &#crate_path::Address) -> Self {
                    Self {
                        env: env.clone(),
                        address: address.clone(),
                        _phantom: core::marker::PhantomData,
                    }
                }
            }
        }
    } else {
        quote! {
            #[doc = #client_doc]
            pub struct #client_ident<'a> {
                pub env: #crate_path::Env,
                pub address: #crate_path::Address,
                #[doc(hidden)]
                config: #crate_path::testutils::ClientInternalConfig<'a>,
            }

            impl<'a> #client_ident<'a> {
                pub fn new(env: &#crate_path::Env, address: &#crate_path::Address) -> Self {
                    Self {
                        env: env.clone(),
                        address: address.clone(),
                        config: #crate_path::testutils::ClientInternalConfig::default(),
                    }
                }

                /// Set authorizations in the environment which will be consumed by
                /// contracts when they invoke `Address::require_auth` or
                /// `Address::require_auth_for_args` functions.
                ///
                /// Requires valid signatures for the authorization to be successful.
                /// To mock auth without requiring valid signatures, use `mock_auths`.
                ///
                /// See `soroban_sdk::Env::set_auths` for more details and examples.
                pub fn set_auths(&self, auths: &'a [#crate_path::xdr::SorobanAuthorizationEntry]) -> Self {
                    Self {
                        env: self.env.clone(),
                        address: self.address.clone(),
                        config: self.config.set_auths(auths),
                    }
                }

                /// Mock authorizations in the environment which will cause matching invokes
                /// of `Address::require_auth` and `Address::require_auth_for_args` to
                /// pass.
                ///
                /// See `soroban_sdk::Env::set_auths` for more details and examples.
                pub fn mock_auths(&self, mock_auths: &'a [#crate_path::testutils::MockAuth<'a>]) -> Self {
                    Self {
                        env: self.env.clone(),
                        address: self.address.clone(),
                        config: self.config.mock_auths(mock_auths),
                    }
                }

                /// Mock all calls to the `Address::require_auth` and
                /// `Address::require_auth_for_args` functions in invoked contracts,
                /// having them succeed as if authorization was provided.
                ///
                /// See `soroban_sdk::Env::mock_all_auths` for more details and
                /// examples.
                pub fn mock_all_auths(&self) -> Self {
                    Self {
                        env: self.env.clone(),
                        address: self.address.clone(),
                        config: self.config.mock_all_auths(),
                    }
                }

                /// A version of `mock_all_auths` that allows authorizations that
                /// are not present in the root invocation.
                ///
                /// Refer to `mock_all_auths` documentation for details and
                /// prefer using `mock_all_auths` unless non-root authorization is
                /// required.
                ///
                /// See `soroban_sdk::Env::mock_all_auths_allowing_non_root_auth`
                /// for more details and examples.
                pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
                    Self {
                        env: self.env.clone(),
                        address: self.address.clone(),
                        config: self.config.mock_all_auths_allowing_non_root_auth(),
                    }
                }
            }
        }
    }
}

pub fn derive_client_impl(crate_path: &Path, name: &str, fns: &[syn_ext::Fn]) -> TokenStream {
    // Map the traits methods to methods for the Client.
    let mut errors = Vec::<Error>::new();
    let fns: Vec<_> = fns
        .iter()
        .filter(|f| {
            // Skip generating client functions for calling contract functions
            // that start with '__', because the Soroban Env won't let those
            // functions be invoked directly as they're reserved for callbacks
            // and hooks. Check the Soroban-facing name so a raw-identifier
            // spelling like `r#__check_auth` can't slip past this filter and then
            // still export as `__check_auth`.
            !f.ident.unraw().to_string().starts_with("__")
        })
        .map(|f| {
            let fn_ident = &f.ident;
            let fn_name = fn_ident.unraw().to_string();
            let fn_try_ident = format_ident!("try_{}", &fn_name);
            let fn_name_symbol = symbol::short_or_long(
                crate_path,
                quote!(&self.env),
                &LitStr::new(&fn_name, fn_ident.span()),
            );

            // Check for the Env argument.
            let env_input = f.inputs.first().and_then(|a| match a {
                FnArg::Typed(pat_type) => {
                    let mut ty = &*pat_type.ty;
                    if let Type::Reference(TypeReference { elem, .. }) = ty {
                        ty = elem;
                    }
                    if let Type::Path(TypePath {
                        path: syn::Path { segments, .. },
                        ..
                    }) = ty
                    {
                        if segments.last().map_or(false, |s| s.ident == "Env") {
                            Some(())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                FnArg::Receiver(_) => None,
            });

            // Map all remaining inputs. The client takes each in the form its
            // type's `client::Type` gives, as a reference or as anything that
            // converts into it.
            let (fn_input_types, fn_input_conversions): (Vec<_>, Vec<_>) = f
                .inputs
                .iter()
                .skip(if env_input.is_some() { 1 } else { 0 })
                .map(|t| {
                    let ident = match syn_ext::fn_arg_ident(t) {
                        Ok(ident) => ident,
                        Err(e) => {
                            errors.push(e);
                            format_ident!("_")
                        }
                    };
                    let ty = match t {
                        FnArg::Typed(pat_type) => syn_ext::type_unwrap_ref(*pat_type.ty.clone()),
                        FnArg::Receiver(_) => Type::Verbatim(quote!(Self)),
                    };
                    let input_type = quote! {
                        #ident: impl #crate_path::client::Arg<#ty>
                    };
                    let conversion = quote! {
                        #crate_path::client::Arg::<#ty>::into_val_for_client(#ident, &self.env)
                    };
                    (input_type, conversion)
                })
                .multiunzip();
            let fn_output = f.output(crate_path);
            let fn_try_output = f.try_output(crate_path);
            let (fn_doc_attrs, fn_attrs): (Vec<_>, Vec<_>) = f
                .attrs
                .iter()
                .filter(|attr| pass_through_attr_to_gen_code(attr))
                .partition(|attr| is_attr_doc(attr));
            let fn_call_doc = format!(" Call `{fn_name}` function on the contract.");
            let fn_contract_docs = (!fn_doc_attrs.is_empty()).then(|| {
                quote! {
                    #[doc = ""]
                    #(#fn_doc_attrs)*
                }
            });
            let fn_try_doc =
                format!(" Use [`Self::{fn_try_ident}`] to receive errors instead of panicking.");
            let fn_docs = quote! {
                #[doc = #fn_call_doc]
                #fn_contract_docs
                #[doc = ""]
                #[doc = " # Panics"]
                #[doc = ""]
                #[doc = " Panics if the invoked contract returns an error or panics."]
                #[doc = ""]
                #[doc = #fn_try_doc]
            };
            let fn_try_docs = quote! {
                #[doc = #fn_call_doc]
                #fn_contract_docs
                #[doc = ""]
                #[doc = " # Errors"]
                #[doc = ""]
                #[doc = " Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller."]
            };
            if cfg!(not(feature = "testutils")) {
                quote! {
                    #fn_docs
                    #(#fn_attrs)*
                    pub fn #fn_ident(&self, #(#fn_input_types),*) -> #fn_output {
                        use core::ops::Not;
                        use #crate_path::{IntoVal,FromVal};
                        let res = self.env.invoke_contract(
                            &self.address,
                            &#fn_name_symbol,
                            #crate_path::vec![&self.env, #(#fn_input_conversions),*],
                        );
                        res
                    }

                    #fn_try_docs
                    #(#fn_attrs)*
                    pub fn #fn_try_ident(&self, #(#fn_input_types),*) -> #fn_try_output {
                        use #crate_path::{IntoVal,FromVal};
                        let res = self.env.try_invoke_contract(
                            &self.address,
                            &#fn_name_symbol,
                            #crate_path::vec![&self.env, #(#fn_input_conversions),*],
                        );
                        res
                    }
                }
            } else {
                quote! {
                    #fn_docs
                    #(#fn_attrs)*
                    pub fn #fn_ident(&self, #(#fn_input_types),*) -> #fn_output {
                        // Exists only to be dropped at the end of this fn, restoring the auth
                        // manager even if the call panics. Binding it to `_` would drop it early.
                        let _call_scope = #crate_path::testutils::ClientCallScope::enter(&self.env, self.config);
                        use #crate_path::{IntoVal,FromVal};
                        let res = self.env.invoke_contract(
                            &self.address,
                            &#fn_name_symbol,
                            #crate_path::vec![&self.env, #(#fn_input_conversions),*],
                        );
                        res
                    }

                    #fn_try_docs
                    #(#fn_attrs)*
                    pub fn #fn_try_ident(&self, #(#fn_input_types),*) -> #fn_try_output {
                        // Exists only to be dropped at the end of this fn, restoring the auth
                        // manager even if the call panics. Binding it to `_` would drop it early.
                        let _call_scope = #crate_path::testutils::ClientCallScope::enter(&self.env, self.config);
                        use #crate_path::{IntoVal,FromVal};
                        let res = self.env.try_invoke_contract(
                            &self.address,
                            &#fn_name_symbol,
                            #crate_path::vec![&self.env, #(#fn_input_conversions),*],
                        );
                        res
                    }
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
        impl<'a> #client_ident<'a> {
            #(#fns)*
        }
    }
}
