use ast::{
    Ast, ExprStmt, Expression, Factor, IfStmt, LetStmt, Literal, MulDiv, PlusMinus, Statement,
    Term, WhileStmt,
};
use lexer::tokens;

pub fn compile_python(ast: &Ast) -> String {
    let mut out = String::new();
    for stmt in &ast.stmts {
        out += &compile_stmt(stmt, 0);
    }
    out
}

fn compile_stmt(stmt: &Statement, indent: usize) -> String {
    match stmt {
        Statement::Let(let_stmt) => compile_let(let_stmt, indent),
        Statement::Expr(expr_stmt) => compile_expr_stmt(expr_stmt, indent),
        Statement::If(if_stmt) => compile_if(if_stmt, indent),
        Statement::While(while_stmt) => compile_while(while_stmt, indent),
    }
}

fn compile_let(stmt: &LetStmt, indent: usize) -> String {
    format!(
        "{}{} = {}\n",
        "\t".repeat(indent),
        stmt.ident.ident,
        compile_expr(&stmt.expr)
    )
}

fn compile_if(stmt: &IfStmt, indent: usize) -> String {
    format!(
        "{}if {}:\n{}",
        "\t".repeat(indent),
        compile_expr(&stmt.expr),
        stmt.stmts
            .iter()
            .map(|stmt| compile_stmt(stmt, indent + 1))
            .collect::<String>()
    )
}

fn compile_while(stmt: &WhileStmt, indent: usize) -> String {
    format!(
        "{}while {}:\n{}",
        "\t".repeat(indent),
        compile_expr(&stmt.expr),
        stmt.stmts
            .iter()
            .map(|stmt| compile_stmt(stmt, indent + 1))
            .collect::<String>()
    )
}

fn compile_expr_stmt(stmt: &ExprStmt, indent: usize) -> String {
    format!(
        "{}print({})\n",
        "\t".repeat(indent),
        compile_expr(&stmt.expr)
    )
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
