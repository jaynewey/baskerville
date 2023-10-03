use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{self, Data, DataEnum, Ident};

#[proc_macro_derive(Validator)]
pub fn validator_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_validator_macro(&ast)
}

fn impl_validator_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data: &Data = &ast.data;
    let generics = &ast.generics;

    fn match_arms<'a>(
        name: &'a Ident,
        data_enum: &'a DataEnum,
        method: Ident,
    ) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a
    {
        data_enum.variants.iter().map(move |variant| {
            let variant_ident = &variant.ident;
            quote! {
                #name::#variant_ident(x) => x.#method(value),
            }
        })
    }

    match data {
        Data::Enum(data_enum) => {
            let validates = match_arms(name, data_enum, Ident::new("validate", Span::call_site()));
            let considers = match_arms(name, data_enum, Ident::new("consider", Span::call_site()));

            quote! {
            impl #generics Validator for #name #generics {
                fn validate(&self, value: &str) -> Result<(), ValidationError> {
                    match self {
                        #(#validates)*
                    }
                }

                fn consider(&mut self, value: &str) -> Result<(), ValidationError> {
                    match self {
                        #(#considers)*
                    }
                }
            }}
            .into()
        }
        _ => panic!("Can only derive for Enum"),
    }
}
