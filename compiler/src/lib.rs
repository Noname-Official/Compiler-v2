#[cfg(feature = "clap")]
use clap::ValueEnum;

use ast::Ast;
use python::compile_python;

mod python;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum Language {
    Python,
}

impl Language {
    #[must_use]
    pub const fn get_extension(&self) -> &'static str {
        match self {
            Language::Python => "py",
        }
    }
}

#[must_use]
pub fn compile(ast: &Ast, language: Language) -> String {
    match language {
        Language::Python => compile_python(ast),
    }
}
