use std::str::FromStr;

use num_bigint::BigUint;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::LitInt;

extern crate proc_macro;

#[proc_macro]
pub fn array_from_lit_int(input: TokenStream) -> TokenStream {
    array_from_lit_int2(input.into()).into()
}

fn array_from_lit_int2(input: TokenStream2) -> TokenStream2 {
    let lit = match syn::parse2::<LitInt>(input) {
        Ok(lit) => lit,
        Err(e) => return e.to_compile_error(),
    };
    let int = BigUint::from_str(lit.base10_digits()).unwrap();
    let bytes = int.to_bytes_be();
    quote! { [#(#bytes),*] }
}

#[cfg(test)]
mod test {
    use crate::array_from_lit_int2;
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::ExprArray;

    #[test]
    fn hex() {
        let tokens = array_from_lit_int2(quote! {0x1});
        let parsed = syn::parse2::<ExprArray>(tokens).unwrap();
        let expect = syn::parse_quote!([1u8]);
        assert_eq!(parsed, expect);

        let tokens = array_from_lit_int2(quote! {0x928374892abc});
        let parsed = syn::parse2::<ExprArray>(tokens).unwrap();
        let expect = syn::parse_quote!([146u8, 131u8, 116u8, 137u8, 42u8, 188u8]);
        assert_eq!(parsed, expect);
    }

    #[test]
    fn base10() {
        let tokens =
            array_from_lit_int2(quote! {340_282_366_920_938_463_463_374_607_431_768_211_455u128});
        let parsed = syn::parse2::<ExprArray>(tokens).unwrap();
        let expect = syn::parse_quote!([
            255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
            255u8, 255u8, 255u8, 255u8
        ]);
        assert_eq!(parsed, expect);

        let tokens =
            array_from_lit_int2(quote! {340_282_366_920_938_463_463_374_607_431_768_211_456});
        let parsed = syn::parse2::<ExprArray>(tokens).unwrap();
        let expect = syn::parse_quote!([
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8
        ]);
        assert_eq!(parsed, expect);
    }
}
