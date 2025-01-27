use std::collections::HashMap;

use ast::Expression;
use lexer::lexer::Lexer;
use parser::Parse;

use crate::interpret_expr;

#[test]
fn test_interpreter() {
    assert!(
        ((1f64 / 2f64 * 3f64 + 4f64 * 5f64 + 6f64 - 7f64)
            - interpret_expr(
                &Expression::parse(&mut Lexer::from_string("1/2*3+4*5+6-7").peekable()).unwrap(),
                &mut HashMap::new(),
            ))
        .abs()
            < 0.1,
    );
}
