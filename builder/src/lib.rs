use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder_name = Ident::new(&format!("{}Builder", name), Span::call_site());
    let builder_fields = builder_fields(&input.data);
    let builder_fields_none = builder_fields_none(&input.data);
    let builder_fields_setters = builder_fields_setter(&input.data);
    let builder_fields_build_1 = builder_fields_build_1(&input.data);
    let builder_fields_build_2 = builder_fields_build_2(&input.data);
    let expanded = quote! {
        pub struct #builder_name {
            #builder_fields
        }
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #builder_fields_none
                }
            }

        }
        use std::error::Error;
        impl #builder_name {
            #builder_fields_setters
            pub fn build(&mut self) -> Result<#name, Box<dyn Error>> {
                #builder_fields_build_1
                Ok(#name {
                    #builder_fields_build_2
                })
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn builder_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {
                        f.span() =>
                        #name: Option<#ty>
                    }
                });
                quote! {
                    #(#recurse,)*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
fn builder_fields_none(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {
                        f.span() =>
                        #name: None
                    }
                });
                quote! {
                    #(#recurse,)*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
fn builder_fields_setter(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {
                        f.span() =>
                        fn #name(&mut self, #name: #ty) -> &mut Self {
                            self.#name = Some(#name);
                            self
                        }
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
fn builder_fields_build_1(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {
                        f.span() =>
                        let #name = self.#name.clone().ok_or(String::from("#name not set"))?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
fn builder_fields_build_2(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {
                        f.span() =>
                        #name: #name,
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
