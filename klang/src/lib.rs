pub mod parser;

use crate::parser::errors::ParseError;
use crate::parser::parse_file;
use std::path::Path;

pub fn read_and_parse_file(file_path: &Path) -> Result<String, ParseError> {
    match parse_file(file_path) {
        Ok(program) => Ok(format!("{:#?}", program)),
        Err(e) => Err(e),
    }
}
