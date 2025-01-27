use crate::Parse;

#[test]
fn test_parser() {
    #[derive(PartialEq, Debug)]
    struct Foo(i32);
    impl Parse for Foo {
        type Token = i32;

        fn parse<Lexer: Iterator<Item = Self::Token>>(
            lexer: &mut std::iter::Peekable<Lexer>,
        ) -> Option<Self> {
            Some(Self(lexer.next()?))
        }

        fn maybe<Lexer: Iterator<Item = Self::Token>>(
            lexer: &mut std::iter::Peekable<Lexer>,
        ) -> bool {
            lexer.peek().is_some()
        }
    }
    assert_eq!(
        Some(Foo(123)),
        Foo::parse(&mut vec![123].into_iter().peekable())
    );
}
