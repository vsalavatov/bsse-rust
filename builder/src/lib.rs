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
    let mut builder_setters: Vec<TokenStream2> = vec![];
    let mut field_checkers: Vec<TokenStream2> = vec![];
    let mut build_pass_vars: Vec<TokenStream2> = vec![];

    if let syn::Data::Struct(data) = input.data {
        for field in data.fields {
            let ident = field.ident.unwrap();
            let builder_field = Ident::new(&format!("{}_value", &ident), Span::call_site());
            let ty = field.ty;
            builder_fields.push(quote! {
                #builder_field: std::option::Option<#ty>,
            });
            builder_setters.push(quote! {
                pub fn #ident(&mut self, value: #ty) -> &mut Self {
                    self.#builder_field = Some(value);
                    self
                }
            });
            field_checkers.push(quote! {
                if let None = self.#builder_field {
                    return Err(String::from("#ident was not set").into());
                }
            });
            build_pass_vars.push(quote! {
                #ident: self.#builder_field.take().unwrap(),
            })
        }

        let expanded = quote! {
            #[derive(Default)]
            pub struct #builder_name {
                #(#builder_fields)*
            }

            impl #builder_name {
                pub fn build(&mut self) -> Result<#struct_name, Box<dyn std::error::Error>> {
                    #(#field_checkers)*
                    Ok(#struct_name {
                        #(#build_pass_vars)*
                    })
                }

                #(#builder_setters)*
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
