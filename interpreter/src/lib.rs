use std::collections::HashMap;

use ast::{Ast, Expression, Factor, IfStmt, LetStmt, Literal, Statement, Term, WhileStmt};
use lexer::tokens;

#[cfg(test)]
mod tests;

pub fn interpret(ast: &Ast) {
    let mut vars = HashMap::new();
    for stmt in &ast.stmts {
        interpret_stmt(stmt, &mut vars);
    }
}

fn interpret_stmt(stmt: &Statement, vars: &mut HashMap<String, f64>) {
    match stmt {
        Statement::Let(let_stmt) => interpret_let(let_stmt, vars),
        Statement::Expr(expr_stmt) => println!("{}", interpret_expr(&expr_stmt.expr, vars)),
        Statement::If(if_stmt) => interpret_if(if_stmt, vars),
        Statement::While(while_stmt) => interpret_while(while_stmt, vars),
    }
}

fn interpret_let(stmt: &LetStmt, vars: &mut HashMap<String, f64>) {
    let val = interpret_expr(&stmt.expr, vars);
    vars.insert(stmt.ident.ident.clone(), val);
}

fn interpret_if(stmt: &IfStmt, vars: &mut HashMap<String, f64>) {
    let val = interpret_expr(&stmt.expr, vars);
    if val != 0. {
        for stmt in &stmt.stmts {
            interpret_stmt(stmt, vars);
        }
    }
}

fn interpret_while(stmt: &WhileStmt, vars: &mut HashMap<String, f64>) {
    while interpret_expr(&stmt.expr, vars) != 0. {
        for stmt in &stmt.stmts {
            interpret_stmt(stmt, vars);
        }
    }
}

fn interpret_expr(expr: &Expression, vars: &mut HashMap<String, f64>) -> f64 {
    expr.rest.iter().fold(
        interpret_term(&expr.first, vars),
        |acc, (op, term)| match op {
            ast::PlusMinus::Plus {} => acc + interpret_term(term, vars),
            ast::PlusMinus::Minus {} => acc - interpret_term(term, vars),
        },
    )
}

fn interpret_term(term: &Term, vars: &mut HashMap<String, f64>) -> f64 {
    term.rest.iter().fold(
        interpret_factor(&term.first, vars),
        |acc, (op, factor)| match op {
            ast::MulDiv::Mul {} => acc * interpret_factor(factor, vars),
            ast::MulDiv::Div {} => acc / interpret_factor(factor, vars),
        },
    )
}

fn interpret_factor(factor: &Factor, vars: &mut HashMap<String, f64>) -> f64 {
    match factor {
        Factor::Literal(Literal(tokens::Literal::Int(int))) => *int as f64,
        Factor::Literal(Literal(tokens::Literal::Float(float))) => *float,
        // TODO: error handling
        Factor::Ident(ident) => *vars.get(&ident.ident).unwrap(),
    }
}
