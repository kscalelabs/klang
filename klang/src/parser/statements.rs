use super::ast::*;
use super::parser::Rule;
use pest::iterators::Pair;

pub fn parse_block(pair: Pair<Rule>) -> Block {
    let statements = pair
        .into_inner()
        .filter_map(|p| {
            if p.as_rule() == Rule::statement {
                Some(parse_statement(p))
            } else {
                None
            }
        })
        .filter(|s| s.stmt.is_some())
        .collect();

    Block { statements }
}

pub fn parse_statement(pair: Pair<Rule>) -> Statement {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::assignment_stmt => parse_assignment_statement(inner),
        Rule::expression_stmt => parse_expression_statement(inner),
        Rule::loop_stmt => parse_loop_statement(inner),
        Rule::break_stmt => parse_break_statement(inner),
        Rule::return_stmt => parse_return_statement(inner),
        Rule::empty_stmt => parse_empty_statement(inner),
        _ => panic!("Unknown statement type: {:?}", inner.as_rule()),
    }
}

fn parse_assignment_statement(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();
    let identifier = inner.next().unwrap().as_str().to_string();
    let operator = inner.next().unwrap().as_str().to_string();
    let expression = super::expressions::parse_expression(inner.next().unwrap());

    let parsed_operator = match operator.as_str() {
        "=" => None,
        "+=" => Some(BinaryOperator::Add),
        "-=" => Some(BinaryOperator::Sub),
        "*=" => Some(BinaryOperator::Mul),
        "/=" => Some(BinaryOperator::Div),
        _ => panic!("Unknown assignment operator: {:?}", operator),
    };

    match parsed_operator {
        Some(op) => Statement {
            stmt: Some(statement::Stmt::Assignment(AssignmentStmt {
                identifier: identifier.clone(),
                expression: Some(Expression {
                    expr: Some(expression::Expr::Binary(Box::new(BinaryExpr {
                        operator: op.into(),
                        left: Some(Box::new(Expression {
                            expr: Some(expression::Expr::Identifier(Identifier {
                                name: identifier,
                            })),
                        })),
                        right: Some(Box::new(expression)),
                    }))),
                }),
            })),
        },
        None => Statement {
            stmt: Some(statement::Stmt::Assignment(AssignmentStmt {
                identifier: identifier,
                expression: Some(expression),
            })),
        },
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

fn parse_break_statement(_pair: Pair<Rule>) -> Statement {
    Statement {
        stmt: Some(statement::Stmt::Break(BreakStmt {})),
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

fn parse_empty_statement(_pair: Pair<Rule>) -> Statement {
    Statement { stmt: None }
}
