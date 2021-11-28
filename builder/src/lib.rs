use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, GenericArgument, Lit, Meta, NestedMeta, PathArguments, Type,
};

#[proc_macro_derive(Builder, attributes(builder))]
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
            let ty = &field.ty;

            let mut optional_generic_type: Option<Type> = None;
            let mut each_name: Option<Lit> = None;
            let mut each_type: Option<Type> = None;

            // check if ty is optional
            if let Type::Path(typ) = ty {
                let is_optional = typ.path.segments[0].ident == "Option";
                let is_vec = typ.path.segments[0].ident == "Vec";
                if is_optional || is_vec {
                    if let PathArguments::AngleBracketed(ab) = &typ.path.segments[0].arguments {
                        if let GenericArgument::Type(ttt) = &ab.args[0] {
                            if is_optional {
                                optional_generic_type = Some(ttt.to_owned())
                            } else {
                                // is_vec
                                each_type = Some(ttt.to_owned())
                            }
                        } else {
                            unimplemented!();
                        }
                    } else {
                        unimplemented!();
                    }
                }
            }
            // check `each` mode
            for attr in field.attrs {
                if attr.path.segments[0].ident == "builder" {
                    let meta_list = if let Meta::List(ml) = attr.parse_meta().unwrap() {
                        ml
                    } else {
                        unimplemented!()
                    };
                    each_name = Some(if let NestedMeta::Meta(submeta) = &meta_list.nested[0] {
                        if let Meta::NameValue(nv) = submeta {
                            nv.lit.to_owned()
                        } else {
                            unimplemented!();
                        }
                    } else {
                        unimplemented!()
                    });
                } else {
                    unimplemented!();
                }
            }

            if optional_generic_type.is_some() {
                builder_fields.push(quote! {
                    #builder_field: #ty,
                });
                let tt = optional_generic_type.unwrap();
                builder_setters.push(quote! {
                    pub fn #ident(&mut self, value: #tt) -> &mut Self {
                        self.#builder_field = Some(value);
                        self
                    }
                });
                build_pass_vars.push(quote! {
                    #ident: self.#builder_field.take(),
                })
            } else if each_name.is_some() {
                builder_fields.push(quote! {
                    #builder_field: #ty,
                });
                let name = each_name.unwrap();
                if let Lit::Str(namestr) = name {
                    let tt = each_type.unwrap();
                    let name_ident = Ident::new(&namestr.value(), Span::call_site());
                    if namestr.value() != ident.to_string() {
                        builder_setters.push(quote! {
                            pub fn #ident(&mut self, value: #ty) -> &mut Self {
                                self.#builder_field = value;
                                self
                            }
                        });
                    }
                    builder_setters.push(quote! {
                        pub fn #name_ident(&mut self, value: #tt) -> &mut Self {
                            self.#builder_field.push(value);
                            self
                        }
                    });
                } else {
                    unimplemented!();
                }
                build_pass_vars.push(quote! {
                    #ident: std::mem::replace(&mut self.#builder_field, std::vec::Vec::new()),
                });
            } else {
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
                });
            }
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
