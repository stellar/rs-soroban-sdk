use syn::{Attribute, Data, DeriveInput, Fields};

/// Returns true if the attribute is an attribute that should be preserved and
/// passed through to code generated for the item the attribute is on.
pub fn pass_through_attr_to_gen_code(attr: &Attribute) -> bool {
    attr.path().is_ident("doc")
        || attr.path().is_ident("cfg")
        || attr.path().is_ident("allow")
        || attr.path().is_ident("deny")
}

pub fn remove_attributes_from_item(input: &mut DeriveInput) {
    if let Data::Struct(ref mut data_struct) = input.data {
        match &mut data_struct.fields {
            Fields::Named(ref mut fields_named) => {
                for field in fields_named.named.iter_mut() {
                    // Retain only attributes that are NOT 'topic' or 'data'.
                    field.attrs.retain(|attr| {
                        !attr.path().is_ident("topic") && !attr.path().is_ident("data")
                    });
                }
            }
            Fields::Unnamed(ref mut fields_unnamed) => {
                for field in fields_unnamed.unnamed.iter_mut() {
                    // Retain only attributes that are NOT 'topic' or 'data'.
                    field.attrs.retain(|attr| {
                        !attr.path().is_ident("topic") && !attr.path().is_ident("data")
                    });
                }
            }
            Fields::Unit => {
                // Unit structs have no fields, nothing to do.
            }
        }
    } else {
        unimplemented!("Only structs are supported by remove_attributes_from_item");
    }
}
