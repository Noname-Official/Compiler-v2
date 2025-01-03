use ast::{Expression, Factor, MulDiv, PlusMinus, Term};
use lexer::tokens::Literal;

pub fn compile_python(ast: &Expression) -> String {
    format!(
        "print({})",
        ast.rest
            .iter()
            .fold(compile_term(&ast.first), |code, (op, term)| code
                + match op {
                    PlusMinus::Plus {} => "+",
                    PlusMinus::Minus {} => "-",
                }
                + &compile_term(term))
    )
}

fn compile_term(term: &Term) -> String {
    term.rest
        .iter()
        .fold(compile_factor(&term.first), |acc, (op, factor)| {
            acc + match op {
                MulDiv::Mul {} => "*",
                MulDiv::Div {} => "/",
            } + &compile_factor(factor)
        })
}

fn compile_factor(factor: &Factor) -> String {
    match factor.0 {
        Literal::Int(int) => int.to_string(),
        Literal::Float(float) => float.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use lexer::lexer::Lexer;
    use parser::Parse;

    use super::*;

    #[test]
    fn test_python_compiler() {
        let python_code = compile_python(
            &Expression::parse(&mut Lexer::from_string("1/2*3+4*5+6-7").peekable()).unwrap(),
        );
        let expected_stdout = (1f64 / 2f64 * 3f64 + 4f64 * 5f64 + 6f64 - 7f64).to_string();
        let output = Command::new("python")
            .arg("-c")
            .arg(python_code)
            .output()
            .unwrap();
        if !output.status.success() {
            panic!("Python failed");
        }
        let expected_stdout = expected_stdout.trim().replace("\r\n", "\n");
        let output = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("\r\n", "\n");
        assert_eq!(expected_stdout, output);
    }
}
