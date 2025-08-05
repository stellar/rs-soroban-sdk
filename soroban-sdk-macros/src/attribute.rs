use syn::{punctuated::Punctuated, Attribute, Data, Fields, FieldsNamed, FieldsUnnamed};

/// Returns true if the attribute is an attribute that should be preserved and
/// passed through to code generated for the item the attribute is on.
pub fn pass_through_attr_to_gen_code(attr: &Attribute) -> bool {
    attr.path().is_ident("doc")
        || attr.path().is_ident("cfg")
        || attr.path().is_ident("allow")
        || attr.path().is_ident("deny")
}

/// Modifies the input, removing any attributes on struct fields that match the attrs name list.
///
/// Currently implemented only for struct data.
pub fn remove_attributes_from_item(data: &mut Data, attrs: &[&str]) {
    let fields = match data {
        Data::Struct(data) => match &mut data.fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
            Fields::Unit => &mut Punctuated::default(), // Unit structs have no fields, nothing to do.
        },
        _ => unimplemented!("Only structs are supported by remove_attributes_from_item"),
    };
    for field in fields {
        field.attrs.retain(|attr| {
            !attr
                .path()
                .get_ident()
                .is_some_and(|ident| attrs.contains(&ident.to_string().as_str()))
        });
    }
}

#[cfg(test)]
mod test {
    use quote::{quote, ToTokens};
    use syn::DeriveInput;

    use super::remove_attributes_from_item;

    #[test]
    fn test_remove_attributes_from_item_struct_named() {
        let input = quote! {
            struct Struct {
                f1: u32,
                #[topic]
                f2: u32,
                f3: u32,
                #[data]
                f4: u32,
            }
        };
        let expect = quote! {
            struct Struct {
                f1: u32,
                f2: u32,
                f3: u32,
                f4: u32,
            }
        };
        let mut input = syn::parse2::<DeriveInput>(input.into()).unwrap();
        remove_attributes_from_item(&mut input.data, &["topic", "data"]);
        assert_eq!(input.to_token_stream().to_string(), expect.to_string());
    }

    #[test]
    fn test_remove_attributes_from_item_struct_unnamed() {
        let input = quote! {
            struct Struct(#[topic] u32, u32, #[data] u64);
        };
        let expect = quote! {
            struct Struct(u32, u32, u64);
        };
        let mut input = syn::parse2::<DeriveInput>(input.into()).unwrap();
        remove_attributes_from_item(&mut input.data, &["topic", "data"]);
        assert_eq!(input.to_token_stream().to_string(), expect.to_string());
    }

    #[test]
    fn test_remove_attributes_from_item_struct_unit() {
        let input = quote! {
            struct Struct;
        };
        let expect = quote! {
            struct Struct;
        };
        let mut input = syn::parse2::<DeriveInput>(input.into()).unwrap();
        remove_attributes_from_item(&mut input.data, &["topic", "data"]);
        assert_eq!(input.to_token_stream().to_string(), expect.to_string());
    }
}
