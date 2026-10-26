/// Defines an enum with a [TokenKind] implementation and a [Lexer] constructor.
///
/// Automatically adds `Unrecognized` and `EndOfFile` variants to the enum, implements the
/// [TokenKind] trait, and generates a `lexer()` method that builds a [Lexer] from the rules.
///
/// # Example
/// ```
/// use lex::lexer::matchers::{digits, ident, whitespace};
/// use lex::{keyword, lexer, line_comment, literal};
///
/// lexer! {
///     #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
///     pub enum Kind {
///         LineComment : line_comment!("//"),
///         Whitespace : whitespace,
///         Import : keyword!("import"),
///         Ident : ident,
///         Integer : digits,
///         LBrace : literal!("{"),
///     }
/// }
///
/// let lexer: lex::lexer::Lexer<Kind> = Kind::lexer();
/// let tokens: Vec<lex::lexer::Token<Kind>> = lexer.lex("import foo {");
/// assert_eq!(tokens[0].kind(), Kind::Import);
/// assert_eq!(tokens[2].kind(), Kind::Ident);
/// assert_eq!(tokens[4].kind(), Kind::LBrace);
/// assert_eq!(tokens[5].kind(), Kind::EndOfFile);
/// ```
#[macro_export]
macro_rules! lexer {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident : $matcher:expr),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant,)*
            Unrecognized,
            EndOfFile,
        }

        impl $crate::lexer::TokenKind for $name {
            fn unrecognized() -> Self {
                $name::Unrecognized
            }

            fn end_of_file() -> Self {
                $name::EndOfFile
            }
        }

        impl $name {
            //! Lexer

            /// Creates a [Lexer] with rules in the order they were declared.
            #[must_use]
            pub fn lexer() -> $crate::lexer::Lexer<$name> {
                $crate::lexer::Lexer::default()
                    $(.with_rule($name::$variant, $matcher))*
            }
        }
    };
}
