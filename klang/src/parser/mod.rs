mod ast {
    include!(concat!(env!("OUT_DIR"), "/proto/ast.rs"));
}

use ast::*;

pub mod errors;
pub mod expressions;
pub mod functions;
pub mod literals;
pub mod parser;
pub mod statements;

use errors::ParseError;
use functions::parse_function_def;
use parser::{PestParser, Rule};
use pest::Parser;
use std::fs;
use std::path::Path;

pub fn parse_program(pair: pest::iterators::Pair<Rule>) -> Result<Program, ParseError> {
    let mut functions = Vec::new();

    for function_pair in pair.into_inner() {
        match function_pair.as_rule() {
            Rule::function_def => functions.push(parse_function_def(function_pair)),
            Rule::EOI => break,
            _ => {
                return Err(ParseError {
                    message: format!("Unknown rule: {:?}", function_pair.as_rule()),
                })
            }
        }
    }

    Ok(Program { functions })
}

pub fn parse_string(input: &str) -> Result<Program, ParseError> {
    match PestParser::parse(Rule::program, input) {
        Ok(mut pairs) => parse_program(pairs.next().unwrap()),
        Err(e) => Err(ParseError {
            message: format!("Error parsing input: {}", e),
        }),
    }
}

pub fn parse_file(file_path: &Path) -> Result<Program, ParseError> {
    let unparsed_file = fs::read_to_string(file_path).map_err(|e| ParseError {
        message: format!("Error reading file '{}': {}", file_path.display(), e),
    })?;

    parse_string(&unparsed_file)
}
