use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(Specified)]
pub fn derive_specified_fn(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;

    let field_descriptions = match &mut ast.data {
        syn::Data::Struct(ref struct_data) => match struct_data.fields {
            Fields::Named(ref fields) => fields
                .named
                .iter()
                .map(|field| {
                    let ident = field.ident.clone().unwrap().to_string();
                    let member_type = &field.ty.to_token_stream().to_string();

                    let token = quote! {
                        let member_string = format!("    {}: {}\n", #ident, #member_type);
                        spec_string.push_str(&member_string);
                    };

                    token
                })
                .collect::<Vec<TokenStream2>>(),
            _ => panic!("`Specified` is intended for structs with named fields"),
        },
        _ => panic!("`Specified` has to be used with structs"),
    };

    let field_descriptions = field_descriptions.iter();
    let ident_str = ident.to_string();

    let output = quote! {
        impl Specified for #ident {
            fn specified() -> std::string::String {
                let mut spec_string = format!("{} {{\n", #ident_str);

                #(#field_descriptions)*

                let end_brace = "}";
                spec_string.push_str(&end_brace);
                spec_string
            }
        }
    };

    output.into()
}
