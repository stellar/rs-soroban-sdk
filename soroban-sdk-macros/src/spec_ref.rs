//! Renders a contract spec entry as a const `ScSpecEntryRef` expression.
//!
//! The derives build owned `ScSpecEntry` values as before, but instead of
//! encoding them to XDR here and emitting the bytes as a literal, they emit the
//! equivalent borrowing `ScSpecEntryRef` value. Contract crates then encode it
//! with `const_xdr_len`/`const_to_xdr`, so the XDR is written at compile time
//! from a value that is visible in the generated code.

use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use stellar_xdr::{
    ScSpecEntry, ScSpecEventV0, ScSpecFunctionV0, ScSpecTypeDef, ScSpecUdtEnumV0,
    ScSpecUdtErrorEnumV0, ScSpecUdtStructV0, ScSpecUdtUnionCaseV0, ScSpecUdtUnionV0, ScSymbol,
    StringM,
};
use syn::Path;

/// Renders `entry` as a const expression of type `#path::xdr::ScSpecEntryRef`.
pub fn spec_entry(path: &Path, entry: &ScSpecEntry) -> TokenStream2 {
    let xdr = &quote!(#path::xdr);
    let variant = format_ident!("{}", entry.name());
    let value = match entry {
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc,
            name,
            inputs,
            outputs,
        }) => {
            let (doc, name) = (string(xdr, doc), symbol(xdr, name));
            let inputs = inputs.iter().map(|i| {
                let (doc, name, type_) = (
                    string(xdr, &i.doc),
                    string(xdr, &i.name),
                    type_def(xdr, &i.type_),
                );
                quote!(#xdr::ScSpecFunctionInputV0Ref { doc: #doc, name: #name, type_: #type_ })
            });
            let outputs = outputs.iter().map(|o| type_def(xdr, o));
            quote!(#xdr::ScSpecFunctionV0Ref {
                doc: #doc,
                name: #name,
                inputs: #xdr::VecMRef::new(&[#(#inputs),*]),
                outputs: #xdr::VecMRef::new(&[#(#outputs),*]),
            })
        }
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc,
            lib,
            name,
            fields,
        }) => {
            let (doc, lib, name) = (string(xdr, doc), string(xdr, lib), string(xdr, name));
            let fields = fields.iter().map(|f| {
                let (doc, name, type_) = (
                    string(xdr, &f.doc),
                    string(xdr, &f.name),
                    type_def(xdr, &f.type_),
                );
                quote!(#xdr::ScSpecUdtStructFieldV0Ref { doc: #doc, name: #name, type_: #type_ })
            });
            quote!(#xdr::ScSpecUdtStructV0Ref {
                doc: #doc,
                lib: #lib,
                name: #name,
                fields: #xdr::VecMRef::new(&[#(#fields),*]),
            })
        }
        ScSpecEntry::UdtUnionV0(ScSpecUdtUnionV0 {
            doc,
            lib,
            name,
            cases,
        }) => {
            let (doc, lib, name) = (string(xdr, doc), string(xdr, lib), string(xdr, name));
            let cases = cases.iter().map(|c| match c {
                ScSpecUdtUnionCaseV0::VoidV0(c) => {
                    let (doc, name) = (string(xdr, &c.doc), string(xdr, &c.name));
                    quote!(#xdr::ScSpecUdtUnionCaseV0Ref::VoidV0(#xdr::ScSpecUdtUnionCaseVoidV0Ref { doc: #doc, name: #name }))
                }
                ScSpecUdtUnionCaseV0::TupleV0(c) => {
                    let (doc, name) = (string(xdr, &c.doc), string(xdr, &c.name));
                    let type_ = c.type_.iter().map(|t| type_def(xdr, t));
                    quote!(#xdr::ScSpecUdtUnionCaseV0Ref::TupleV0(#xdr::ScSpecUdtUnionCaseTupleV0Ref { doc: #doc, name: #name, type_: #xdr::VecMRef::new(&[#(#type_),*]) }))
                }
            });
            quote!(#xdr::ScSpecUdtUnionV0Ref {
                doc: #doc,
                lib: #lib,
                name: #name,
                cases: #xdr::VecMRef::new(&[#(#cases),*]),
            })
        }
        ScSpecEntry::UdtEnumV0(ScSpecUdtEnumV0 {
            doc,
            lib,
            name,
            cases,
        }) => {
            let (doc, lib, name) = (string(xdr, doc), string(xdr, lib), string(xdr, name));
            let cases = cases.iter().map(|c| {
                let (doc, name, value) = (string(xdr, &c.doc), string(xdr, &c.name), c.value);
                quote!(#xdr::ScSpecUdtEnumCaseV0Ref { doc: #doc, name: #name, value: #value })
            });
            quote!(#xdr::ScSpecUdtEnumV0Ref {
                doc: #doc,
                lib: #lib,
                name: #name,
                cases: #xdr::VecMRef::new(&[#(#cases),*]),
            })
        }
        ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
            doc,
            lib,
            name,
            cases,
        }) => {
            let (doc, lib, name) = (string(xdr, doc), string(xdr, lib), string(xdr, name));
            let cases = cases.iter().map(|c| {
                let (doc, name, value) = (string(xdr, &c.doc), string(xdr, &c.name), c.value);
                quote!(#xdr::ScSpecUdtErrorEnumCaseV0Ref { doc: #doc, name: #name, value: #value })
            });
            quote!(#xdr::ScSpecUdtErrorEnumV0Ref {
                doc: #doc,
                lib: #lib,
                name: #name,
                cases: #xdr::VecMRef::new(&[#(#cases),*]),
            })
        }
        ScSpecEntry::EventV0(ScSpecEventV0 {
            doc,
            lib,
            name,
            prefix_topics,
            params,
            data_format,
        }) => {
            let (doc, lib, name) = (string(xdr, doc), string(xdr, lib), symbol(xdr, name));
            let prefix_topics = prefix_topics.iter().map(|t| symbol(xdr, t));
            let params = params.iter().map(|p| {
                let (doc, name, type_) = (string(xdr, &p.doc), string(xdr, &p.name), type_def(xdr, &p.type_));
                let location = format_ident!("{}", p.location.name());
                quote!(#xdr::ScSpecEventParamV0Ref { doc: #doc, name: #name, type_: #type_, location: #xdr::ScSpecEventParamLocationV0::#location })
            });
            let data_format = format_ident!("{}", data_format.name());
            quote!(#xdr::ScSpecEventV0Ref {
                doc: #doc,
                lib: #lib,
                name: #name,
                prefix_topics: #xdr::VecMRef::new(&[#(#prefix_topics),*]),
                params: #xdr::VecMRef::new(&[#(#params),*]),
                data_format: #xdr::ScSpecEventDataFormat::#data_format,
            })
        }
    };
    quote!(#xdr::ScSpecEntryRef::#variant(#value))
}

