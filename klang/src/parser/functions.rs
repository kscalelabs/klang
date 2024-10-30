use super::ast::*;
use super::parser::Rule;
use super::statements::parse_block;
use pest::iterators::Pair;

pub(crate) fn parse_function_def(function_def: Pair<Rule>) -> FunctionDef {
    let mut name = String::new();
    let mut parameters = Vec::new();
    let mut doc_string = String::new();
    let mut body = None;

    for part in function_def.into_inner() {
        match part.as_rule() {
            Rule::identifier => name = part.as_str().to_string(),
            Rule::parameter_list => parameters = parse_parameters(part),
            Rule::doc_string => doc_string = parse_doc_string(part),
            Rule::block => body = Some(parse_block(part)),
            _ => {}
        }
    }

    FunctionDef {
        name,
        parameters,
        doc_string,
        body,
    }
}

fn parse_parameters(parameters: Pair<Rule>) -> Vec<Parameter> {
    let mut result = Vec::new();

    for part in parameters.into_inner() {
        match part.as_rule() {
            Rule::parameter_value => {
                result.push(parse_parameter_value(part));
            }
            _ => {}
        }
    }

    result
}

fn parse_parameter_value(parameter_value: Pair<Rule>) -> Parameter {
    let mut identifier = String::new();
    let mut param_type = String::new();

    for part in parameter_value.into_inner() {
        match part.as_rule() {
            Rule::identifier => identifier = part.as_str().to_string(),
            Rule::parameter_type => param_type = part.as_str().to_string(),
            _ => {}
        }
    }

    let parsed_type = match param_type.as_str() {
        "string" => ParameterType::String,
        "number" => ParameterType::Number,
        "boolean" => ParameterType::Boolean,
        _ => panic!("Unknown parameter type: {:?}", param_type),
    };

    Parameter {
        name: identifier,
        r#type: parsed_type.into(),
    }
}

fn parse_doc_string(pair: Pair<Rule>) -> String {
    let inner = pair.into_inner().next().unwrap();
    inner.as_str()[1..inner.as_str().len() - 1].to_string()
}
