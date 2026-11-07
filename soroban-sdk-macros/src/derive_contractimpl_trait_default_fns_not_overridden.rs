use crate::{
    default_crate_path,
    derive_args::derive_args_impl,
    derive_client::derive_client_impl,
    derive_fn::{derive_contract_function_registration_ctor, derive_pub_fns},
    derive_spec_fn::derive_fns_spec,
    syn_ext::{self, ident_to_type},
};
use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    ext::IdentExt as _, spanned::Spanned as _, Attribute, FnArg, LitStr, Path, ReturnType, Type,
};

// See soroban-sdk/docs/contracttrait.md for documentation on how this works.

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    trait_ident: Path,
    trait_default_fns: Vec<LitStr>,
    impl_ident: Ident,
    impl_fns: Vec<LitStr>,
    client_name: String,
    args_name: String,
    spec_name: Type,
    spec_export: Option<bool>,
}

pub fn derive_contractimpl_trait_default_fns_not_overridden(
    metadata: TokenStream2,
) -> TokenStream2 {
    match derive_or_err(metadata) {
        Ok(tokens) => tokens,
        Err(err) => err.write_errors(),
    }
}

fn derive_or_err(metadata: TokenStream2) -> Result<TokenStream2, Error> {
    let args = NestedMeta::parse_meta_list(metadata.into())?;
    let args = Args::from_list(&args)?;
    let derived = derive(&args)?;
    Ok(quote! {
        #derived
    }
    .into())
}

fn derive(args: &Args) -> Result<TokenStream2, Error> {
    let impl_ty = ident_to_type(args.impl_ident.clone());
    let trait_ident = &args.trait_ident;
    let spec_export = args.spec_export.unwrap_or(true);

    let trait_default_fns = syn_ext::strs_to_fns(&args.trait_default_fns)?;
    let impl_fns = syn_ext::strs_to_fns(&args.impl_fns)?;

    // Filter the list of default fns down to only default fns that have not been redefined /
    // overridden in the input fns.
    let fns = trait_default_fns
        .into_iter()
        .filter_map(|mut f| {
            let matching_impl_fns = impl_fns
                .iter()
                .filter(|impl_fn| impl_fn.ident.unraw() == f.ident.unraw())
                .collect::<Vec<_>>();
            if matching_impl_fns.is_empty() {
                return Some(Ok(f));
            }

            for impl_fn in matching_impl_fns {
                match cfg_condition(&impl_fn.attrs) {
                    Ok(Some(cfg)) => {
                        f.attrs.push(syn::parse_quote!(#[cfg(not(#cfg))]));
                    }
                    Ok(None) => return None,
                    Err(err) => return Some(Err(err)),
                }
            }

            Some(Ok(f))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    // With no default fns left there is nothing to generate, and the empty
    // registration ctor would otherwise clash with an empty impl's own.
    if fns.is_empty() {
        return Ok(quote! {});
    }

    let mut output = quote! {};
    output.extend(derive_type_checks(
        &args.crate_path,
        &impl_ty,
        trait_ident,
        &fns,
    ));
    output.extend(derive_pub_fns(
        &args.crate_path,
        &impl_ty,
        &fns,
        Some(trait_ident),
        &args.client_name,
    ));
    output.extend(derive_fns_spec(
        &args.crate_path,
        &args.spec_name,
        &fns,
        spec_export,
    ));
    output.extend(derive_client_impl(
        &args.crate_path,
        &args.client_name,
        &fns,
    ));
    output.extend(derive_args_impl(&args.args_name, &fns));
    output.extend(derive_contract_function_registration_ctor(
        &args.crate_path,
        &impl_ty,
        Some(trait_ident),
        &fns,
    ));

    Ok(output)
}

// The default fns' signatures are carried here as strings, so the type names in
// them resolve here, at the impl, rather than where the trait is defined. The
// spec, client, and args generated for the default fns use the types the names
// resolve to here, so fail to compile if they are not the types of the trait's
// fns, which happens if the trait and the impl see different types by the same
// name. The trait's fn is passed to a fn generic over its types, so that each
// type is inferred from the trait's fn and compared to the type its name
// resolves to here, and an error names the type of the trait's that differs.
fn derive_type_checks(
    crate_path: &Path,
    impl_ty: &Type,
    trait_ident: &Path,
    fns: &[syn_ext::Fn],
) -> TokenStream2 {
    let checks = fns.iter().map(|f| {
        let ident = &f.ident;
        let cfg_attrs = f.attrs.iter().filter(|attr| attr.path().is_ident("cfg"));
        let mut params = Vec::new();
        let mut ptr_inputs = Vec::new();
        let mut bounds = Vec::new();
        let bound = |param: &Ident, named: &Type| quote!(#param: #crate_path::ContractTraitType<#named>);
        for (i, input) in f.inputs.iter().enumerate() {
            let ty = match input {
                FnArg::Typed(pat_ty) => &*pat_ty.ty,
                FnArg::Receiver(r) => &*r.ty,
            };
            let param = format_ident!("A{i}");
            let elem = match ty {
                Type::Reference(r) => {
                    ptr_inputs.push(quote!(&#param));
                    &*r.elem
                }
                ty => {
                    ptr_inputs.push(quote!(#param));
                    ty
                }
            };
            // The env is left unchecked, like the exported fn leaves it to be
            // inferred, because its name is not used and need not be in scope.
            let is_env = matches!(elem, Type::Path(p) if p.path.segments.last().is_some_and(|s| s.ident == "Env"));
            if !(i == 0 && is_env) {
                bounds.push(bound(&param, elem));
            }
            params.push(param);
        }
        let ptr_output = match &f.output {
            ReturnType::Default => quote!(),
            ReturnType::Type(_, ty) => match syn_ext::unpack_result(ty) {
                Some((t, e)) => {
                    let (t_param, e_param) = (format_ident!("T"), format_ident!("E"));
                    bounds.push(bound(&t_param, &t));
                    bounds.push(bound(&e_param, &e));
                    params.extend([t_param, e_param]);
                    quote!(-> ::core::result::Result<T, E>)
                }
                None => {
                    let r_param = format_ident!("R");
                    bounds.push(bound(&r_param, ty));
                    params.push(r_param);
                    quote!(-> R)
                }
            },
        };
        let call = quote_spanned! {trait_ident.span()=>
            check(<#impl_ty as #trait_ident>::#ident)
        };
        quote! {
            #(#cfg_attrs)*
            const _: () = {
                #[allow(dead_code)]
                fn check_types() {
                    fn check<#(#params),*>(_: fn(#(#ptr_inputs),*) #ptr_output) where #(#bounds),* {}
                    #call;
                }
            };
        }
    });
    quote! { #(#checks)* }
}

fn cfg_condition(attrs: &[Attribute]) -> Result<Option<TokenStream2>, Error> {
    let cfgs = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("cfg"))
        .map(|attr| attr.parse_args::<TokenStream2>().map_err(Error::from))
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(match cfgs.as_slice() {
        [] => None,
        [cfg] => Some(quote! { #cfg }),
        _ => Some(quote! { all(#(#cfgs),*) }),
    })
}
