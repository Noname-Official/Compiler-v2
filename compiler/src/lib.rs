use ast::Ast;
use python::compile_python;

mod python;

pub enum Language {
    Python,
}

pub fn compile(ast: &Ast, language: Language) -> String {
    match language {
        Language::Python => compile_python(ast),
    }
}
