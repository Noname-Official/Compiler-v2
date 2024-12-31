use std::{env, fs, process::exit};

use lexer::lexer::Lexer;

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
    for token in &mut lexer {
        println!("{token:#?}");
    }
    if lexer.error {
        exit(-1);
    }
}
