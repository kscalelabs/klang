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

            // If the unit is provided, use it to convert to standard units.
            let value = if unit.is_empty() {
                value
            } else {
                convert_to_standard_units(value, unit)
            };

            Expression {
                expr: Some(expression::Expr::Literal(LiteralExpr {
                    value: Some(literal_expr::Value::NumberLiteral(NumberLiteral { value })),
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

fn convert_to_standard_units(value: f64, unit: &str) -> f64 {
    match unit {
        // Convert lengths to meters.
        "mm" => value * 0.001,
        "cm" => value * 0.01,
        "m" => value,
        "km" => value * 1000.0,
        "in" => value * 0.0254,
        "ft" => value * 0.3048,
        "yd" => value * 0.9144,
        "mi" => value * 1609.34,
        // Convert time to seconds.
        "ms" => value * 0.001,
        "s" => value,
        "min" => value * 60.0,
        "hr" => value * 3600.0,
        // Convert angles to degrees.
        "deg" => value,
        "rad" => value * (180.0 / std::f64::consts::PI),
        // Error if the unit is unknown.
        _ => panic!("Unknown unit: {:?}", unit),
    }
}

pub(crate) fn parse_identifier(pair: pest::iterators::Pair<Rule>) -> Expression {
    Expression {
        expr: Some(expression::Expr::Identifier(Identifier {
            name: pair.as_str().to_string(),
        })),
    }
}
