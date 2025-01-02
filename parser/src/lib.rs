#[cfg(test)]
mod tests;

#[cfg(feature = "derive")]
pub use derive::Parse;

use std::iter::Peekable;

pub trait Parse: Sized {
    type Token;

    fn parse<Lexer: Iterator<Item = Self::Token>>(lexer: &mut Peekable<Lexer>) -> Option<Self>;
    fn maybe<Lexer: Iterator<Item = Self::Token>>(lexer: &mut Peekable<Lexer>) -> bool;
}

#[macro_export]
macro_rules! token_ast {
    ($Token: ty,) => {};
    ($Token: ty,
        $(#[$attr: meta])*
        $vis: vis struct $name: ident = $token: path { $ident: ident : $pat: pat }
        $($rest: tt)*
    ) => {
        $(#[$attr])*
        $vis struct $name($vis $token);

        impl ::std::ops::Deref for $name {
            type Target = $token;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $crate::Parse for $name {
            type Token = $Token;

            fn parse<Lexer: ::std::iter::Iterator<Item = Self::Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> ::std::option::Option<Self> {
                match lexer.next() {
                    Some($pat) => Some($name($ident)),
                    _ => None,
                }
            }

            fn maybe<Lexer: ::std::iter::Iterator<Item = Self::Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> bool {
                match lexer.peek() {
                    #[allow(unused)]
                    Some($pat) => true,
                    _ => false,
                }
            }
        }
        token_ast!{$Token, $($rest)*}
    };
    ($Token: ty,
        $(#[$attr: meta])*
        $vis: vis enum $name: ident { $($sub_name: ident ($($binding: ident: $Ty: ty),*): $pat: pat),+ }
        $($rest: tt)*
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $($sub_name {
                $($binding: $Ty,)*
            },)+
        }

        impl $crate::Parse for $name {
            type Token = $Token;

            fn parse<Lexer: ::std::iter::Iterator<Item = Self::Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> ::std::option::Option<Self> {
                match lexer.next() {
                    $(
                        Some($pat) => Some(Self::$sub_name {
                            $($binding,)*
                        }),
                    )+
                    _ => None
                }
            }

            fn maybe<Lexer: ::std::iter::Iterator<Item = Self::Token>>(
                lexer: &mut ::std::iter::Peekable<Lexer>,
            ) -> bool {
                match lexer.peek() {
                    $(
                        #[allow(unused)]
                        Some($pat) => true,
                    )+
                    _ => false,
                }
            }
        }
        token_ast!{$Token, $($rest)*}
    };
}

impl<T: Parse> Parse for Option<T> {
    type Token = T::Token;

    fn parse<Lexer: Iterator<Item = Self::Token>>(lexer: &mut Peekable<Lexer>) -> Option<Self> {
        if T::maybe(lexer) {
            Some(Some(T::parse(lexer)?))
        } else {
            Some(None)
        }
    }

    fn maybe<Lexer: Iterator<Item = Self::Token>>(_lexer: &mut Peekable<Lexer>) -> bool {
        true
    }
}

impl<T: Parse> Parse for Vec<T> {
    type Token = T::Token;

    fn parse<Lexer: Iterator<Item = Self::Token>>(lexer: &mut Peekable<Lexer>) -> Option<Self> {
        let mut res = vec![];
        while T::maybe(lexer) {
            res.push(T::parse(lexer)?);
        }
        Some(res)
    }

    fn maybe<Lexer: Iterator<Item = Self::Token>>(_lexer: &mut Peekable<Lexer>) -> bool {
        true
    }
}

macro_rules! parse_tuple {
    ($(
        ($first: ident $(,$rest: ident)* $(,)?)
    ),* $(,)?) => {$(
        impl<$first: Parse, $($rest: Parse<Token = $first::Token>),*> Parse for ($first, $($rest),*) {
            type Token = $first::Token;

            fn parse<Lexer: Iterator<Item = Self::Token>>(lexer: &mut Peekable<Lexer>) -> Option<Self> {
                Some(($first::parse(lexer)?, $($rest::parse(lexer)?),*))
            }

            fn maybe<Lexer: Iterator<Item = Self::Token>>(lexer: &mut Peekable<Lexer>) -> bool {
                $first::maybe(lexer)
            }
        }
    )*};
}

parse_tuple!(
    (T0),
    (T0, T1),
    (T0, T1, T2),
    (T0, T1, T2, T3),
    (T0, T1, T2, T3, T4),
    (T0, T1, T2, T3, T4, T5),
    (T0, T1, T2, T3, T4, T5, T6),
    (T0, T1, T2, T3, T4, T5, T6, T7),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
);
