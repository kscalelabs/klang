use super::ast::*;
use super::literals::{parse_identifier, parse_literal};
use super::parser::Rule;

pub(crate) fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::expression => parse_expression(pair.into_inner().next().unwrap()),
        Rule::conditional => parse_conditional(pair),
        Rule::logical_or => parse_logical_or(pair),
        Rule::logical_and => parse_logical_and(pair),
        Rule::equality => parse_equality(pair),
        Rule::comparison => parse_comparison(pair),
        Rule::additive => parse_additive(pair),
        Rule::multiplicative => parse_multiplicative(pair),
        Rule::unary => parse_unary(pair),
        Rule::postfix => parse_postfix(pair),
        Rule::primary => parse_primary(pair),
        // _ => panic!("Unknown expression type: {:?}", pair.as_rule()),
        _ => Expression { expr: None },
    }
}

fn parse_conditional(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut parts = pair.into_inner();
    match parts.len() {
        1 => parse_expression(parts.next().unwrap()),
        3 => {
            let condition = parse_expression(parts.next().unwrap());
            let if_true = parse_expression(parts.next().unwrap());
            let if_false = parse_expression(parts.next().unwrap());

            Expression {
                expr: Some(expression::Expr::Conditional(Box::new(ConditionalExpr {
                    condition: Some(Box::new(condition)),
                    then_expr: Some(Box::new(if_true)),
                    else_expr: Some(Box::new(if_false)),
                }))),
            }
        }
        _ => panic!(
            "Unexpected number of parts in conditional: {:?}",
            parts.len()
        ),
    }
}

fn parse_logical_or(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut parts = pair.into_inner();
    match parts.len() {
        1 => parse_expression(parts.next().unwrap()),
        2 => {
            let left = parse_expression(parts.next().unwrap());
            let right = parse_expression(parts.next().unwrap());

            Expression {
                expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                    operator: BinaryOperator::Or.into(),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                }))),
            }
        }
        _ => panic!(
            "Unexpected number of parts in logical or: {:?}",
            parts.len()
        ),
    }
}

fn parse_logical_and(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut parts = pair.into_inner();
    match parts.len() {
        1 => parse_expression(parts.next().unwrap()),
        2 => {
            let left = parse_expression(parts.next().unwrap());
            let right = parse_expression(parts.next().unwrap());

            Expression {
                expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                    operator: BinaryOperator::And.into(),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                }))),
            }
        }
        _ => panic!(
            "Unexpected number of parts in logical and: {:?}",
            parts.len()
        ),
    }
}

fn parse_equality(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut parts = pair.into_inner();
    match parts.len() {
        1 => parse_expression(parts.next().unwrap()),
        3 => {
            let left = parse_expression(parts.next().unwrap());
            let operator = parts.next().unwrap();
            let right = parse_expression(parts.next().unwrap());

            let operator = match operator.as_str() {
                "==" => BinaryOperator::Eq,
                "!=" => BinaryOperator::NotEq,
                _ => panic!("Unknown equality operator: {:?}", operator),
            };

            Expression {
                expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                    operator: operator.into(),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                }))),
            }
        }
        _ => panic!("Unexpected number of parts in equality: {:?}", parts.len()),
    }
}

fn parse_comparison(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut parts = pair.into_inner();
    match parts.len() {
        1 => parse_expression(parts.next().unwrap()),
        3 => {
            let left = parse_expression(parts.next().unwrap());
            let operator = parts.next().unwrap();
            let right = parse_expression(parts.next().unwrap());

            let operator = match operator.as_str() {
                "<" => BinaryOperator::Lt,
                ">" => BinaryOperator::Gt,
                "<=" => BinaryOperator::Lte,
                ">=" => BinaryOperator::Gte,
                _ => panic!("Unknown comparison operator: {:?}", operator),
            };

            Expression {
                expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                    operator: operator.into(),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                }))),
            }
        }
        _ => panic!(
            "Unexpected number of parts in comparison: {:?}",
            parts.len()
        ),
    }
}

fn parse_additive(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut parts = pair.into_inner();
    match parts.len() {
        1 => parse_expression(parts.next().unwrap()),
        3 => {
            let left = parse_expression(parts.next().unwrap());
            let operator = parts.next().unwrap();
            let right = parse_expression(parts.next().unwrap());

            let operator = match operator.as_str() {
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Sub,
                _ => panic!("Unknown additive operator: {:?}", operator),
            };

            Expression {
                expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                    operator: operator.into(),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                }))),
            }
        }
        _ => panic!("Unexpected number of parts in additive: {:?}", parts.len()),
    }
}

fn parse_multiplicative(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut parts = pair.into_inner();
    match parts.len() {
        1 => parse_expression(parts.next().unwrap()),
        3 => {
            let left = parse_expression(parts.next().unwrap());
            let operator = parts.next().unwrap();
            let right = parse_expression(parts.next().unwrap());

            let operator = match operator.as_str() {
                "*" => BinaryOperator::Mul,
                "/" => BinaryOperator::Div,
                _ => panic!("Unknown multiplicative operator: {:?}", operator),
            };

            Expression {
                expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                    operator: operator.into(),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                }))),
            }
        }
        _ => panic!(
            "Unexpected number of parts in multiplicative: {:?}",
            parts.len()
        ),
    }
}

fn parse_unary(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut parts = pair.into_inner();
    match parts.len() {
        1 => parse_expression(parts.next().unwrap()),
        2 => {
            let operator = parts.next().unwrap();
            let operand = parse_expression(parts.next().unwrap());

            let operator = match operator.as_str() {
                "!" => UnaryOperator::Not,
                "-" => UnaryOperator::Neg,
                _ => panic!("Unknown unary operator: {:?}", operator),
            };

            Expression {
                expr: Some(expression::Expr::Unary(Box::new(UnaryExpr {
                    operator: operator.into(),
                    operand: Some(Box::new(operand)),
                }))),
            }
        }
        _ => panic!("Unexpected number of parts in unary: {:?}", parts.len()),
    }
}

fn parse_postfix(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut parts = pair.into_inner();
    match parts.len() {
        1 => parse_expression(parts.next().unwrap()),
        2 => {
            let operand = parse_expression(parts.next().unwrap());
            let argument_list = parse_argument_list(parts.next().unwrap());

            Expression {
                expr: Some(expression::Expr::FunctionCall(Box::new(FunctionCallExpr {
                    function: Some(Box::new(operand)),
                    arguments: argument_list,
                }))),
            }
        }
        _ => panic!("Unexpected number of parts in postfix: {:?}", parts.len()),
    }
}

fn parse_argument_list(pair: pest::iterators::Pair<Rule>) -> Vec<Argument> {
    pair.into_inner()
        .map(|p| {
            let mut parts = p.into_inner();
            let identifier = parts.next().unwrap();
            let expression = parse_expression(parts.next().unwrap());

            Argument {
                name: identifier.as_str().to_string(),
                value: Some(expression),
            }
        })
        .collect()
}

fn parse_primary(pair: pest::iterators::Pair<Rule>) -> Expression {
    let parts = pair.into_inner();
    match parts.len() {
        1 => {
            let part = parts.into_iter().next().unwrap();

            match part.as_rule() {
                Rule::expression => parse_expression(part),
                Rule::literal => parse_literal(part),
                Rule::identifier => parse_identifier(part),
                _ => panic!("Unknown primary type: {:?}", part.as_rule()),
            }
        }
        _ => panic!("Unexpected number of parts in primary: {:?}", parts.len()),
    }
}
