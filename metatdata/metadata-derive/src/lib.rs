use std::collections::HashMap;

use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataStructAttribute {
    author: String,
    #[deluxe(default = 0)]
    serial_version: usize,
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataFieldAttributes {
    author: String,
}

fn extract_meta_data_field_attributes(
    ast: &mut DeriveInput,
) -> deluxe::Result<HashMap<String, MetaDataFieldAttributes>> {
    let mut field_attrs = HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let attrs: MetaDataFieldAttributes = deluxe::extract_attributes(field)?;
            field_attrs.insert(field_name, attrs);
        }
    }
    Ok(field_attrs)
}

fn metadata_derive_macro2(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // parse

    let mut ast: DeriveInput = syn::parse2(item)?;

    let MetaDataStructAttribute {
        author,
        serial_version,
    } = deluxe::extract_attributes(&mut ast)?;

    let field_attrs: HashMap<String, MetaDataFieldAttributes> =
        extract_meta_data_field_attributes(&mut ast)?;

    let (field_name, field_authors): (Vec<String>, Vec<String>) = field_attrs
        .into_iter()
        .map(|(field, attrs)| (field, attrs.author))
        .unzip();
    // extract struct attribute
    let ident = &ast.ident;
    let (impl_generic, type_generic, where_clause) = ast.generics.split_for_impl();
    // generate
    Ok(quote::quote!(
        impl #impl_generic MetaData for #ident #type_generic #where_clause {
            fn author(&self) -> &str {
                #author
            }

            fn serial_version(&self) -> usize {
                #serial_version
            }

            fn field_authors(&self) -> std::collections::HashMap<&str, &str> {
                let fields = [#(#field_name),*];
                let authors = [#(#field_authors),*];

                let map: std::collections::HashMap<&str, &str> = fields.iter()
                .zip(authors.iter())
                .map(|(&field, &author)| (field, author))
                .collect();
                map
            }
        }
    ))
}

#[proc_macro_derive(MetaData, attributes(metadata))]
pub fn metadata_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    metadata_derive_macro2(item.into()).unwrap().into()
}
