#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(Ident),
    Keyword(Keyword),
    Literal(Literal),
    Punct(Punct),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ident {
    pub ident: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keyword {
    Let(Let),
    If(If),
    While(While),
    For(For),
    Fn(Fn),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Let;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct If;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct While;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct For;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fn;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(isize),
    Float(f64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Punct {
    Plus(Plus),
    Minus(Minus),
    Star(Star),
    Slash(Slash),
    Eq(Eq),
    SemiColon(SemiColon),
    LParen(LParen),
    RParen(RParen),
    LBrace(LBrace),
    RBrace(RBrace),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Plus;
#[derive(Debug, PartialEq, Eq)]
pub struct Minus;
#[derive(Debug, PartialEq, Eq)]
pub struct Star;
#[derive(Debug, PartialEq, Eq)]
pub struct Slash;
#[derive(Debug, PartialEq, Eq)]
pub struct Eq;
#[derive(Debug, PartialEq, Eq)]
pub struct SemiColon;
#[derive(Debug, PartialEq, Eq)]
pub struct LParen;
#[derive(Debug, PartialEq, Eq)]
pub struct RParen;
#[derive(Debug, PartialEq, Eq)]
pub struct LBrace;
#[derive(Debug, PartialEq, Eq)]
pub struct RBrace;
