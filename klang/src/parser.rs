use pest::Parser;
use pest_derive::Parser;
use std::fs;

mod ast {
    include!(concat!(env!("OUT_DIR"), "/proto/ast.rs"));
}

use ast::*;

#[derive(Parser)]
#[grammar = "pest/klang.pest"]
struct PestParser;

fn parse_program(pair: pest::iterators::Pair<Rule>) -> Program {
    let mut functions = Vec::new();

    for function_pair in pair.into_inner() {
        if function_pair.as_rule() == Rule::function_def {
            functions.push(parse_function_def(function_pair));
        }
    }

    Program { functions }
}

fn parse_function_def(pair: pest::iterators::Pair<Rule>) -> FunctionDef {
    let mut name = String::new();
    let mut parameters = Vec::new();
    let mut doc_string = String::new();
    let mut body = None;

    let mut pairs = pair.into_inner();

    pairs.next(); // Skip 'fn'
    if let Some(id_pair) = pairs.next() {
        name = id_pair.as_str().to_string();
    }

    if let Some(param_list_pair) = pairs.next() {
        if param_list_pair.as_rule() == Rule::parameter_list {
            parameters = parse_parameters(param_list_pair);
        }
    }

    if let Some(doc_pair) = pairs.peek() {
        if doc_pair.as_rule() == Rule::doc_string {
            doc_string = parse_doc_string(pairs.next().unwrap());
        }
    }

    if let Some(block_pair) = pairs.next() {
        body = Some(parse_block(block_pair));
    }

    FunctionDef {
        name,
        parameters,
        doc_string,
        body: body.unwrap(),
    }
}

fn parse_parameters(pair: pest::iterators::Pair<Rule>) -> Vec<Parameter> {
    let mut params = Vec::new();

    for param_pair in pair.into_inner() {
        if param_pair.as_rule() == Rule::parameter_value {
            let mut inner_pairs = param_pair.into_inner();
            let name = inner_pairs.next().unwrap().as_str().to_string();
            inner_pairs.next(); // Skip ':'
            let type_name = inner_pairs.next().unwrap().as_str().to_string();
            params.push(Parameter {
                name,
                type_: Some(Type { name: type_name }),
            });
        }
    }

    params
}

fn parse_doc_string(pair: pest::iterators::Pair<Rule>) -> String {
    pair.into_inner().next().unwrap().as_str().to_string()
}

fn parse_block(pair: pest::iterators::Pair<Rule>) -> Block {
    let mut statements = Vec::new();

    for stmt_pair in pair.into_inner() {
        if let Some(stmt) = parse_statement(stmt_pair) {
            statements.push(stmt);
        }
    }

    Block { statements }
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Option<Statement> {
    match pair.as_rule() {
        Rule::assignment_stmt => Some(Statement {
            stmt: Some(statement::Stmt::Assignment(parse_assignment_stmt(pair))),
        }),
        Rule::expression_stmt => Some(Statement {
            stmt: Some(statement::Stmt::ExpressionStmt(ExpressionStmt {
                expression: Some(parse_expression(pair.into_inner().next().unwrap())),
            })),
        }),
        Rule::loop_stmt => Some(Statement {
            stmt: Some(statement::Stmt::Loop(parse_loop_stmt(pair))),
        }),
        Rule::break_stmt => Some(Statement {
            stmt: Some(statement::Stmt::BreakStmt(BreakStmt {})),
        }),
        Rule::return_stmt => Some(Statement {
            stmt: Some(statement::Stmt::ReturnStmt(parse_return_stmt(pair))),
        }),
        Rule::empty_stmt => Some(Statement {
            stmt: Some(statement::Stmt::EmptyStmt(EmptyStmt {})),
        }),
        _ => None,
    }
}

fn parse_assignment_stmt(pair: pest::iterators::Pair<Rule>) -> AssignmentStmt {
    let mut inner = pair.into_inner();

    inner.next(); // Skip 'let'
    let identifier = inner.next().unwrap().as_str().to_string();
    let operator = inner.next().unwrap().as_str().to_string();
    let expression = parse_expression(inner.next().unwrap());

    AssignmentStmt {
        identifier,
        operator,
        expression: Some(expression),
    }
}

fn parse_loop_stmt(pair: pest::iterators::Pair<Rule>) -> LoopStmt {
    let mut inner = pair.into_inner();
    inner.next(); // Skip 'loop'
    let body = parse_block(inner.next().unwrap());

    LoopStmt { body: Some(body) }
}

fn parse_return_stmt(pair: pest::iterators::Pair<Rule>) -> ReturnStmt {
    let mut inner = pair.into_inner();
    inner.next(); // Skip 'return'
    let expression = parse_expression(inner.next().unwrap());

    ReturnStmt {
        expression: Some(expression),
    }
}

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::expression => parse_expression(pair.into_inner().next().unwrap()),
        Rule::conditional => parse_conditional_expr(pair),
        Rule::logical_or => parse_logical_or_expr(pair),
        Rule::logical_and => parse_logical_and_expr(pair),
        Rule::equality => parse_binary_expr(pair, vec!["==", "!="]),
        Rule::comparison => parse_binary_expr(pair, vec!["<", ">", "<=", ">="]),
        Rule::additive => parse_binary_expr(pair, vec!["+", "-"]),
        Rule::multiplicative => parse_binary_expr(pair, vec!["*", "/"]),
        Rule::unary => parse_unary_expr(pair),
        Rule::postfix => parse_postfix_expr(pair),
        Rule::primary => parse_primary_expr(pair),
        _ => panic!("Unknown expression: {:?}", pair),
    }
}

