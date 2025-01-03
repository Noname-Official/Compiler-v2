#[cfg(test)]
mod tests;

use lexer::tokens;
#[cfg(feature = "parser")]
use lexer::tokens::Token;
#[cfg(feature = "parser")]
use parser::{token_ast, Parse};

#[cfg(not(feature = "parser"))]
macro_rules! token_ast {
    ($Token: ty,) => {};
    ($Token: ty,
        $(#[$attr: meta])*
        $vis: vis struct $name: ident = $token: path { $ident: ident : $pat: pat }
        $($rest: tt)*
    ) => {
        $(#[$attr])*
        $vis struct $name($vis $token);

        impl ::std::ops::Deref for $name {
            type Target = $token;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        token_ast!{$Token, $($rest)*}
    };
    ($Token: ty,
        $(#[$attr: meta])*
        $vis: vis enum $name: ident { $($sub_name: ident ($($binding: ident: $Ty: ty),*): $pat: pat),+ }
        $($rest: tt)*
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $($sub_name {
                $($binding: $Ty,)*
            },)+
        }
        token_ast!{$Token, $($rest)*}
    };
}

token_ast! {Token,
    #[derive(Debug, PartialEq)]
    pub struct Literal = tokens::Literal { lit: Token::Literal(lit) }
    #[derive(Debug, PartialEq)]
    pub struct Punct = tokens::Punct { punct: Token::Punct(punct) }
    #[derive(Debug, PartialEq)]
    pub enum PlusMinus { Plus(): Token::Punct(tokens::Punct::Plus(_)), Minus(): Token::Punct(tokens::Punct::Minus(_)) }
    #[derive(Debug, PartialEq)]
    pub enum MulDiv { Mul(): Token::Punct(tokens::Punct::Star(_)), Div(): Token::Punct(tokens::Punct::Slash(_)) }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "parser", derive(Parse))]
pub struct Expression {
    pub first: Term,
    pub rest: Vec<(PlusMinus, Term)>,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "parser", derive(Parse))]
pub struct Term {
    pub first: Factor,
    pub rest: Vec<(MulDiv, Factor)>,
}

pub type Factor = Literal;
