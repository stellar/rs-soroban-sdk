pub mod client;
pub mod parse;
pub mod r#trait;
pub mod types;
pub mod wasm;

use proc_macro2::TokenStream;
use quote::quote;
use soroban_env_host::xdr::ScSpecEntry;

use self::types::{generate_struct, generate_union};

pub fn generate(specs: &[ScSpecEntry], wasm: &[u8]) -> TokenStream {
    let mut spec_fns = Vec::new();
    let mut spec_structs = Vec::new();
    let mut spec_unions = Vec::new();
    for s in specs {
        match s {
            ScSpecEntry::FunctionV0(f) => spec_fns.push(f),
            ScSpecEntry::UdtStructV0(s) => spec_structs.push(s),
            ScSpecEntry::UdtUnionV0(u) => spec_unions.push(u),
        }
    }
    // let client = client::generate("Client", &spec_fns);
    let wasm_consts = wasm::generate_consts(wasm);
    let trait_ = r#trait::generate_trait("Contract", &spec_fns);
    let structs = spec_structs.iter().map(|s| generate_struct(s));
    let unions = spec_unions.iter().map(|s| generate_union(s));
    quote! {
        #wasm_consts
        #trait_
        // #client
        #(#structs)*
        #(#unions)*
    }
}
