use ast::{Expression, Factor, Term};
use lexer::tokens::Literal;

#[cfg(test)]
mod tests;

pub fn interpret(expr: &Expression) -> f64 {
    expr.rest
        .iter()
        .fold(interpret_term(&expr.first), |acc, (op, term)| match op {
            ast::PlusMinus::Plus {} => acc + interpret_term(term),
            ast::PlusMinus::Minus {} => acc - interpret_term(term),
        })
}

fn interpret_term(term: &Term) -> f64 {
    term.rest.iter().fold(
        interpret_factor(&term.first),
        |acc, (op, factor)| match op {
            ast::MulDiv::Mul {} => acc * interpret_factor(factor),
            ast::MulDiv::Div {} => acc / interpret_factor(factor),
        },
    )
}

fn interpret_factor(factor: &Factor) -> f64 {
    match factor.0 {
        Literal::Int(int) => int as f64,
        Literal::Float(float) => float,
    }
}
