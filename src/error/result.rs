use crate::{Error, ParseContext};

/// The result of parsing an element `T` with error type `E`.
///
/// `Ok(parsed_element, after_parsed_element)`
/// `Err(Error { error_token, typed_error })`
pub type Result<'a, T, E> = std::result::Result<(T, ParseContext<'a>), Error<'a, E>>;
