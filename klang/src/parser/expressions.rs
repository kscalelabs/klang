use super::ast::*;
use super::literals::parse_literal;
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
        _ => panic!("Unknown expression type: {:?}", pair.as_rule()),
    }
}

fn parse_conditional(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let condition = parse_expression(inner.next().unwrap());

    if let Some(question_mark) = inner.next() {
        assert_eq!(question_mark.as_str(), "?");
        let then_expr = parse_expression(inner.next().unwrap());
        let colon = inner.next().unwrap();
        assert_eq!(colon.as_str(), ":");
        let else_expr = parse_expression(inner.next().unwrap());

        Expression {
            expr: Some(expression::Expr::Conditional(Box::new(ConditionalExpr {
                condition: Some(Box::new(condition)),
                then_expr: Some(Box::new(then_expr)),
                else_expr: Some(Box::new(else_expr)),
            }))),
        }
    } else {
        condition
    }
}

fn parse_logical_or(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let operator = op_pair.as_str().to_string();
        let right_expr = parse_expression(inner.next().unwrap());
        expr = Expression {
            expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right_expr)),
            }))),
        };
    }

    expr
}

fn parse_logical_and(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let operator = op_pair.as_str().to_string();
        let right_expr = parse_expression(inner.next().unwrap());
        expr = Expression {
            expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right_expr)),
            }))),
        };
    }

    expr
}

fn parse_equality(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let operator = op_pair.as_str().to_string();
        let right_expr = parse_expression(inner.next().unwrap());
        expr = Expression {
            expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right_expr)),
            }))),
        };
    }

    expr
}

fn parse_comparison(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let operator = op_pair.as_str().to_string();
        let right_expr = parse_expression(inner.next().unwrap());
        expr = Expression {
            expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right_expr)),
            }))),
        };
    }

    expr
}

fn parse_additive(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let operator = op_pair.as_str().to_string();
        let right_expr = parse_expression(inner.next().unwrap());
        expr = Expression {
            expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right_expr)),
            }))),
        };
    }

    expr
}

fn parse_multiplicative(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let operator = op_pair.as_str().to_string();
        let right_expr = parse_expression(inner.next().unwrap());
        expr = Expression {
            expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right_expr)),
            }))),
        };
    }

    expr
}

fn parse_unary(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut operators = Vec::new();

    while let Some(next_pair) = inner.peek() {
        if next_pair.as_rule() == Rule::unary_operator {
            operators.push(inner.next().unwrap().as_str().to_string());
        } else {
            break;
        }
    }

    let operand = parse_expression(inner.next().unwrap());

    operators
        .into_iter()
        .rev()
        .fold(operand, |expr, op| Expression {
            expr: Some(expression::Expr::Unary(Box::new(UnaryExpr {
                operator: op,
                operand: Some(Box::new(expr)),
            }))),
        })
}

fn parse_postfix(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(next_pair) = inner.next() {
        match next_pair.as_rule() {
            Rule::argument_list => {
                let arguments = parse_argument_list(next_pair);
                expr = Expression {
                    expr: Some(expression::Expr::FunctionCall(Box::new(FunctionCallExpr {
                        function: Some(Box::new(expr)),
                        arguments,
                    }))),
                };
            }
            _ => panic!("Unknown postfix part: {:?}", next_pair),
        }
    }

    expr
}

fn parse_primary(pair: pest::iterators::Pair<Rule>) -> Expression {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::expression => {
            let expr = parse_expression(inner_pair);
            Expression {
                expr: Some(expression::Expr::Grouping(Box::new(GroupingExpr {
                    expression: Some(Box::new(expr)),
                }))),
            }
        }
        Rule::identifier => {
            let name = inner_pair.as_str().to_string();
            Expression {
                expr: Some(expression::Expr::Identifier(Identifier { name })),
            }
        }
        Rule::literal => parse_literal(inner_pair),
        _ => panic!("Unknown primary type: {:?}", inner_pair),
    }
}

fn parse_argument_list(pair: pest::iterators::Pair<Rule>) -> Vec<Argument> {
    pair.into_inner()
        .filter_map(|p| {
            if p.as_rule() == Rule::argument {
                Some(parse_argument(p))
            } else {
                None
            }
        })
        .collect()
}

fn parse_argument(pair: pest::iterators::Pair<Rule>) -> Argument {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let value = parse_expression(inner.next().unwrap());
    Argument {
        name,
        value: Some(value),
    }
}