/// Renders a `StringM` as a const `StringMRef` expression. The `MAX` of the
/// `StringMRef` is inferred from the field it is assigned to.
fn string<const MAX: u32>(xdr: &TokenStream2, s: &StringM<MAX>) -> TokenStream2 {
    let lit = Literal::byte_string(s.as_vec());
    quote!(#xdr::StringMRef::new(#lit))
}

/// Renders a `ScSymbol` as a const `ScSymbolRef` expression.
fn symbol(xdr: &TokenStream2, s: &ScSymbol) -> TokenStream2 {
    let s = string(xdr, &s.0);
    quote!(#xdr::ScSymbolRef(#s))
}

/// Renders a `ScSpecTypeDef` as a const `ScSpecTypeDefRef` expression.
fn type_def(xdr: &TokenStream2, t: &ScSpecTypeDef) -> TokenStream2 {
    let variant = format_ident!("{}", t.name());
    // Variants that hold a value. The recursive ones are behind a reference in
    // the Ref type, matching the Box in the owned type.
    let value = match t {
        ScSpecTypeDef::Option(o) => {
            let value_type = type_def(xdr, &o.value_type);
            Some(quote!((&#xdr::ScSpecTypeOptionRef { value_type: &#value_type })))
        }
        ScSpecTypeDef::Result(r) => {
            let (ok_type, error_type) = (type_def(xdr, &r.ok_type), type_def(xdr, &r.error_type));
            Some(
                quote!((&#xdr::ScSpecTypeResultRef { ok_type: &#ok_type, error_type: &#error_type })),
            )
        }
        ScSpecTypeDef::Vec(v) => {
            let element_type = type_def(xdr, &v.element_type);
            Some(quote!((&#xdr::ScSpecTypeVecRef { element_type: &#element_type })))
        }
        ScSpecTypeDef::Map(m) => {
            let (key_type, value_type) = (type_def(xdr, &m.key_type), type_def(xdr, &m.value_type));
            Some(
                quote!((&#xdr::ScSpecTypeMapRef { key_type: &#key_type, value_type: &#value_type })),
            )
        }
        ScSpecTypeDef::Tuple(t) => {
            let value_types = t.value_types.iter().map(|t| type_def(xdr, t));
            Some(
                quote!((&#xdr::ScSpecTypeTupleRef { value_types: #xdr::VecMRef::new(&[#(#value_types),*]) })),
            )
        }
        ScSpecTypeDef::BytesN(b) => {
            let n = b.n;
            Some(quote!((#xdr::ScSpecTypeBytesN { n: #n })))
        }
        ScSpecTypeDef::Udt(u) => {
            let name = string(xdr, &u.name);
            Some(quote!((#xdr::ScSpecTypeUdtRef { name: #name })))
        }
        // All remaining variants are void.
        _ => None,
    };
    quote!(#xdr::ScSpecTypeDefRef::#variant #value)
}
