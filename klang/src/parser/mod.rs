mod ast {
    include!(concat!(env!("OUT_DIR"), "/proto/ast.rs"));
}

use ast::*;

pub mod errors;
pub mod expressions;
pub mod functions;
pub mod literals;
pub mod statements;
pub mod structs;

use errors::ParseError;
use functions::parse_function_def;
use pest::Parser;
use std::fs;
use std::path::Path;
use structs::{KlangProgram, PestParser, Rule};

pub fn parse_program(pair: pest::iterators::Pair<Rule>) -> Result<KlangProgram, ParseError> {
    let mut functions = Vec::new();

    for function_pair in pair.into_inner() {
        match function_pair.as_rule() {
            Rule::function_def => match parse_function_def(function_pair) {
                Ok(function) => functions.push(function),
                Err(e) => return Err(e),
            },
            Rule::EOI => break,
            _ => {
                return Err(ParseError::from_pair(
                    format!("Unknown rule: {:?}", function_pair.as_rule()),
                    function_pair,
                ));
            }
        }
    }

    Ok(KlangProgram {
        program: Program { functions },
    })
}

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

pub fn write_program_to_file(program: &KlangProgram, file_path: &Path) -> Result<(), ParseError> {
    let mut buf = Vec::new();
    prost::Message::encode(&program.program, &mut buf).map_err(|e| {
        ParseError::new(format!(
            "Error encoding program to file '{}': {}",
            file_path.display(),
            e
        ))
    })?;

    fs::write(file_path, &buf).map_err(|e| {
        ParseError::new(format!(
            "Error writing program to file '{}': {}",
            file_path.display(),
            e
        ))
    })?;

    Ok(())
}

pub fn read_program_from_file(file_path: &Path) -> Result<KlangProgram, ParseError> {
    let buf = fs::read(file_path).map_err(|e| {
        ParseError::new(format!(
            "Error reading file '{}': {}",
            file_path.display(),
            e
        ))
    })?;
    let program = prost::Message::decode(&*buf)
        .map_err(|e| ParseError::new(format!("Error decoding program: {}", e)))?;
    Ok(KlangProgram { program })
}
