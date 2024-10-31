use crate::parser::Rule;
use pest::iterators::Pair;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: String) -> ParseError {
        ParseError {
            message,
            line: 0,
            column: 0,
        }
    }

    pub fn from_pair(message: String, pair: Pair<Rule>) -> ParseError {
        let (line, column) = pair.as_span().start_pos().line_col();
        ParseError {
            message,
            line,
            column,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (line: {}, column: {})",
            self.message, self.line, self.column
        )
    }
}

impl Error for ParseError {}
