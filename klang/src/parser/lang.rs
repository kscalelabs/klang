use super::errors::ParseError;
use super::ir::{
    line::LineKind, text_part::PartKind, Command, Function, FunctionArg, FunctionCall, Line,
    Program, TextPart, TextWithArgs,
};
use super::structs::Rule;
use crate::parser::passes::ir_to_ast;
use crate::parser::KlangProgram;
use pest::iterators::Pair;

pub fn parse_program(pair: pest::iterators::Pair<Rule>) -> Result<KlangProgram, ParseError> {
    let mut all_lines = Vec::new();

    for line_pair in pair.into_inner() {
        match line_pair.as_rule() {
            Rule::line => match parse_line(line_pair) {
                Ok(lines) => all_lines.extend(lines),
                Err(e) => return Err(e),
            },
            Rule::EOI => break,
            _ => {
                return Err(ParseError::from_pair(
                    format!("Unknown rule: {:?}", line_pair.as_rule()),
                    line_pair,
                ));
            }
        }
    }

    let ir_program = Program { lines: all_lines };
    let ast_program = ir_to_ast(&ir_program)?;

    Ok(KlangProgram {
        ast_program,
        ir_program,
    })
}

fn parse_line(line: Pair<Rule>) -> Result<Vec<Line>, ParseError> {
    line.into_inner()
        .map(|line_pair| match line_pair.as_rule() {
            Rule::function_def => match parse_function_def(line_pair) {
                Ok(func) => Some(Ok(Line {
                    line_kind: Some(LineKind::Function(func)),
                })),
                Err(e) => Some(Err(e)),
            },
            Rule::function_call => match parse_function_call(line_pair) {
                Ok(call) => Some(Ok(Line {
                    line_kind: Some(LineKind::FunctionCall(call)),
                })),
                Err(e) => Some(Err(e)),
            },
            Rule::command => match parse_command(line_pair) {
                Ok(cmd) => Some(Ok(Line {
                    line_kind: Some(LineKind::Command(cmd)),
                })),
                Err(e) => Some(Err(e)),
            },
            Rule::empty_line => None,
            _ => Some(Err(ParseError::from_pair(
                format!("Unknown rule: {:?}", line_pair.as_rule()),
                line_pair,
            ))),
        })
        .flatten()
        .collect()
}

fn parse_function_def(pair: Pair<Rule>) -> Result<Function, ParseError> {
    let text_with_args = parse_text_with_args(pair.clone().into_inner().next().unwrap())?;
    let children: Result<Vec<Line>, ParseError> = pair
        .into_inner()
        .skip(1)
        .map(parse_line)
        .try_fold(Vec::new(), |mut acc, result| {
            acc.extend(result?);
            Ok(acc)
        });

    Ok(Function {
        name: Some(text_with_args),
        lines: children?,
    })
}

fn parse_function_call(pair: Pair<Rule>) -> Result<FunctionCall, ParseError> {
    Ok(FunctionCall {
        name: Some(parse_text_with_args(pair.into_inner().next().unwrap())?),
    })
}

fn parse_command(pair: Pair<Rule>) -> Result<Command, ParseError> {
    Ok(Command {
        text: Some(parse_text_with_args(pair.into_inner().next().unwrap())?),
    })
}

fn parse_text_with_args(pair: Pair<Rule>) -> Result<TextWithArgs, ParseError> {
    match pair.as_rule() {
        Rule::text_with_function_args | Rule::text_with_function_params => {
            let parts = pair
                .into_inner()
                .map(parse_text_part)
                .collect::<Result<Vec<TextPart>, ParseError>>()?;
            Ok(TextWithArgs { parts })
        }
        _ => Err(ParseError::from_pair(
            format!("Expected text with args, got {:?}", pair.as_rule()),
            pair,
        )),
    }
}

fn parse_text_part(pair: Pair<Rule>) -> Result<TextPart, ParseError> {
    match pair.as_rule() {
        Rule::text => Ok(TextPart {
            part_kind: Some(PartKind::Text(pair.as_str().to_string())),
        }),
        Rule::function_arg | Rule::function_param => {
            let func_arg = FunctionArg {
                text: pair.into_inner().next().unwrap().as_str().to_string(),
            };
            Ok(TextPart {
                part_kind: Some(PartKind::FunctionArg(func_arg)),
            })
        }
        _ => Err(ParseError::from_pair(
            format!("Expected text or function arg, got {:?}", pair.as_rule()),
            pair,
        )),
    }
}
