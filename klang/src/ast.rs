use pest::error::Error as PestError;
use pest::error::LineColLocation;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest::Parser;
use pest::Position;
use pest_derive::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[grammar = "klang.pest"]
pub struct Klang;

use lazy_static::lazy_static;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::prefix(unary_minus))
    };
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub import_statement: Option<ImportStatement>,
    pub functions: Vec<FunctionDef>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImportStatement {
    pub module: String,
    pub imports: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDef {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
    Assignment(Assignment),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    IfStatement(IfStatement),
    ParallelExecution(ParallelExecution),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Assignment {
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ForLoop {
    pub iterator: String,
    pub iterable: Expression,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParallelExecution {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    FunctionCall(FunctionCall),
    BinaryOperation(Box<BinaryOperation>),
    UnaryOperation(Box<UnaryOperation>),
    Literal(Literal),
    Identifier(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryOperation {
    pub left: Expression,
    pub operator: String,
    pub right: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnaryOperation {
    pub operator: String,
    pub operand: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Number(String),
    String(String),
    Boolean(bool),
    Array(Vec<Expression>),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("Parsing error: {0}")]
    ParsingError(PestError<Rule>),

    #[error("Semantic error: {0}")]
    SemanticError(String),
}

impl ParseError {
    pub fn from_pest_error(error: PestError<Rule>, input: &str) -> Self {
        // Extract line and column numbers from LineColLocation
        let (line, col) = match error.line_col {
            LineColLocation::Pos((line, col)) => (line, col),
            LineColLocation::Span((start_line, start_col), _) => (start_line, start_col), // Use start of the span
        };

        let line_str = error.line();

        let error_message = format!(
            "Parsing error at line {}, column {}: {}\n{}",
            line,
            col,
            error.variant.message().as_ref(),
            line_str.parse().unwrap_or("".to_string())
        );

        // Obtain the span and create a Position from it
        let span = match error.location {
            pest::error::InputLocation::Pos(pos) => pos,
            pest::error::InputLocation::Span((start, _)) => start, // Use the start of the span
        };

        let position = Position::new(input, span).unwrap(); // Create a Position<'_> from the span and input

        ParseError::ParsingError(PestError::new_from_pos(
            pest::error::ErrorVariant::CustomError {
                message: error_message,
            },
            position,
        ))
    }
}

pub fn parse_file_to_ast<P: AsRef<Path>>(path: P) -> Result<Program, ParseError> {
    // Read the file content
    let file_content = fs::read_to_string(path)?;

    // Parse the file content using the KlangParser
    match Klang::parse(Rule::file_input, &file_content) {
        Ok(mut parsed_file) => {
            let parsed = parsed_file.next().unwrap();
            match builders::build_ast(parsed) {
                Ok(ast) => Ok(ast),
                Err(e) => Err(ParseError::SemanticError(e)),
            }
        }
        Err(error) => Err(ParseError::from_pest_error(error, &file_content)),
    }
}

mod builders {
    use super::*;

    pub fn build_ast(pair: pest::iterators::Pair<Rule>) -> Result<Program, String> {
        let mut import_statement = None;
        let mut functions = Vec::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::import_statement => {
                    import_statement = Some(build_import_statement(inner_pair)?);
                }
                Rule::function_def => {
                    functions.push(build_function_def(inner_pair)?);
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }

        Ok(Program {
            import_statement,
            functions,
        })
    }

    fn build_import_statement(
        pair: pest::iterators::Pair<Rule>,
    ) -> Result<ImportStatement, String> {
        let mut inner_rules = pair.into_inner();
        let module = inner_rules.next().unwrap().as_str().to_string();
        let imports = inner_rules.map(|p| p.as_str().to_string()).collect();

        Ok(ImportStatement { module, imports })
    }

    fn build_function_def(pair: pest::iterators::Pair<Rule>) -> Result<FunctionDef, String> {
        let mut inner_rules = pair.into_inner();
        let name = inner_rules.next().unwrap().as_str().to_string();
        let parameters = build_parameter_list(inner_rules.next().unwrap())?;
        let body = build_block(inner_rules.next().unwrap())?;

        Ok(FunctionDef {
            name,
            parameters,
            body,
        })
    }

    fn build_parameter_list(pair: pest::iterators::Pair<Rule>) -> Result<Vec<String>, String> {
        pair.into_inner()
            .map(|p| Ok(p.as_str().to_string()))
            .collect()
    }

    fn build_block(pair: pest::iterators::Pair<Rule>) -> Result<Block, String> {
        let statements = pair
            .into_inner()
            .map(build_statement)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Block { statements })
    }

    fn build_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, String> {
        match pair.as_rule() {
            Rule::expression_stmt => Ok(Statement::Expression(build_expression(
                pair.into_inner().next().unwrap(),
            )?)),
            Rule::assignment_stmt => {
                let mut inner_rules = pair.into_inner();
                let identifier = inner_rules.next().unwrap().as_str().to_string();
                let expression = build_expression(inner_rules.next().unwrap())?;
                Ok(Statement::Assignment(Assignment {
                    identifier,
                    expression,
                }))
            }
            Rule::for_loop => {
                let mut inner_rules = pair.into_inner();
                let iterator = inner_rules.next().unwrap().as_str().to_string();
                let iterable = build_expression(inner_rules.next().unwrap())?;
                let body = build_block(inner_rules.next().unwrap())?;
                Ok(Statement::ForLoop(ForLoop {
                    iterator,
                    iterable,
                    body,
                }))
            }
            Rule::while_loop => {
                let mut inner_rules = pair.into_inner();
                let condition = build_expression(inner_rules.next().unwrap())?;
                let body = build_block(inner_rules.next().unwrap())?;
                Ok(Statement::WhileLoop(WhileLoop { condition, body }))
            }
            Rule::if_statement => {
                let mut inner_rules = pair.into_inner();
                let condition = build_expression(inner_rules.next().unwrap())?;
                let body = build_block(inner_rules.next().unwrap())?;
                Ok(Statement::IfStatement(IfStatement { condition, body }))
            }
            Rule::parallel_execution => {
                let expressions = pair
                    .into_inner()
                    .map(build_expression)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Statement::ParallelExecution(ParallelExecution {
                    expressions,
                }))
            }
            _ => unreachable!(),
        }
    }

    fn build_expression(pair: pest::iterators::Pair<Rule>) -> Result<Expression, String> {
        match pair.as_rule() {
            Rule::function_call => build_function_call(pair).map(Expression::FunctionCall),
            Rule::binary_operation => {
                build_binary_operation(pair).map(|bo| Expression::BinaryOperation(Box::new(bo)))
            }
            Rule::unary_operation => {
                build_unary_operation(pair).map(|uo| Expression::UnaryOperation(Box::new(uo)))
            }
            Rule::literal => build_literal(pair).map(Expression::Literal),
            Rule::identifier => Ok(Expression::Identifier(pair.as_str().to_string())),
            _ => unreachable!(),
        }
    }

    fn build_function_call(pair: pest::iterators::Pair<Rule>) -> Result<FunctionCall, String> {
        let mut inner_rules = pair.into_inner();
        let name = inner_rules.next().unwrap().as_str().to_string();
        let arguments = inner_rules
            .map(build_expression)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(FunctionCall { name, arguments })
    }

    fn build_binary_operation(
        pair: pest::iterators::Pair<Rule>,
    ) -> Result<BinaryOperation, String> {
        let mut inner_rules = pair.into_inner();
        let left = build_expression(inner_rules.next().unwrap())?;
        let operator = inner_rules.next().unwrap().as_str().to_string();
        let right = build_expression(inner_rules.next().unwrap())?;

        Ok(BinaryOperation {
            left,
            operator,
            right,
        })
    }

    fn build_unary_operation(pair: pest::iterators::Pair<Rule>) -> Result<UnaryOperation, String> {
        let mut inner_rules = pair.into_inner();
        let operator = inner_rules.next().unwrap().as_str().to_string();
        let operand = build_expression(inner_rules.next().unwrap())?;

        Ok(UnaryOperation { operator, operand })
    }

    fn build_literal(pair: pest::iterators::Pair<Rule>) -> Result<Literal, String> {
        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::number => Ok(Literal::Number(inner_pair.as_str().to_string())),
            Rule::string => Ok(Literal::String(inner_pair.as_str().to_string())),
            Rule::boolean => Ok(Literal::Boolean(inner_pair.as_str() == "true")),
            Rule::array => {
                let elements = inner_pair
                    .into_inner()
                    .map(build_expression)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Literal::Array(elements))
            }
            _ => unreachable!(),
        }
    }
}
