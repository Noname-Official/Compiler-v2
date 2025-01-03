use std::{env, fs, path::Path, process::exit};

use ast::Ast;
use compiler::{compile, Language};
use lexer::lexer::Lexer;
use parser::Parse;

fn main() {
    let mut args = env::args().skip(1);
    let path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!(
                "usage: {} path [output_path]",
                env::args().next().unwrap_or("compiler".into())
            );
            return;
        }
    };
    let output_path = match args.next() {
        Some(path) => path,
        None => Path::new(&path)
            .with_extension("py")
            .to_string_lossy()
            .into_owned(),
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
    let ast = Ast::parse(&mut peekable);
    peekable.for_each(|_| {});
    if lexer.error {
        exit(-1);
    }
    let Some(ast) = ast else {
        eprintln!("Error parsing");
        exit(-1);
    };
    let asm = compile(&ast, Language::Python);
    fs::write(output_path, asm).unwrap();
}
