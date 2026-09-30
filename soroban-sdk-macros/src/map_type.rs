use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use stellar_xdr::{ScSymbol, StringM, SC_SPEC_TYPE_NAME_LIMIT};
use syn::{ext::IdentExt as _, Error, Generics, Ident, Path, Type, TypeReference};

/// The names of the soroban_sdk types that a user-defined type cannot take,
/// so that a reference to a user-defined type can never be mistaken for one of
/// them by a reader of the contract.
const RESERVED_NAMES: &[&str] = &[
    "Val",
    "bool",
    "u32",
    "i32",
    "u64",
    "i64",
    "u128",
    "i128",
    "U256",
    "I256",
    "Timepoint",
    "Duration",
    "Bytes",
    "BytesN",
    "Hash",
    "String",
    "Symbol",
    "Address",
    "MuxedAddress",
    "Vec",
    "Map",
    "Option",
    "Result",
    "Fp",
    "Fp2",
    "Fr",
    "G1Affine",
    "G2Affine",
    "Bls12381Fp",
    "Bls12381Fp2",
    "Bls12381Fr",
    "Bls12381G1Affine",
    "Bls12381G2Affine",
    "Bn254Fp",
    "Bn254Fr",
    "Bn254G1Affine",
    "Bn254G2Affine",
    "BnScalar",
];

/// Checks that an `ident` and `generics` can be a user-defined type.
///
/// ### Errors
/// - If `ident` is the name of a soroban_sdk type
/// - If `ident` is longer than the spec's type name limit
/// - If `generics` has any parameters, as UDTs don't support generics
pub fn check_udt_ident(ident: &Ident, generics: &Generics) -> Result<(), Error> {
    let name = ident.unraw().to_string();
    if RESERVED_NAMES.contains(&name.as_str()) {
        return Err(Error::new(
            ident.span(),
            format!("type `{ident}` conflicts with a soroban_sdk type and cannot be used as a user-defined type"),
        ));
    }
    if let Err(e) = StringM::<SC_SPEC_TYPE_NAME_LIMIT>::try_from(name.as_str()) {
        return Err(Error::new(
            ident.span(),
            format!("type `{ident}` cannot be used in XDR spec: {e}"),
        ));
    }
    if !generics.params.is_empty() {
        return Err(Error::new(
            ident.span(),
            format!(
                "type `{}` contains generics `{}`, which are not supported for user-defined types",
                ident,
                generics.params.to_token_stream()
            ),
        ));
    }
    Ok(())
}

