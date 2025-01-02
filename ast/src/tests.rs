use lexer::lexer::Lexer;

use crate::*;

#[test]
fn test_ast() {
    assert_eq!(
        Some(Expression {
            first: Term {
                first: Literal(tokens::Literal::Int(1)),
                rest: vec![
                    (MulDiv::Div {}, Literal(tokens::Literal::Int(2))),
                    (MulDiv::Mul {}, Literal(tokens::Literal::Int(3))),
                ],
            },
            rest: vec![
                (
                    PlusMinus::Plus {},
                    Term {
                        first: Literal(tokens::Literal::Int(4)),
                        rest: vec![(MulDiv::Mul {}, Literal(tokens::Literal::Int(5)))],
                    },
                ),
                (
                    PlusMinus::Plus {},
                    Term {
                        first: Literal(tokens::Literal::Int(6)),
                        rest: vec![],
                    },
                ),
                (
                    PlusMinus::Minus {},
                    Term {
                        first: Literal(tokens::Literal::Int(7)),
                        rest: vec![],
                    },
                ),
            ],
        }),
        Expression::parse(&mut Lexer::from_string("1/2*3+4*5+6-7").peekable()),
    );
}
