use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let builder_name = Ident::new(&format!("{}Builder", struct_name), Span::call_site());

    let mut builder_fields: Vec<TokenStream2> = vec![];

    if let syn::Data::Struct(data) = input.data {
        for field in data.fields {
            let builder_field = Ident::new(
                &format!("{}_value", field.ident.unwrap()),
                Span::call_site(),
            );
            let ty = field.ty;
            builder_fields.push(quote! {
                #builder_field: std::option::Option<#ty>,
            })
        }

        let expanded = quote! {
            #[derive(Default)]
            pub struct #builder_name {
                #(#builder_fields)*
            }

            impl #struct_name {
                pub fn builder() -> #builder_name {
                    #builder_name::default()
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        unimplemented!()
    }
}
