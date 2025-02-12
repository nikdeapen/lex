use crate::{Error, ParseContext};

/// The result of parsing an element `T` with error type `E`.
///
/// `Ok(parsed_element, after_parsed_element)`
/// `Err(Error { error_token, typed_error })`
pub type LexResult<'a, T, E> = Result<(T, ParseContext<'a>), Error<'a, E>>;
