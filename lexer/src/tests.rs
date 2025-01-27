use std::io::{self, Read};

use super::{lexer::*, tokens::*};

#[test]
fn test_lexer_from_readable() {
    pub struct Readable;
    impl Read for Readable {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Ok(0)
        }
    }
    Lexer::from_readable(Readable);
}

#[test]
fn test_lexer_from_char_iter() {
    Lexer::from_char_iter(['a']);
}

#[test]
fn test_lexer_ints() {
    assert_eq!(
        vec![
            Token::Literal(Literal::Int(0)),
            Token::Literal(Literal::Int(1)),
            Token::Literal(Literal::Int(5)),
            Token::Literal(Literal::Int(1234)),
        ],
        Lexer::from_string("0 1 5 1234").collect::<Vec<_>>(),
    );
}

#[test]
fn test_lexer_floats() {
    assert_eq!(
        vec![
            Token::Literal(Literal::Float(1.2)),
            Token::Literal(Literal::Float(0.1)),
            Token::Literal(Literal::Float(1.)),
            Token::Literal(Literal::Float(1234.5678)),
        ],
        Lexer::from_string("1.2 .1 1. 1234.5678").collect::<Vec<_>>(),
    );
}

#[test]
fn test_lexer_punctuation() {
    assert_eq!(
        vec![
            Token::Punct(Punct::Plus(Plus)),
            Token::Punct(Punct::Minus(Minus)),
            Token::Punct(Punct::Star(Star)),
            Token::Punct(Punct::Slash(Slash)),
            Token::Punct(Punct::Eq(Eq)),
            Token::Punct(Punct::SemiColon(SemiColon)),
        ],
        Lexer::from_string("+-*/=;").collect::<Vec<_>>(),
    );
}

#[test]
fn test_lexer_ident() {
    assert_eq!(
        vec![
            Token::Ident(Ident {
                ident: String::from("abcd")
            }),
            Token::Ident(Ident {
                ident: String::from("efgh")
            }),
        ],
        Lexer::from_string("abcd efgh").collect::<Vec<_>>(),
    );
}
