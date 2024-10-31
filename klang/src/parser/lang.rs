use super::errors::ParseError;
use super::ir::*;
use super::structs::Rule;
use crate::parser::passes::ir_to_ast;
use crate::parser::KlangProgram;
use pest::iterators::Pair;

pub fn parse_program(pair: pest::iterators::Pair<Rule>) -> Result<KlangProgram, ParseError> {
    let mut all_commands = Vec::new();

    for command_pair in pair.into_inner() {
        match command_pair.as_rule() {
            Rule::line => match parse_line(command_pair) {
                Ok(commands) => all_commands.extend(commands),
                Err(e) => return Err(e),
            },
            Rule::EOI => break,
            _ => {
                return Err(ParseError::from_pair(
                    format!("Unknown rule: {:?}", command_pair.as_rule()),
                    command_pair,
                ));
            }
        }
    }

    let ir_program = Program {
        commands: all_commands,
    };
    let ast_program = ir_to_ast(&ir_program)?;

    Ok(KlangProgram {
        program: ast_program,
    })
}

fn parse_line(line: Pair<Rule>) -> Result<Vec<Command>, ParseError> {
    line.into_inner()
        .map(|command_pair| match command_pair.as_rule() {
            Rule::function => Some(parse_function(command_pair)),
            Rule::function_call => Some(parse_function_call(command_pair)),
            Rule::command => Some(parse_command(command_pair)),
            Rule::empty_line => None,
            _ => Some(Err(ParseError::from_pair(
                format!("Unknown rule: {:?}", command_pair.as_rule()),
                command_pair,
            ))),
        })
        .filter_map(|command| command)
        .collect()
}

fn parse_function(pair: Pair<Rule>) -> Result<Command, ParseError> {
    let mut inner = pair.into_inner();
    let text = inner.next().unwrap().as_str().to_string();

    let mut children = Vec::new();
    for child in inner {
        children.extend(parse_line(child)?);
    }

    Ok(Command { text, children })
}

fn parse_function_call(pair: Pair<Rule>) -> Result<Command, ParseError> {
    Ok(Command {
        text: pair.as_str().to_string(),
        children: Vec::new(),
    })
}

fn parse_command(pair: Pair<Rule>) -> Result<Command, ParseError> {
    Ok(Command {
        text: pair.as_str().to_string(),
        children: Vec::new(),
    })
}
