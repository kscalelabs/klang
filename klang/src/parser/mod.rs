mod ast {
    include!(concat!(env!("OUT_DIR"), "/proto/ast.rs"));
}

mod ir {
    include!(concat!(env!("OUT_DIR"), "/proto/ir.rs"));
}

pub mod errors;
pub mod lang;
pub mod passes;
pub mod structs;

use errors::ParseError;
use lang::parse_program;
use pest::Parser;
use std::fs;
use std::path::Path;
use structs::{KlangProgram, PestParser, Rule};

pub fn parse_string(input: &str) -> Result<KlangProgram, ParseError> {
    match PestParser::parse(Rule::program, input) {
        Ok(mut pairs) => parse_program(pairs.next().unwrap()),
        Err(e) => Err(ParseError::new(format!("Error parsing input: {}", e))),
    }
}

pub fn parse_file(file_path: &Path) -> Result<KlangProgram, ParseError> {
    let unparsed_file = fs::read_to_string(file_path).map_err(|e| {
        ParseError::new(format!(
            "Error reading file '{}': {}",
            file_path.display(),
            e
        ))
    })?;

    parse_string(&unparsed_file)
}

pub fn write_program_to_file(
    program: &KlangProgram,
    file_path: &Path,
    binary: bool,
) -> Result<(), ParseError> {
    if binary {
        program.save_binary(file_path)
    } else {
        program.save_text(file_path)
    }
}
