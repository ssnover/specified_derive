use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use syn::{parse_macro_input, DeriveInput, Fields};
use quote::quote;

#[proc_macro_derive(Specified)]
pub fn derive_specified_fn(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;

    let field_descriptions = match &mut ast.data {
        syn::Data::Struct(ref struct_data) => {
            match struct_data.fields {
                Fields::Named(ref fields) => { 
                    fields.named.iter().map(|field| {
                        let ident = field.ident.clone().unwrap();
                        let member_type = &field.ty;

                        let token = quote! { 
                            let member_string = format!("{}: {}", #ident, #member_type);
                            spec_string.push_str(&member_string);
                        };

                        token

                    }).collect::<Vec<TokenStream2>>()
                },
                _ => panic!("`Specified` is intended for structs with named fields"),
            }
        }
        _ => panic!("`Specified` has to be used with structs"),
    };

    let field_descriptions = field_descriptions.iter();

    let output = quote! {
        impl ::specified_derive::Specified for #ident {
            fn specified() -> std::string::String {
                let mut spec_string = format!("{} {\n", #ident);

                #(#field_descriptions)*

                let end_brace = "}";
                spec_string.push_str(&spec_string);
                spec_string
            }
        }
    };

    output.into()
}
