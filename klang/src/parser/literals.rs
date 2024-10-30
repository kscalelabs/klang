use super::ast::*;
use super::parser::Rule;

pub(crate) fn parse_literal(pair: pest::iterators::Pair<Rule>) -> Expression {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::string => {
            let value = inner_pair.as_str()[1..inner_pair.as_str().len() - 1].to_string();
            Expression {
                expr: Some(expression::Expr::Literal(LiteralExpr {
                    value: Some(literal_expr::Value::StringLiteral(StringLiteral { value })),
                })),
            }
        }
        Rule::number => {
            let s = inner_pair.as_str();
            let (value_str, unit) = s.trim().split_at(
                s.find(|c: char| !c.is_digit(10) && c != '.' && c != '-')
                    .unwrap_or(s.len()),
            );
            let value: f64 = value_str.parse().unwrap();
            Expression {
                expr: Some(expression::Expr::Literal(LiteralExpr {
                    value: Some(literal_expr::Value::NumberLiteral(NumberLiteral {
                        value,
                        unit: unit.to_string(),
                    })),
                })),
            }
        }
        Rule::boolean => {
            let value = inner_pair.as_str() == "true";
            Expression {
                expr: Some(expression::Expr::Literal(LiteralExpr {
                    value: Some(literal_expr::Value::BooleanLiteral(BooleanLiteral {
                        value,
                    })),
                })),
            }
        }
        _ => panic!("Unknown literal type: {:?}", inner_pair),
    }
}
