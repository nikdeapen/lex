use crate::{Context, Error};

/// The result of parsing an element `T` with error type `E`.
///
/// `Ok(parsed_element, after_parsed_element)`
/// `Err(Error { error_token, typed_error })`
pub type ParseResult<'a, T, E> = Result<(T, Context<'a>), Error<'a, E>>;
