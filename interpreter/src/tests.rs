use ast::Expression;
use lexer::lexer::Lexer;
use parser::Parse;

use crate::interpret;

#[test]
fn test_interpreter() {
    assert_eq!(
        1f64 / 2f64 * 3f64 + 4f64 * 5f64 + 6f64 - 7f64,
        interpret(&Expression::parse(&mut Lexer::from_string("1/2*3+4*5+6-7").peekable()).unwrap()),
    )
}
