use ast::{Ast, ExprStmt, Expression, Factor, LetStmt, MulDiv, PlusMinus, Statement, Term};
use lexer::tokens::Literal;

pub fn compile_python(ast: &Ast) -> String {
    let mut out = String::new();
    for stmt in &ast.stmts {
        out += &compile_stmt(stmt);
    }
    out
}

fn compile_stmt(stmt: &Statement) -> String {
    match stmt {
        Statement::Let(let_stmt) => compile_let(let_stmt),
        Statement::Expr(expr_stmt) => compile_expr_stmt(expr_stmt),
    }
}

fn compile_let(stmt: &LetStmt) -> String {
    format!("{} = {}\n", stmt.ident.ident, compile_expr(&stmt.expr))
}

fn compile_expr_stmt(stmt: &ExprStmt) -> String {
    format!("print({})\n", compile_expr(&stmt.expr))
}

fn compile_expr(expr: &Expression) -> String {
    expr.rest
        .iter()
        .fold(compile_term(&expr.first), |code, (op, term)| {
            code + match op {
                PlusMinus::Plus {} => "+",
                PlusMinus::Minus {} => "-",
            } + &compile_term(term)
        })
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
    use std::{ffi::OsStr, process::Command};

    use lexer::lexer::Lexer;
    use parser::Parse;

    use super::*;

    #[test]
    fn test_python_compiler() {
        let ast =
            Ast::parse(&mut Lexer::from_string("let a = 1; 1/2*3+4*5+6-7;").peekable()).unwrap();
        println!("{:#?}", ast);
        let python_code = compile_python(&ast);
        println!("'{}'", python_code);
        let expected_stdout = (1f64 / 2f64 * 3f64 + 4f64 * 5f64 + 6f64 - 7f64).to_string();
        let mut command = Command::new("python");
        command.arg("-c").arg(python_code);
        println!(
            "{:?}",
            command.get_args().collect::<Vec<_>>().join(OsStr::new(" "))
        );
        let output = command.output().unwrap();
        if !output.status.success() {
            panic!("Python failed");
        }
        println!("'{}'", String::from_utf8_lossy(&output.stderr));
        let expected_stdout = expected_stdout.trim().replace("\r\n", "\n");
        let output = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("\r\n", "\n");
        assert_eq!(expected_stdout, output);
    }
}
