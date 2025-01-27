use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DataEnum, DataStruct, DeriveInput, Fields,
    Ident, Token,
};

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    match input.data {
        Data::Struct(data_struct) => derive_parse_struct(&name, &data_struct),
        Data::Enum(data_enum) => derive_parse_enum(&name, &data_enum),
        _ => panic!("expected struct or enum"),
    }
}

fn gen_parse(name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => {
            // Unwrap is safe here, because Named fields will always have an ident
            let field_names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());
            quote! {
                #name {
                    #(#field_names: ::parser::Parse::parse(lexer)?,)*
                }
            }
        }
        Fields::Unnamed(fields) => {
            let parse = fields
                .unnamed
                .iter()
                .map(|_| quote! {::parser::Parse::parse(lexer)?,});
            quote! {
                #name(#(#parse)*)
            }
        }
        Fields::Unit => quote! {#name},
    }
}

fn gen_maybe(fields: &Fields) -> proc_macro2::TokenStream {
    let first_field_ty = &fields
        .iter()
        .next()
        .map_or_else(|| quote! {true}, |field| field.ty.to_token_stream());
    quote! {<#first_field_ty as ::parser::Parse>::maybe(lexer)}
}

fn derive_parse_struct(name: &Ident, input: &DataStruct) -> TokenStream {
    // TODO: unit structs
    let first_field_ty = input.fields.iter().next().map(|field| &field.ty).unwrap();
    let constructor = gen_parse(name, &input.fields);
    let maybe = gen_maybe(&input.fields);
    quote!(
        impl ::parser::Parse for #name {
            type Token = <#first_field_ty as ::parser::Parse>::Token;

            fn parse<Lexer: ::std::iter::Iterator<Item = Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> Option<Self> {
                Some(#constructor)
            }

            fn maybe<Lexer: ::std::iter::Iterator<Item = Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> bool {
                #maybe
            }
        }
    )
    .into()
}

fn derive_parse_enum(name: &Ident, input: &DataEnum) -> TokenStream {
    // TODO: enum with only unit variants
    let first_field_ty = input
        .variants
        .iter()
        .find_map(|variant| variant.fields.iter().next().map(|field| &field.ty))
        .unwrap();
    let parse = input.variants.iter().map(|variant| {
        let maybe = gen_maybe(&variant.fields);
        let constructor = gen_parse(&variant.ident, &variant.fields);
        quote! {
            if #maybe {
                return Some(Self::#constructor);
            }
        }
    });
    let maybe: Punctuated<_, Token![||]> = input
        .variants
        .iter()
        .map(|variant| gen_maybe(&variant.fields))
        .collect();
    quote! {
        impl ::parser::Parse for #name {
            type Token = <#first_field_ty as ::parser::Parse>::Token;

            fn parse<Lexer: ::std::iter::Iterator<Item = Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> Option<Self> {
                #(#parse)*
                None
            }

            fn maybe<Lexer: ::std::iter::Iterator<Item = Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> bool {
                #maybe
            }
        }
    }
    .into()
}
