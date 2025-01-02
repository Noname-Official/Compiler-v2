use std::{env, fs, process::exit};

use ast::Expression;
use lexer::lexer::Lexer;
use parser::Parse;

fn main() {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!(
                "usage: {} path",
                env::args().next().unwrap_or("compiler".into())
            );
            return;
        }
    };
    let file = match fs::OpenOptions::new().read(true).open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("error opening file '{path}': {e}");
            return;
        }
    };
    let mut lexer = Lexer::from_readable(file);
    let mut peekable = lexer.by_ref().peekable();
    let expr = Expression::parse(&mut peekable);
    peekable.for_each(|_| {});
    if lexer.error {
        exit(-1);
    }
    let Some(ast) = expr else {
        eprintln!("Error parsing");
        exit(-1);
    };
    println!("{:#?}", ast);
}