fn parse_conditional_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let condition = parse_expression(inner.next().unwrap());

    if let Some(question_mark) = inner.next() {
        if question_mark.as_str() == "?" {
            let then_expr = parse_expression(inner.next().unwrap());
            inner.next(); // Skip ':'
            let else_expr = parse_expression(inner.next().unwrap());

            Expression {
                expr: Some(expression::Expr::ConditionalExpr(ConditionalExpr {
                    condition: Some(condition),
                    then_expr: Some(then_expr),
                    else_expr: Some(else_expr),
                })),
            }
        } else {
            condition
        }
    } else {
        condition
    }
}

fn parse_logical_or_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let operator = op_pair.as_str().to_string();
        let right_expr = parse_expression(inner.next().unwrap());

        expr = Expression {
            expr: Some(expression::Expr::BinaryExpr(BinaryExpr {
                left: Some(expr),
                operator,
                right: Some(right_expr),
            })),
        };
    }

    expr
}

fn parse_logical_and_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let operator = op_pair.as_str().to_string();
        let right_expr = parse_expression(inner.next().unwrap());

        expr = Expression {
            expr: Some(expression::Expr::BinaryExpr(BinaryExpr {
                left: Some(expr),
                operator,
                right: Some(right_expr),
            })),
        };
    }

    expr
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, operators: Vec<&str>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let operator = op_pair.as_str().to_string();
        if operators.contains(&operator.as_str()) {
            let right_expr = parse_expression(inner.next().unwrap());

            expr = Expression {
                expr: Some(expression::Expr::BinaryExpr(BinaryExpr {
                    left: Some(expr),
                    operator,
                    right: Some(right_expr),
                })),
            };
        } else {
            break;
        }
    }

    expr
}

fn parse_unary_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut operators = Vec::new();

    while let Some(op_pair) = inner.peek() {
        match op_pair.as_rule() {
            Rule::unary_operator => {
                operators.push(inner.next().unwrap().as_str().to_string());
            }
            _ => break,
        }
    }

    let mut expr = parse_expression(inner.next().unwrap());

    for operator in operators.into_iter().rev() {
        expr = Expression {
            expr: Some(expression::Expr::UnaryExpr(UnaryExpr {
                operator,
                operand: Some(expr),
            })),
        };
    }

    expr
}

fn parse_postfix_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();
    let mut expr = parse_expression(inner.next().unwrap());

    while let Some(arg_list_pair) = inner.next() {
        expr = Expression {
            expr: Some(expression::Expr::FunctionCallExpr(FunctionCallExpr {
                function: Some(expr),
                arguments: parse_argument_list(arg_list_pair),
            })),
        };
    }

    expr
}

fn parse_primary_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::expression => parse_expression(inner_pair),
        Rule::identifier => Expression {
            expr: Some(expression::Expr::Identifier(Identifier {
                name: inner_pair.as_str().to_string(),
            })),
        },
        Rule::literal => parse_literal_expr(inner_pair),
        _ => panic!("Unknown primary expression: {:?}", inner_pair),
    }
}

fn parse_literal_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::string => {
            let value = inner_pair.as_str().to_string();
            Expression {
                expr: Some(expression::Expr::LiteralExpr(LiteralExpr {
                    value: Some(literal_expr::Value::StringLiteral(StringLiteral { value })),
                })),
            }
        }
        Rule::number => {
            let value = inner_pair.as_str().parse::<f64>().unwrap();
            Expression {
                expr: Some(expression::Expr::LiteralExpr(LiteralExpr {
                    value: Some(literal_expr::Value::NumberLiteral(NumberLiteral {
                        value,
                        unit: "".to_string(),
                    })),
                })),
            }
        }
        Rule::boolean => {
            let value = inner_pair.as_str() == "true";
            Expression {
                expr: Some(expression::Expr::LiteralExpr(LiteralExpr {
                    value: Some(literal_expr::Value::BooleanLiteral(BooleanLiteral {
                        value,
                    })),
                })),
            }
        }
        _ => panic!("Unknown literal: {:?}", inner_pair),
    }
}

fn parse_argument_list(pair: pest::iterators::Pair<Rule>) -> Vec<Argument> {
    let mut arguments = Vec::new();
    for arg_pair in pair.into_inner() {
        if arg_pair.as_rule() == Rule::argument {
            let mut inner = arg_pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            inner.next(); // Skip ':'
            let value = parse_expression(inner.next().unwrap());
            arguments.push(Argument {
                name,
                value: Some(value),
            });
        }
    }
    arguments
}
