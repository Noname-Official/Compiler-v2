use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    io::ErrorKind,
    iter::Iterator,
    process::Command,
};

use ast::Ast;
use compiler::{compile, Language};
use lexer::lexer::Lexer;
use parser::Parse;

// FIXME(NoName): use this function instead of `fs::read_dir` bcs windows file system invalidated lexer folder
// fn read_dir_recursive(path: impl AsRef<Path>) -> Vec<DirEntry> {
//     let mut paths = vec![];
//     for file in fs::read_dir(path).unwrap() {
//         let file = file.unwrap();
//         let file_type = file.file_type().unwrap();
//         if file_type.is_dir() {
//             paths.extend(read_dir_recursive(file.path()));
//         } else if file_type.is_file() {
//             paths.push(file);
//         }
//     }
//     paths
// }

#[test]
fn test() {
    for file in fs::read_dir("tests").unwrap() {
        fn read_expected(file: &DirEntry, extension: impl AsRef<OsStr>) -> String {
            let extension = extension.as_ref();
            let mut new_extension = file.path().extension().unwrap_or_default().to_owned();
            new_extension.push(extension);
            match fs::read_to_string(file.path().with_extension(new_extension)) {
                Ok(output) => output,
                Err(e) if e.kind() == ErrorKind::NotFound => String::new(),
                Err(e) => panic!("Error reading {} file: {e}", extension.to_string_lossy()),
            }
            .trim()
            .replace("\r\n", "\n")
        }

        let file = file.unwrap();
        if file.path().extension().and_then(OsStr::to_str) != Some("txt") {
            continue;
        }
        let expected_stdout = read_expected(&file, ".stdout");
        let expected_stderr = read_expected(&file, ".stderr");
        if expected_stderr.is_empty() {
            let ast = Ast::parse(
                &mut Lexer::from_readable(
                    fs::OpenOptions::new().read(true).open(file.path()).unwrap(),
                )
                .peekable(),
            )
            .unwrap();
            #[allow(clippy::single_element_loop, reason = "future proof")]
            for (lang, command) in [(Language::Python, |code| {
                Command::new("python").arg("-c").arg(code).output().unwrap()
            })] {
                let output = command(compile(&ast, lang));
                assert_eq!(
                    String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .replace("\r\n", "\n"),
                    expected_stdout,
                );
                assert_eq!(
                    String::from_utf8_lossy(&output.stderr)
                        .trim()
                        .replace("\r\n", "\n"),
                    "",
                );
                assert!(output.status.success());
            }
        }
        let output = Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .arg("--")
            .arg("interpret")
            .arg(file.path())
            .output()
            .unwrap();
        assert_eq!(
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .replace("\r\n", "\n"),
            expected_stdout
        );
        assert_eq!(
            String::from_utf8_lossy(&output.stderr)
                .trim()
                .replace("\r\n", "\n"),
            expected_stderr
        );
    }
}
