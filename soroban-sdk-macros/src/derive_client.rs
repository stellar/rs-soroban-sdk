use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Error, FnArg, Path, Type, TypePath, TypeReference};

use crate::syn_ext;

pub fn derive_client_type(crate_path: &Path, ty: &str, name: &str) -> TokenStream {
    let ty_str = quote!(#ty).to_string();
    // Render the Client.
    let client_doc = format!("{name} is a client for calling the contract defined in {ty_str}.");
    let client_ident = format_ident!("{}", name);
    quote! {
        #[doc = #client_doc]
        pub struct #client_ident<'a> {
            pub env: #crate_path::Env,
            pub address: #crate_path::Address,
            #[doc(hidden)]
            #[cfg(not(any(test, feature = "testutils")))]
            _phantom: core::marker::PhantomData<&'a ()>,
            #[doc(hidden)]
            #[cfg(any(test, feature = "testutils"))]
            set_auths: Option<&'a [#crate_path::xdr::SorobanAuthorizationEntry]>,
            #[doc(hidden)]
            #[cfg(any(test, feature = "testutils"))]
            mock_auths: Option<&'a [#crate_path::testutils::MockAuth<'a>]>,
            #[doc(hidden)]
            #[cfg(any(test, feature = "testutils"))]
            mock_all_auths: bool,
            #[doc(hidden)]
            #[cfg(any(test, feature = "testutils"))]
            allow_non_root_auth: bool,
        }

        impl<'a> #client_ident<'a> {
            pub fn new(env: &#crate_path::Env, address: &#crate_path::Address) -> Self {
                Self {
                    env: env.clone(),
                    address: address.clone(),
                    #[cfg(not(any(test, feature = "testutils")))]
                    _phantom: core::marker::PhantomData,
                    #[cfg(any(test, feature = "testutils"))]
                    set_auths: None,
                    #[cfg(any(test, feature = "testutils"))]
                    mock_auths: None,
                    #[cfg(any(test, feature = "testutils"))]
                    mock_all_auths: false,
                    #[cfg(any(test, feature = "testutils"))]
                    allow_non_root_auth: false,
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
            #[cfg(any(test, feature = "testutils"))]
            pub fn set_auths(&self, auths: &'a [#crate_path::xdr::SorobanAuthorizationEntry]) -> Self {
                Self {
                    env: self.env.clone(),
                    address: self.address.clone(),
                    set_auths: Some(auths),
                    mock_auths: self.mock_auths.clone(),
                    mock_all_auths: false,
                    allow_non_root_auth: false,
                }
            }

            /// Mock authorizations in the environment which will cause matching invokes
            /// of `Address::require_auth` and `Address::require_auth_for_args` to
            /// pass.
            ///
            /// See `soroban_sdk::Env::set_auths` for more details and examples.
            #[cfg(any(test, feature = "testutils"))]
            pub fn mock_auths(&self, mock_auths: &'a [#crate_path::testutils::MockAuth<'a>]) -> Self {
                Self {
                    env: self.env.clone(),
                    address: self.address.clone(),
                    set_auths: self.set_auths.clone(),
                    mock_auths: Some(mock_auths),
                    mock_all_auths: false,
                    allow_non_root_auth: false,
                }
            }

            /// Mock all calls to the `Address::require_auth` and
            /// `Address::require_auth_for_args` functions in invoked contracts,
            /// having them succeed as if authorization was provided.
            ///
            /// See `soroban_sdk::Env::mock_all_auths` for more details and
            /// examples.
            #[cfg(any(test, feature = "testutils"))]
            pub fn mock_all_auths(&self) -> Self {
                Self {
                    env: self.env.clone(),
                    address: self.address.clone(),
                    set_auths: None,
                    mock_auths: None,
                    mock_all_auths: true,
                    allow_non_root_auth: false,
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
            #[cfg(any(test, feature = "testutils"))]
            pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
                Self {
                    env: self.env.clone(),
                    address: self.address.clone(),
                    set_auths: None,
                    mock_auths: None,
                    mock_all_auths: true,
                    allow_non_root_auth: true,
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
        .map(|f| {
            let fn_ident = &f.ident;
            let fn_try_ident = format_ident!("try_{}", &f.ident);
            let fn_name = fn_ident.to_string();

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
            let fn_try_output = f.try_output(crate_path);
            let fn_attrs = f.attrs;
            quote! {
                #(#fn_attrs)*
                pub fn #fn_ident(&self, #(#fn_input_types),*) -> #fn_output {
                    use core::ops::Not;
                    #[cfg(any(test, feature = "testutils"))]
                    let old_auth_manager = self.env.in_contract().not().then(||
                        self.env.host().snapshot_auth_manager().unwrap()
                    );
                    #[cfg(any(test, feature = "testutils"))]
                    {
                        if let Some(set_auths) = self.set_auths {
                            self.env.set_auths(set_auths);
                        }
                        if let Some(mock_auths) = self.mock_auths {
                            self.env.mock_auths(mock_auths);
                        }
                        if self.mock_all_auths {
                            if self.allow_non_root_auth {
                                self.env.mock_all_auths_allowing_non_root_auth();
                            } else {
                                self.env.mock_all_auths();
                            }
                        }
                    }
                    use #crate_path::{IntoVal,FromVal};
                    let res = self.env.invoke_contract(
                        &self.address,
                        &#crate_path::Symbol::new(&self.env, &#fn_name),
                        #crate_path::vec![&self.env, #(#fn_input_names.into_val(&self.env)),*],
                    );
                    #[cfg(any(test, feature = "testutils"))]
                    if let Some(old_auth_manager) = old_auth_manager {
                        self.env.host().set_auth_manager(old_auth_manager).unwrap();
                    }
                    res
                }

                #(#fn_attrs)*
                pub fn #fn_try_ident(&self, #(#fn_input_types),*) -> #fn_try_output {
                    #[cfg(any(test, feature = "testutils"))]
                    use core::ops::Not;
                    #[cfg(any(test, feature = "testutils"))]
                    let old_auth_manager = self.env.in_contract().not().then(||
                        self.env.host().snapshot_auth_manager().unwrap()
                    );
                    #[cfg(any(test, feature = "testutils"))]
                    {
                        if let Some(set_auths) = self.set_auths {
                            self.env.set_auths(set_auths);
                        }
                        if let Some(mock_auths) = self.mock_auths {
                            self.env.mock_auths(mock_auths);
                        }
                        if self.mock_all_auths {
                            self.env.mock_all_auths();
                        }
                    }
                    use #crate_path::{IntoVal,FromVal};
                    let res = self.env.try_invoke_contract(
                        &self.address,
                        &#crate_path::Symbol::new(&self.env, &#fn_name),
                        #crate_path::vec![&self.env, #(#fn_input_names.into_val(&self.env)),*],
                    );
                    #[cfg(any(test, feature = "testutils"))]
                    if let Some(old_auth_manager) = old_auth_manager {
                        self.env.host().set_auth_manager(old_auth_manager).unwrap();
                    }
                    res
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
