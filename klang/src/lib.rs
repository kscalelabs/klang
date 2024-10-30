pub mod parser;

use crate::parser::errors::ParseError;
use crate::parser::{parse_file, write_program_to_file};
use std::path::Path;

pub fn compile_file(file_path: &Path, output_path: &Path) -> Result<(), ParseError> {
    match parse_file(file_path) {
        Ok(program) => write_program_to_file(&program, output_path),
        Err(e) => Err(e),
    }
}

pub fn compile_file_inplace(file_path: &Path) -> Result<(), ParseError> {
    compile_file(file_path, file_path.with_extension("ko").as_path())
}
