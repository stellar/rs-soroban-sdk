use proc_macro2::TokenStream;
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::ScSpecEntry;

use crate::{
    r#trait::generate_function,
    types::{generate_enum, generate_error_enum, generate_event, generate_struct, generate_union},
};

/// Generate a single item for the entry.
pub fn generate_single(spec: &ScSpecEntry) -> TokenStream {
    match spec {
        ScSpecEntry::FunctionV0(f) => generate_function(f),
        ScSpecEntry::UdtStructV0(s) => generate_struct(s),
        ScSpecEntry::UdtUnionV0(u) => generate_union(u),
        ScSpecEntry::UdtEnumV0(e) => generate_enum(e),
        ScSpecEntry::UdtErrorEnumV0(e) => generate_error_enum(e),
        ScSpecEntry::EventV0(e) => generate_event(e),
    }
}
