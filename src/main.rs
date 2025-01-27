use std::{fs, path::Path, process::exit};

#[cfg(test)]
mod tests;

use ast::Ast;
use clap::{Parser, Subcommand};
use compiler::{compile, Language};
use interpreter::interpret;
use lexer::lexer::Lexer;
use parser::Parse;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Compile {
        #[arg(short, long)]
        language: Language,
        input_file: String,
        output_file: Option<String>,
    },
    Interpret {
        input_file: String,
    },
}

impl Commands {
    fn input_file(&self) -> &str {
        match self {
            Commands::Compile { input_file, .. } | Commands::Interpret { input_file, .. } => {
                input_file
            }
        }
    }
}

fn main() {
    let args = Cli::parse();
    let file = match fs::OpenOptions::new()
        .read(true)
        .open(args.command.input_file())
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("error opening file '{}': {e}", args.command.input_file());
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
    match args.command {
        Commands::Compile {
            language,
            input_file,
            output_file,
        } => {
            let asm = compile(&ast, language);
            let output_path = output_file.unwrap_or_else(|| {
                Path::new(&input_file)
                    .with_extension(language.get_extension())
                    .to_string_lossy()
                    .into_owned()
            });
            fs::write(output_path, asm).unwrap();
        }
        Commands::Interpret { input_file: _ } => {
            interpret(&ast);
        }
    }
}
