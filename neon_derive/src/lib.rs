use syn::{DeriveInput, Fields};

#[proc_macro_derive(Deserialize)]
pub fn impl_deserializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let implementation = match input.data {
        syn::Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let implementations = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;

                    quote::quote!(#field_name: reader.read::<#field_type>()?,)
                });

                quote::quote! {
                    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
                        Some(#name {
                            #(#implementations)*
                        })
                    }
                }
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    proc_macro::TokenStream::from(quote::quote! {
        impl neon_io::serializable::Deserializable for #name {
            type Output<'a> = #name;

            #implementation
        }

        impl neon_io::serializable::Deserializable for &'_ #name {
            type Output<'a> = #name;

            #implementation
        }
    })
}

#[proc_macro_derive(Serialize)]
pub fn impl_serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let implementation = match input.data {
        syn::Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let implementations = fields.named.iter().map(|f| {
                    let field_name = &f.ident;

                    quote::quote!(writer.write(&self.#field_name);)
                });

                quote::quote! {
                    fn serialize(&self, writer: &mut Writer) {
                        #(#implementations)*
                    }
                }
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    proc_macro::TokenStream::from(quote::quote! {
        impl neon_io::serializable::Serializable for #name {
            #implementation
        }
    })
}
