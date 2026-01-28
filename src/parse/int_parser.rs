use crate::parse::IntParseError::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Responsible for parsing integers.
#[derive(Copy, Clone, Debug)]
pub struct IntParser {
    allow_underscores: bool,
    allow_starting_underscore: bool,
    allow_ending_underscore: bool,
    allow_double_underscore: bool,
}

impl Default for IntParser {
    fn default() -> Self {
        Self {
            allow_underscores: true,
            allow_starting_underscore: false,
            allow_ending_underscore: false,
            allow_double_underscore: false,
        }
    }
}

/// An error parsing an integer.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum IntParseError {
    Empty,
    InvalidChar(char),
    ContainsUnderscore,
    StartsWithUnderscore,
    EndsWithUnderscore,
    ContainsDoubleUnderscore,
    ContainsOnlyUnderscores,
    ValueOutOfRange,
}

impl IntParseError {
    //! Message

    /// Gets the error message.
    pub fn message(&self) -> &'static str {
        match self {
            Empty => "integers cannot be empty",
            InvalidChar(_) => "the integer contains an invalid char",
            ContainsUnderscore => "integers cannot contain underscores",
            StartsWithUnderscore => "integers cannot start with underscores",
            EndsWithUnderscore => "integers cannot end with underscores",
            ContainsDoubleUnderscore => "integers cannot contain double underscores",
            ContainsOnlyUnderscores => "integers cannot contain only underscores",
            ValueOutOfRange => "the integer value is out of range",
        }
    }
}

impl Display for IntParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for IntParseError {}

impl IntParser {
    //! Validation

    /// Validates the string and strips the starting & ending underscores.
    pub fn validate<'a>(&self, s: &'a str) -> Result<&'a str, IntParseError> {
        if s.is_empty() {
            Err(Empty)
        } else if let Some(c) = s.chars().find(|c| !c.is_ascii_digit() && *c != '_') {
            Err(InvalidChar(c))
        } else if !self.allow_underscores && s.as_bytes().contains(&b'_') {
            Err(ContainsUnderscore)
        } else if !self.allow_starting_underscore && s.as_bytes()[0] == b'_' {
            Err(StartsWithUnderscore)
        } else if !self.allow_ending_underscore && s.as_bytes()[s.len() - 1] == b'_' {
            Err(EndsWithUnderscore)
        } else if !self.allow_double_underscore && s.contains("__") {
            Err(ContainsDoubleUnderscore)
        } else {
            let s: &str = s.trim_start_matches("_");
            let s: &str = s.trim_end_matches("_");
            if s.is_empty() {
                Err(ContainsOnlyUnderscores)
            } else {
                Ok(s)
            }
        }
    }
}

impl IntParser {
    //! `u8`

    /// Parses a `u8` value.
    pub fn parse_u8(&self, s: &str) -> Result<u8, IntParseError> {
        let s: &str = self.validate(s)?;
        u8::from_str(s).map_err(|_| ValueOutOfRange)
    }
}

impl IntParser {
    //! `u16`

    /// Parses a `u16` value.
    pub fn parse_u16(&self, s: &str) -> Result<u16, IntParseError> {
        let s: &str = self.validate(s)?;
        u16::from_str(s).map_err(|_| ValueOutOfRange)
    }
}

impl IntParser {
    //! `u32`

    /// Parses a `u32` value.
    pub fn parse_u32(&self, s: &str) -> Result<u32, IntParseError> {
        let s: &str = self.validate(s)?;
        u32::from_str(s).map_err(|_| ValueOutOfRange)
    }
}

impl IntParser {
    //! `u64`

    /// Parses a `u64` value.
    pub fn parse_u64(&self, s: &str) -> Result<u64, IntParseError> {
        let s: &str = self.validate(s)?;
        u64::from_str(s).map_err(|_| ValueOutOfRange)
    }
}

impl IntParser {
    //! `u128`

    /// Parses a `u128` value.
    pub fn parse_u128(&self, s: &str) -> Result<u128, IntParseError> {
        let s: &str = self.validate(s)?;
        u128::from_str(s).map_err(|_| ValueOutOfRange)
    }
}
