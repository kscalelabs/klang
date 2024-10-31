use crate::parser::Rule;
use pest::iterators::Pair;
use prost::{DecodeError, EncodeError};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: String) -> ParseError {
        ParseError { message }
    }

    pub fn from_pair(message: String, pair: Pair<Rule>) -> ParseError {
        let (line, column) = pair.as_span().start_pos().line_col();
        ParseError {
            message: format!("{} (line: {}, column: {})", message, line, column),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError::new(format!("{}", error))
    }
}

impl From<EncodeError> for ParseError {
    fn from(error: EncodeError) -> Self {
        ParseError::new(format!("{}", error))
    }
}

impl From<DecodeError> for ParseError {
    fn from(error: DecodeError) -> Self {
        ParseError::new(format!("{}", error))
    }
}
