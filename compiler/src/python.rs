use ast::{
    Ast, ExprStmt, Expression, Factor, LetStmt, Literal, MulDiv, PlusMinus, Statement, Term,
};
use lexer::tokens;

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
    match factor {
        Factor::Ident(ident) => ident.ident.clone(),
        Factor::Literal(Literal(tokens::Literal::Int(int))) => int.to_string(),
        Factor::Literal(Literal(tokens::Literal::Float(float))) => float.to_string(),
    }
}
