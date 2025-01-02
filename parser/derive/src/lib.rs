use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields = match input.data {
        // TODO: tuple structs
        Data::Struct(data_struct) => data_struct.fields,
        // TODO: enums
        _ => panic!("expected struct"),
    };
    let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
    // TODO: Unit structs
    let first_field_ty = &fields.iter().next().unwrap().ty;
    quote!(
        impl crate::parser::Parse for #name {
            type Token = <#first_field_ty as crate::parser::Parse>::Token;

            fn parse<Lexer: ::std::iter::Iterator<Item = Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> Option<Self> {
                Some(Self {
                    #(#field_names: crate::parser::Parse::parse(lexer)?,)*
                })
            }

            fn maybe<Lexer: ::std::iter::Iterator<Item = Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> bool {
                #first_field_ty::maybe(lexer)
            }
        }
    )
    .into()
}
