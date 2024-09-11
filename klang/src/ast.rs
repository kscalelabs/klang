use pest::iterators::Pairs;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "klang.pest"]
pub struct KlangParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
    };
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: Rule,
        right: Box<Expr>,
    },
    // Add other expression types as needed
}

impl KlangParser {
    pub fn parse_expression(pairs: Pairs<Rule>) -> Expr {
        PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::number => Expr::Number(primary.as_str().parse().unwrap()),
                Rule::identifier => Expr::Identifier(primary.as_str().to_string()),
                Rule::expression => KlangParser::parse_expression(primary.into_inner()),
                _ => unreachable!(),
            })
            .map_infix(|lhs, op, rhs| {
                let op_rule = op.as_rule();
                Expr::BinaryOp {
                    left: Box::new(lhs),
                    op: op_rule,
                    right: Box::new(rhs),
                }
            })
            .parse(pairs)
    }
}
