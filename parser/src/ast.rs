use derive::Parse;
use lexer::tokens::{Literal, Punct, Token};

use crate::token_ast;

token_ast! {Token,
    struct Literal { lit: Token::Literal(lit) }
    struct Punct { punct: Token::Punct(punct) }
    #[derive(Debug, PartialEq)]
    pub enum PlusMinus { Plus(): Token::Punct(Punct::Plus(_)), Minus(): Token::Punct(Punct::Minus(_)) }
    #[derive(Debug, PartialEq)]
    pub enum MulDiv { Mul(): Token::Punct(Punct::Star(_)), Div(): Token::Punct(Punct::Slash(_)) }
}

#[derive(Parse, Debug, PartialEq)]
pub struct Expression {
    pub first: Term,
    pub rest: Vec<(PlusMinus, Term)>,
}

#[derive(Parse, Debug, PartialEq)]
pub struct Term {
    pub first: Factor,
    pub rest: Vec<(MulDiv, Factor)>,
}

type Factor = Literal;