/// Renders the spec type of a Rust type as a const expression of type
/// `#path::xdr::r#const::ScSpecTypeDef`, so the containing spec entry can be
/// encoded to XDR at compile time by the contract crate.
///
/// The spec type comes from the type's `SpecTypeDef` impl, which the compiler
/// resolves, so that a type alias has the spec type of the type it aliases and
/// a user-defined type reports itself by its fully qualified name, which only
/// its own expansion knows because only it sees the module it is defined in.
/// A reference has the spec type of the type it refers to.
pub fn const_view_type_def(path: &Path, rust: &Type) -> TokenStream2 {
    // Outer references are removed rather than left to the reference impl,
    // because their lifetimes may be declared by a function signature, and the
    // spec entry is not rendered within the function's scope.
    let ty = unref(rust);
    quote!(<#ty as #path::SpecTypeDef>::SPEC_TYPE_DEF)
}

/// The Rust type behind any number of references.
fn unref(t: &Type) -> &Type {
    match t {
        Type::Reference(TypeReference { elem, .. }) => unref(elem),
        t => t,
    }
}

/// Emits the `SpecName` and `SpecTypeDef` impls for a user-defined type: the
/// name the contract spec knows it by, which is its Rust path — the module it
/// is defined in, then its own name — and the spec type that refers to it by
/// that name.
///
/// The module path is only known where the type is defined, and a macro cannot
/// see it, so `module_path!` is emitted for the compiler to expand in place
/// rather than resolved here. It is rooted with a leading `::` so the name is
/// an absolute crate path, distinguishing crates that would otherwise collide
/// with a same-named module imported into each context.
///
/// The generics arguments carry the type's `split_for_impl` pieces so an event
/// struct that borrows its fields can repeat its generics on the impl; a type
/// without generics passes `None` for each.
///
/// The name must fit the spec's type name limit, and its length is only known
/// once the compiler has expanded `module_path!`, so it is checked at compile
/// time. The check is part of evaluating the name, so an over-long name fails
/// with a single error naming the type, rather than with the XDR length error
/// that encoding the name would hit in every spec entry that uses it.
pub fn spec_type_def_gen(
    path: &Path,
    ident: &Ident,
    gen_impl: Option<TokenStream2>,
    gen_types: Option<TokenStream2>,
    gen_where: Option<TokenStream2>,
) -> TokenStream2 {
    let name = Literal::string(&ident.unraw().to_string());
    let too_long = Literal::string(&format!(
        "type `{}` full name including its module path is longer than the contract spec's type name limit, shorten its module path or name: `::",
        ident.unraw(),
    ));
    let gen_impl = gen_impl.unwrap_or_default();
    let gen_types = gen_types.unwrap_or_default();
    let gen_where = gen_where.unwrap_or_default();
    // Spanned to the type's name so that the error points at the type. The name
    // is held in consts rather than a local, because a local's name would be
    // resolved in the type's module and so could clash with a const there.
    let checked_name = quote_spanned! {ident.span()=>
        const NAME: &str = ::core::concat!("::", ::core::module_path!(), "::", #name);
        const CHECKED_NAME: &str = {
            ::core::assert!(
                NAME.len() <= #path::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize,
                ::core::concat!(#too_long, ::core::module_path!(), "::", #name, "`")
            );
            NAME
        };
    };
    quote! {
        impl #gen_impl #path::SpecName for #ident #gen_types #gen_where {
            const SPEC_NAME: &'static str = {
                #checked_name
                CHECKED_NAME
            };
        }
        impl #gen_impl #path::SpecTypeDef for #ident #gen_types #gen_where {
            const SPEC_TYPE_DEF: #path::xdr::r#const::ScSpecTypeDef =
                #path::xdr::r#const::ScSpecTypeDef::Udt(#path::xdr::r#const::ScSpecTypeUdt {
                    name: #path::xdr::r#const::StringM::try_from_str_or_panic(
                        <Self as #path::SpecName>::SPEC_NAME,
                    ),
                });
        }
    }
}

