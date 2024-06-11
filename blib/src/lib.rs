extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(ToStringMacro)]
pub fn to_string_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = match ast.data {
        Data::Struct(data_struct) => {
            let fields_str = match data_struct.fields {
                Fields::Named(fields) => {
                    fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote! { format!("{}: {:?}", stringify!(#name), &self.#name) }
                    }).collect::<Vec<_>>()
                },
                _ => Vec::new(), // Simplification: only handle named fields
            };
            quote! {
                impl #name {
                    pub fn to_string(&self) -> String {
                        format!("{} {{ {} }}", stringify!(#name), [#(#fields_str),*].join(", "))
                    }
                }
            }
        },
        _ => quote! {}, // Simplification: only handle structs
    };
    gen.into()
}