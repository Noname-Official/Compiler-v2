use std::{
    env,
    ffi::OsStr,
    fs,
    io::{self, Read},
    process::Command,
};

use super::{lexer::*, tokens::*};

#[test]
fn test_lexer_from_readable() {
    pub struct Readable;
    impl Read for Readable {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Ok(0)
        }
    }
    Lexer::from_readable(Readable);
}

#[test]
fn test_lexer_from_char_iter() {
    Lexer::from_char_iter(['a']);
}

#[test]
fn test_lexer_ints() {
    assert_eq!(
        vec![
            Token::Literal(Literal::Int(0)),
            Token::Literal(Literal::Int(1)),
            Token::Literal(Literal::Int(5)),
            Token::Literal(Literal::Int(1234)),
        ],
        Lexer::from_string("0 1 5 1234").collect::<Vec<_>>(),
    )
}

#[test]
fn test_lexer_floats() {
    assert_eq!(
        vec![
            Token::Literal(Literal::Float(1.2)),
            Token::Literal(Literal::Float(0.1)),
            Token::Literal(Literal::Float(1.)),
            Token::Literal(Literal::Float(1234.5678)),
        ],
        Lexer::from_string("1.2 .1 1. 1234.5678").collect::<Vec<_>>(),
    )
}

#[test]
fn test_lexer_punctuation() {
    assert_eq!(
        vec![
            Token::Punct(Punct::Plus(Plus)),
            Token::Punct(Punct::Minus(Minus)),
            Token::Punct(Punct::Star(Star)),
            Token::Punct(Punct::Slash(Slash)),
            Token::Punct(Punct::Eq(Eq)),
            Token::Punct(Punct::SemiColon(SemiColon)),
        ],
        Lexer::from_string("+-*/=;").collect::<Vec<_>>(),
    )
}

#[test]
fn test_lexer_ident() {
    assert_eq!(
        vec![
            Token::Ident(Ident {
                ident: String::from("abcd")
            }),
            Token::Ident(Ident {
                ident: String::from("efgh")
            }),
        ],
        Lexer::from_string("abcd efgh").collect::<Vec<_>>(),
    )
}

#[test]
fn test_lexer_errs() {
    env::set_current_dir("..").unwrap();
    let mut total_tests = 0;
    let mut successes = 0;
    for test in fs::read_dir("tests/lexer/errs").unwrap() {
        let test = test.unwrap();
        if test.path().extension().and_then(OsStr::to_str) != Some("txt") {
            continue;
        }
        print!(
            "test tests::lexer::errs::{} ... ",
            test.file_name().to_string_lossy()
        );
        total_tests += 1;
        let output = Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .arg("--")
            .arg(test.path().as_os_str())
            .output();
        let output = output.unwrap();
        if output.status.success() {
            println!("FAILED\nExpected test to fail, but it succeeded");
            continue;
        }
        let output = output.stderr;
        let output = String::from_utf8(output).unwrap();
        let path = test.path();
        let mut extension = path.extension().unwrap_or_default().to_owned();
        extension.push(".stderr");
        let expected = fs::read_to_string(path.with_extension(extension)).unwrap();
        let output = output.trim().replace("\r\n", "\n");
        let expected = expected.trim().replace("\r\n", "\n");
        if output != expected {
            println!("FAILED");
            println!("Expected:");
            println!("{expected}");
            println!("Actual:");
            println!("{output}");
            continue;
        }
        successes += 1;
        println!("ok");
    }
    println!("{successes}/{total_tests} tests succeeded");
    assert_eq!(successes, total_tests, "Not all test succeeded");
}