/// Renders a [StringM] as a const expression of type `#path::xdr::r#const::StringM`.
/// The `MAX` of the `const::StringM` is inferred from the field it is assigned to.
pub fn const_view_string<const MAX: u32>(path: &Path, s: &StringM<MAX>) -> TokenStream2 {
    let xdr = quote!(#path::xdr);
    let lit = Literal::byte_string(s.as_vec());
    quote!(#xdr::r#const::StringM::try_from_slice_or_panic(#lit))
}

/// Renders a [ScSymbol] as a const expression of type
/// `#path::xdr::r#const::ScSymbol`.
pub fn const_view_symbol(path: &Path, s: &ScSymbol) -> TokenStream2 {
    let xdr = quote!(#path::xdr);
    let s = const_view_string(path, &s.0);
    quote!(#xdr::r#const::ScSymbol(#s))
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::Span;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn test_check_udt_ident_sdk_type_errors() {
        let input: DeriveInput = parse_quote!(
            struct Address {
                pub key: [u8; 32],
            }
        );
        let err = check_udt_ident(&input.ident, &input.generics).unwrap_err();
        assert_eq!(
            err.to_string(),
            "type `Address` conflicts with a soroban_sdk type and cannot be used as a user-defined type"
        );
    }

    #[test]
    fn test_check_udt_ident_unique_generic_type_errors() {
        let input: DeriveInput = parse_quote!(
            struct GenericType<A, B> {
                pub key: T,
            }
        );
        let err = check_udt_ident(&input.ident, &input.generics).unwrap_err();
        assert_eq!(err.to_string(), "type `GenericType` contains generics `A , B`, which are not supported for user-defined types");
    }

    #[test]
    fn test_check_udt_ident_sdk_generic_type_errors() {
        let input: DeriveInput = parse_quote!(
            struct BytesN<T> {
                pub key: T,
            }
        );
        let err = check_udt_ident(&input.ident, &input.generics).unwrap_err();
        assert_eq!(
            err.to_string(),
            "type `BytesN` conflicts with a soroban_sdk type and cannot be used as a user-defined type"
        );
    }

    #[test]
    fn test_check_udt_ident_raw_ident_is_unrawed() {
        let ident = Ident::new_raw("Vec", Span::call_site());
        let err = check_udt_ident(&ident, &Generics::default()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "type `r#Vec` conflicts with a soroban_sdk type and cannot be used as a user-defined type"
        );
    }

    #[test]
    fn test_check_udt_ident_error_is_allowed() {
        // A contract's own error enum is commonly named Error, and is referred
        // to by its qualified name, so it does not conflict.
        let ident = Ident::new("Error", Span::call_site());
        assert!(check_udt_ident(&ident, &Generics::default()).is_ok());
    }

    #[test]
    fn test_check_udt_ident_unique_xdr_error() {
        // A name longer than the XDR spec's type name limit.
        let name = "A".repeat(SC_SPEC_TYPE_NAME_LIMIT as usize + 1);
        let ident = Ident::new(&name, Span::call_site());
        let err = check_udt_ident(&ident, &Generics::default()).unwrap_err();
        assert_eq!(
            err.to_string(),
            format!("type `{name}` cannot be used in XDR spec: xdr value max length exceeded")
        );
    }

    #[test]
    fn test_check_udt_ident_unique_ok() {
        let input: DeriveInput = parse_quote!(
            struct MyType {
                pub key: [u8; 32],
            }
        );
        assert!(check_udt_ident(&input.ident, &input.generics).is_ok());
    }
}

#[cfg(test)]
mod test_const_view {
    use super::*;
    use syn::parse_quote;

    fn path() -> Path {
        parse_quote!(soroban_sdk)
    }

    fn assert_tokens(actual: TokenStream2, expected: TokenStream2) {
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_type_def_comes_from_the_rust_type() {
        let ty: Type = parse_quote!(Option<u32>);
        assert_tokens(
            const_view_type_def(&path(), &ty),
            quote!(<Option<u32> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF),
        );
    }

    #[test]
    fn test_type_def_reaches_through_references() {
        let ty: Type = parse_quote!(&&mymod::MyType);
        assert_tokens(
            const_view_type_def(&path(), &ty),
            quote!(<mymod::MyType as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF),
        );
    }

    #[test]
    fn test_type_def_path_is_used_verbatim() {
        let ty: Type = parse_quote!(u32);
        let p: Path = parse_quote!(crate);
        assert_tokens(
            const_view_type_def(&p, &ty),
            quote!(<u32 as crate::SpecTypeDef>::SPEC_TYPE_DEF),
        );
    }

    #[test]
    fn test_string_empty() {
        assert_tokens(
            const_view_string(&path(), &StringM::<60>::try_from("").unwrap()),
            quote!(soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b""
            )),
        );
    }

    #[test]
    fn test_string_ascii() {
        assert_tokens(
            const_view_string(&path(), &StringM::<60>::try_from("hello").unwrap()),
            quote!(soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"hello"
            )),
        );
    }

    #[test]
    fn test_string_max_is_not_rendered() {
        let narrow =
            const_view_string(&path(), &StringM::<4>::try_from("abcd").unwrap()).to_string();
        let wide =
            const_view_string(&path(), &StringM::<1024>::try_from("abcd").unwrap()).to_string();
        assert_eq!(narrow, wide);
        assert_eq!(
            narrow,
            quote!(soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"abcd"
            ))
            .to_string()
        );
    }

    #[test]
    fn test_symbol() {
        assert_tokens(
            const_view_symbol(&path(), &ScSymbol("transfer".try_into().unwrap())),
            quote!(soroban_sdk::xdr::r#const::ScSymbol(
                soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"transfer")
            )),
        );
    }

    #[test]
    fn test_symbol_empty() {
        assert_tokens(
            const_view_symbol(&path(), &ScSymbol("".try_into().unwrap())),
            quote!(soroban_sdk::xdr::r#const::ScSymbol(
                soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"")
            )),
        );
    }

    #[test]
    fn test_symbol_wraps_the_string_rendering() {
        let p = path();
        let sym = ScSymbol("abc".try_into().unwrap());
        let inner = const_view_string(&p, &sym.0);
        assert_tokens(
            const_view_symbol(&p, &sym),
            quote!(soroban_sdk::xdr::r#const::ScSymbol(#inner)),
        );
    }

    #[test]
    fn test_symbol_path_is_used_verbatim() {
        let p: Path = parse_quote!(crate);
        assert_tokens(
            const_view_symbol(&p, &ScSymbol("s".try_into().unwrap())),
            quote!(crate::xdr::r#const::ScSymbol(
                crate::xdr::r#const::StringM::try_from_slice_or_panic(b"s")
            )),
        );
    }
}
