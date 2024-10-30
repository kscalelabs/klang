mod ast {
    include!(concat!(env!("OUT_DIR"), "/proto/ast.rs"));
}

use ast::*;

pub mod expressions;
pub mod functions;
pub mod literals;
pub mod parser;
pub mod statements;

use functions::parse_function_def;
use parser::Rule;

pub fn parse_program(pair: pest::iterators::Pair<Rule>) -> Program {
    let mut functions = Vec::new();

    for function_pair in pair.into_inner() {
        match function_pair.as_rule() {
            Rule::function_def => functions.push(parse_function_def(function_pair)),
            Rule::EOI => break,
            _ => panic!("Unknown rule: {:?}", function_pair.as_rule()),
        }
    }

    Program { functions }
}
