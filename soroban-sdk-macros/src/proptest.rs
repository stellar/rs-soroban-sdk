use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Path};

/// Implement proptest's `Arbitrary` for the prototype, by forwarding to the `arbitrary`
/// implementation derived for it. Lets the prototype be named directly in the parameter list of a
/// `proptest!` test.
///
/// The tokens are emitted inside the scope that declares the prototype, rather than beside it,
/// because the prototype is declared in an anonymous const and is not nameable from outside it.
/// Emitting the implementation at module scope instead fails for a private `contracttype`, whose
/// prototype is private too, with a private type in a public interface.
///
/// Empty unless the proptest support is enabled, because the paths it refers to only exist when the
/// sdk's "testutils-proptest" feature is on, and that feature is what enables this one.
pub fn quote_proptest(path: &Path, arbitrary_type_ident: &Ident) -> TokenStream2 {
    if !cfg!(feature = "testutils-proptest") {
        return quote! {};
    }
    quote! {
        impl #path::testutils::proptest::proptest::arbitrary::Arbitrary for #arbitrary_type_ident {
            type Parameters = ();
            type Strategy = #path::testutils::proptest::proptest_arbitrary_interop::ArbStrategy<Self>;
            fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
                #path::testutils::proptest::arb_from_size_hint::<Self>()
            }
        }
    }
}
