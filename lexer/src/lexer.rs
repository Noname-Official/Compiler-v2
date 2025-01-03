use std::{collections::HashMap, io::Read, iter::Peekable, str::Chars};

use crate::tokens::{
    Eq, Ident, Keyword, Let, Literal, Minus, Plus, Punct, SemiColon, Slash, Star, Token,
};

pub struct Lexer<T: Iterator<Item = char>> {
    source: Peekable<T>,
    pub error: bool,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn from_char_iter<T: IntoIterator<IntoIter = I>>(iter: T) -> Self {
        Self {
            source: iter.into_iter().peekable(),
            error: false,
        }
    }
}

impl<R: Read> Lexer<ReadIter<R>> {
    pub fn from_readable(readable: R) -> Self {
        Self::from_char_iter(ReadIter::new(readable))
    }
}

impl<'a> Lexer<Chars<'a>> {
    pub fn from_string(string: &'a str) -> Self {
        Self::from_char_iter(string.chars())
    }
}

pub struct ReadIter<T: Read> {
    inner: T,
    buf: [u8; 1024],
    index: usize,
    end: usize,
}

impl<T: Read> ReadIter<T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            buf: [0; 1024],
            index: 0,
            end: 0,
        }
    }
}

impl<T: Read> Iterator for ReadIter<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.end {
            match self.inner.read(&mut self.buf) {
                Ok(len) => self.end = len,
                Err(e) => {
                    eprintln!("Failed to read: {e}");
                    return None;
                }
            }
            self.index = 0;
            if self.end == 0 {
                return None;
            }
        }
        self.index += 1;
        Some(self.buf[self.index - 1] as char)
    }
}

impl<T: Iterator<Item = char>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut c = ' ';
            while c.is_whitespace() {
                c = self.source.next()?
            }
            return match c {
                '+' => Some(Token::Punct(Punct::Plus(Plus))),
                '-' => Some(Token::Punct(Punct::Minus(Minus))),
                '*' => Some(Token::Punct(Punct::Star(Star))),
                '/' => Some(Token::Punct(Punct::Slash(Slash))),
                ';' => Some(Token::Punct(Punct::SemiColon(SemiColon))),
                '=' => Some(Token::Punct(Punct::Eq(Eq))),
                'a'..='z' => {
                    let key_words = {
                        let mut test = HashMap::new();
                        test.insert("let", Keyword::Let(Let));
                        test
                    };

                    let mut ident = c.to_string();
                    while let Some(c) = self.source.next_if(char::is_ascii_lowercase) {
                        ident.push(c);
                    }
                    Some(match key_words.get(ident.as_str()) {
                        Some(kw) => Token::Keyword(*kw),
                        None => Token::Ident(Ident { ident }),
                    })
                }
                '0'..='9' | '.' => {
                    let mut value = c.to_string();
                    let mut float = c == '.';
                    let mut valid = true;
                    while let Some(c @ '0'..='9' | c @ '.') = self.source.peek() {
                        let c = *c;
                        self.source.next();
                        if c == '.' && float {
                            valid = false;
                        }
                        value.push(c);
                        float |= c == '.';
                    }
                    Some(Token::Literal(if !valid {
                        eprintln!("Invalid float literal '{value}'");
                        self.error = true;
                        Literal::Int(0)
                    } else if float {
                        // TODO: notify user of bug in compiler on Err
                        Literal::Float(value.parse().unwrap())
                    } else {
                        // TODO: notify user of bug in compiler on Err
                        Literal::Int(value.parse().unwrap())
                    }))
                }
                _ => {
                    eprintln!("Unrecognized token: '{c}'");
                    self.error = true;
                    continue;
                }
            };
        }
    }
}
