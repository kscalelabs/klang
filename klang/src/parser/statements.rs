use super::ast::*;
use super::expressions::parse_expression;
use super::parser::Rule;
use pest::iterators::Pair;

pub fn parse_block(block: Pair<Rule>) -> Block {
    let mut statements = Vec::new();

    for part in block.into_inner() {
        match part.as_rule() {
            Rule::statement => {
                for stmt_part in part.into_inner() {
                    let rule = stmt_part.as_rule();
                    match rule {
                        Rule::assignment_stmt => {
                            statements.push(Statement {
                                stmt: Some(statement::Stmt::Assignment(parse_assignment_stmt(
                                    stmt_part,
                                ))),
                            });
                        }
                        Rule::expression_stmt => {
                            statements.push(Statement {
                                stmt: Some(statement::Stmt::Expression(ExpressionStmt {
                                    expression: Some(parse_expression(stmt_part)),
                                })),
                            });
                        }
                        Rule::loop_stmt => {
                            statements.push(Statement {
                                stmt: Some(statement::Stmt::Loop(parse_loop_stmt(stmt_part))),
                            });
                        }
                        Rule::break_stmt => {
                            statements.push(Statement {
                                stmt: Some(statement::Stmt::Break(BreakStmt {})),
                            });
                        }
                        Rule::return_stmt => {
                            statements.push(Statement {
                                stmt: Some(statement::Stmt::Return(parse_return_stmt(stmt_part))),
                            });
                        }
                        Rule::empty_stmt => {}
                        _ => {
                            panic!("Unknown statement type: {:?}", rule);
                        }
                    }
                }
            }
            _ => {
                panic!("Unknown block part: {:?}", part);
            }
        }
    }

    Block { statements }
}

pub fn parse_assignment_stmt(assignment_stmt: Pair<Rule>) -> AssignmentStmt {
    let mut identifier = String::new();
    let mut operator = String::new();
    let mut expression = None;

    for part in assignment_stmt.into_inner() {
        match part.as_rule() {
            Rule::identifier => identifier = part.as_str().to_string(),
            Rule::assign_op => operator = part.as_str().to_string(),
            Rule::expression => expression = Some(super::expressions::parse_expression(part)),
            _ => {
                panic!("Unknown assignment statement part: {:?}", part);
            }
        }
    }

    AssignmentStmt {
        identifier,
        operator,
        expression,
    }
}

pub fn parse_loop_stmt(loop_stmt: Pair<Rule>) -> LoopStmt {
    let mut body = None;

    for part in loop_stmt.into_inner() {
        match part.as_rule() {
            Rule::block => body = Some(parse_block(part)),
            _ => {
                panic!("Unknown loop statement part: {:?}", part);
            }
        }
    }

    LoopStmt { body }
}

pub fn parse_return_stmt(return_stmt: Pair<Rule>) -> ReturnStmt {
    let mut expression = None;

    for part in return_stmt.into_inner() {
        match part.as_rule() {
            Rule::expression => expression = Some(super::expressions::parse_expression(part)),
            _ => {
                panic!("Unknown return statement part: {:?}", part);
            }
        }
    }

    ReturnStmt { expression }
}
