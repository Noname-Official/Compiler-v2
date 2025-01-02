use crate::parser::Parse;

#[cfg(feature = "ast")]
#[test]
fn test_ast() {
    use crate::ast::{Expression, MulDiv, PlusMinus, Term};
    use lexer::{lexer::Lexer, tokens::Literal};
    assert_eq!(
        Some(Expression {
            first: Term {
                first: Literal::Int(1),
                rest: vec![
                    (MulDiv::Div {}, Literal::Int(2)),
                    (MulDiv::Mul {}, Literal::Int(3)),
                ],
            },
            rest: vec![
                (
                    PlusMinus::Plus {},
                    Term {
                        first: Literal::Int(4),
                        rest: vec![(MulDiv::Mul {}, Literal::Int(5))],
                    },
                ),
                (
                    PlusMinus::Plus {},
                    Term {
                        first: Literal::Int(6),
                        rest: vec![],
                    },
                ),
                (
                    PlusMinus::Minus {},
                    Term {
                        first: Literal::Int(7),
                        rest: vec![],
                    },
                ),
            ],
        }),
        Expression::parse(&mut Lexer::from_string("1/2*3+4*5+6-7").peekable()),
    );
}

#[test]
fn test_parser() {
    #[derive(PartialEq, Debug)]
    struct Foo(i32);
    impl Parse for Foo {
        type Token = i32;

        fn parse<Lexer: Iterator<Item = Self::Token>>(
            lexer: &mut std::iter::Peekable<Lexer>,
        ) -> Option<Self> {
            Some(Foo(lexer.next()?))
        }

        fn maybe<Lexer: Iterator<Item = Self::Token>>(
            lexer: &mut std::iter::Peekable<Lexer>,
        ) -> bool {
            lexer.peek().is_some()
        }
    }
    assert_eq!(
        Some(Foo(123)),
        Foo::parse(&mut vec![123].into_iter().peekable())
    );
}
