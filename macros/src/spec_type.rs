use proc_macro2::TokenStream as TokenStream2;

use syn::{Fields, Ident};

#[allow(clippy::too_many_lines)]
pub fn spec_type(_ident: &Ident, _inputs: &Fields) -> TokenStream2 {
    todo!()
    // // Collect errors as they are encountered and emit them at the end.
    // let mut errors = Vec::<Error>::new();

    // // If errors have occurred, render them instead.
    // if !errors.is_empty() {
    //     let compile_errors = errors.iter().map(Error::to_compile_error);
    //     return quote! { #(#compile_errors)* };
    // }

    // // Generated code spec.
    // let spec_entry_fn = SpecEntryFunctionV0 {
    //     name: wrap_export_name.clone().try_into().unwrap(),
    //     input_types: spec_args.try_into().unwrap(),
    //     output_types: spec_result.try_into().unwrap(),
    // };
    // let spec_entry = SpecEntry::Function(SpecEntryFunction::V0(spec_entry_fn));
    // let spec_xdr = spec_entry.to_xdr().unwrap();
    // let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
    // let spec_xdr_len = spec_xdr.len();
    // let spec_ident = format_ident!("__SPEC_XDR_{}", ident.to_string().to_uppercase());

    // // Generated code.
    // quote! {
    //     #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
    //     pub static #spec_ident: [u8; #spec_xdr_len] = *#spec_xdr_lit;

    //     #[export_name = #wrap_export_name]
    //     fn #wrap_ident(env: stellar_contract_sdk::Env, #(#wrap_args),*) -> stellar_contract_sdk::RawVal {
    //         <_ as stellar_contract_sdk::IntoVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal>>::into_val(
    //             #call(
    //                 #env_call
    //                 #(#wrap_calls),*
    //             ),
    //             &env
    //         )
    //     }
    // }
}
