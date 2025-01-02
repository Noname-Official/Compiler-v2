#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(Literal),
    Punct(Punct),
}

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
}

#[derive(Debug, PartialEq)]
pub struct Plus;
#[derive(Debug, PartialEq)]
pub struct Minus;
#[derive(Debug, PartialEq)]
pub struct Star;
#[derive(Debug, PartialEq)]
pub struct Slash;
