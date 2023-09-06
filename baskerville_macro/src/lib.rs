use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data};

#[proc_macro_derive(Validator)]
pub fn validator_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_validator_macro(&ast)
}

fn impl_validator_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data: &Data = &ast.data;
    let generics = &ast.generics;
    match data {
        Data::Enum(data_enum) => {
            let match_arms = data_enum.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                quote! {
                    #name::#variant_ident(x) => x.validate(value),
                }
            });

            quote! {
            impl #generics Validator for #name #generics {
                fn validate(&mut self, value: &str) -> bool {
                    match self {
                        #(#match_arms)*
                    }
                }
            }}
            .into()
        }
        _ => panic!("Can only derive for Enum"),
    }
}

