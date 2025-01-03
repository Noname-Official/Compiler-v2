#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(Ident),
    Keyword(Keyword),
    Literal(Literal),
    Punct(Punct),
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    pub ident: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Let(Let),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Let;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(isize),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum Punct {
    Plus(Plus),
    Minus(Minus),
    Star(Star),
    Slash(Slash),
    Eq(Eq),
    SemiColon(SemiColon),
}

#[derive(Debug, PartialEq)]
pub struct Plus;
#[derive(Debug, PartialEq)]
pub struct Minus;
#[derive(Debug, PartialEq)]
pub struct Star;
#[derive(Debug, PartialEq)]
pub struct Slash;
#[derive(Debug, PartialEq)]
pub struct Eq;
#[derive(Debug, PartialEq)]
pub struct SemiColon;
