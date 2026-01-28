use crate::{Config, Context, Error, Token};
use std::fmt::Debug;

impl<'a> Token<'a> {
    //! Error

    /// Converts the token to an error with the given `config`.
    ///
    /// If the `token` is empty, the `Error` token will be empty.
    /// If the `token` starts with a `symbol`, the `Error` token will be the `symbol`.
    /// Otherwise, the `Error` token will be the first char.
    pub fn to_error<E>(self, config: &'a Config, e: E) -> Error<'a, E>
    where
        E: Debug,
    {
        let token: Token = if let (Some(symbol), _after) = Context::new(self, config).symbol() {
            symbol
        } else if let Some(first) = self.value().chars().next() {
            let (first_char, _after) = unsafe { self.split(first.len_utf8()) };
            first_char
        } else {
            self
        };
        Error::new(token, e)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, Error, Token};

    #[test]
    fn to_error() {
        let test_cases: &[(&str, &str)] = &[
            ("", ""),
            ("symbol after-symbol", "symbol"),
            ("! not-symbol", "!"),
        ];
        for (input, expected) in test_cases {
            let config: Config = Config::default();
            let token: Token = Token::from(*input);
            let result: Error<&str> = token.to_error(&config, "error");

            let expected: Token = Token::from(*expected);
            let expected: Error<&str> = Error::new(expected, "error");
            assert_eq!(result, expected);
        }
    }
}
