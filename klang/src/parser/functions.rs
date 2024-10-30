use super::ast::*;
use super::parser::Rule;
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

    Parameter {
        name: identifier,
        r#type: Some(Type { name: param_type }),
    }
}

fn parse_doc_string(pair: Pair<Rule>) -> String {
    let inner = pair.into_inner().next().unwrap();
    inner.as_str()[1..inner.as_str().len() - 1].to_string()
}

fn parse_block(pair: Pair<Rule>) -> Block {
    let statements = pair
        .into_inner()
        .filter_map(|p| {
            if p.as_rule() == Rule::statement {
                Some(parse_statement(p))
            } else {
                None
            }
        })
        .collect();

    Block { statements }
}

fn parse_statement(pair: Pair<Rule>) -> Statement {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::assignment_stmt => parse_assignment_statement(inner),
        Rule::expression_stmt => parse_expression_statement(inner),
        Rule::loop_stmt => parse_loop_statement(inner),
        Rule::break_stmt => Statement {
            stmt: Some(statement::Stmt::Break(BreakStmt {})),
        },
        Rule::return_stmt => parse_return_statement(inner),
        Rule::empty_stmt => Statement { stmt: None },
        _ => panic!("Unknown statement type: {:?}", inner.as_rule()),
    }
}

fn parse_assignment_statement(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();
    let identifier = inner.next().unwrap().as_str().to_string();
    let operator = inner.next().unwrap().as_str().to_string();
    let expression = super::expressions::parse_expression(inner.next().unwrap());

    Statement {
        stmt: Some(statement::Stmt::Assignment(AssignmentStmt {
            identifier,
            operator,
            expression: Some(expression),
        })),
    }
}

fn parse_expression_statement(pair: Pair<Rule>) -> Statement {
    let expression = super::expressions::parse_expression(pair.into_inner().next().unwrap());
    Statement {
        stmt: Some(statement::Stmt::Expression(ExpressionStmt {
            expression: Some(expression),
        })),
    }
}

fn parse_loop_statement(pair: Pair<Rule>) -> Statement {
    let block = parse_block(pair.into_inner().next().unwrap());
    Statement {
        stmt: Some(statement::Stmt::Loop(LoopStmt { body: Some(block) })),
    }
}

fn parse_return_statement(pair: Pair<Rule>) -> Statement {
    let expression = super::expressions::parse_expression(pair.into_inner().next().unwrap());
    Statement {
        stmt: Some(statement::Stmt::Return(ReturnStmt {
            expression: Some(expression),
        })),
    }
}
