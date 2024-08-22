use pest::error::Error as PestError;
use pest::error::LineColLocation;
use pest::Parser;
use pest::Position;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use thiserror::Error;


#[derive(Parser)]
#[grammar = "klang.pest"]
pub struct Klang;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Program { statements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Import(ImportStmt),
    FunctionDef(FunctionDef),
    Expr(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportStmt {
    pub module: String,
    pub imports: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<(String, Expr)>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    FunctionCall(FunctionCall),
    Assignment(Assignment),
    BinaryOp(Box<BinaryOp>),
    UnaryOp(Box<UnaryOp>),
    Literal(Literal),
    Loop(Loop),
    Comment(String),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub is_async: bool,
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    pub target: String,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOp {
    pub left: Expr,
    pub operator: BinaryOperator,
    pub right: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    TernaryIf,
    TernaryElse,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOp {
    pub operator: UnaryOperator,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    NumberWithUnit(f64, String),
    String(String),
    Boolean(bool),
    List(Vec<Expr>),
    Dict(HashMap<String, Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Loop {
    For(ForLoop),
    While(WhileLoop),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForLoop {
    pub iterator: String,
    pub iterable: Box<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileLoop {
    pub condition: Box<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start: Box<Expr>,
    pub end: Box<Expr>,
    pub step: Option<Box<Expr>>,
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

    pub fn build_ast(pair: Pair<Rule>) -> Result<Program, String> {
        let statements = pair.into_inner()
            .filter(|p| p.as_rule() == Rule::stmt)
            .map(build_stmt)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Program { statements })
    }

    fn build_stmt(pair: Pair<Rule>) -> Result<Stmt, String> {
        match pair.as_rule() {
            Rule::import_stmt => build_import_stmt(pair).map(Stmt::Import),
            Rule::function_def => build_function_def(pair).map(Stmt::FunctionDef),
            Rule::expr_stmt => build_expr(pair.into_inner().next().unwrap()).map(Stmt::Expr),
            _ => Err(format!("Unexpected rule: {:?}", pair.as_rule())),
        }
    }

    fn build_import_stmt(pair: Pair<Rule>) -> Result<ImportStmt, String> {
        let mut inner = pair.into_inner();
        let module = inner.next().unwrap().as_str().to_string();
        let imports = inner.map(|p| p.as_str().to_string()).collect();
        Ok(ImportStmt { module, imports })
    }

    fn build_function_def(pair: Pair<Rule>) -> Result<FunctionDef, String> {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let params = build_param_list(inner.next().unwrap())?;
        let body = inner.next().unwrap().into_inner()
            .map(build_stmt)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FunctionDef { name, params, body })
    }

    fn build_param_list(pair: Pair<Rule>) -> Result<Vec<(String, Expr)>, String> {
        pair.into_inner()
            .map(|p| {
                let mut inner = p.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let expr = build_expr(inner.next().unwrap())?;
                Ok((name, expr))
            })
            .collect()
    }

    fn build_expr(pair: Pair<Rule>) -> Result<Expr, String> {
        match pair.as_rule() {
            Rule::function_call => build_function_call(pair).map(Expr::FunctionCall),
            Rule::assignment => build_assignment(pair).map(Expr::Assignment),
            Rule::binary_op => build_binary_op(pair).map(|op| Expr::BinaryOp(Box::new(op))),
            Rule::unary_op => build_unary_op(pair).map(|op| Expr::UnaryOp(Box::new(op))),
            Rule::literal => build_literal(pair).map(Expr::Literal),
            Rule::loop_expr => build_loop(pair).map(Expr::Loop),
            Rule::COMMENT => Ok(Expr::Comment(pair.as_str().to_string())),
            Rule::IDENT => Ok(Expr::Identifier(pair.as_str().to_string())),
            _ => Err(format!("Unexpected rule in expr: {:?}", pair.as_rule())),
        }
    }

    fn build_function_call(pair: Pair<Rule>) -> Result<FunctionCall, String> {
        let mut inner = pair.into_inner();
        let is_async = inner.next().unwrap().as_rule() == Rule::async_call;
        let call = if is_async { inner.next().unwrap() } else { inner.next().unwrap() };
        let mut call_inner = call.into_inner();
        let name = call_inner.next().unwrap().as_str().to_string();
        let args = call_inner.next().unwrap().into_inner()
            .map(build_expr)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FunctionCall { is_async, name, args })
    }

    fn build_assignment(pair: Pair<Rule>) -> Result<Assignment, String> {
        let mut inner = pair.into_inner();
        let target = inner.next().unwrap().as_str().to_string();
        let value = build_expr(inner.next().unwrap())?;
        Ok(Assignment { target, value: Box::new(value) })
    }

    fn build_binary_op(pair: Pair<Rule>) -> Result<BinaryOp, String> {
        let mut inner = pair.into_inner();
        let left = build_expr(inner.next().unwrap())?;
        let operator = match inner.next().unwrap().as_str() {
            "+" => BinaryOperator::Add,
            "-" => BinaryOperator::Subtract,
            "*" => BinaryOperator::Multiply,
            "/" => BinaryOperator::Divide,
            "?" => BinaryOperator::TernaryIf,
            ":" => BinaryOperator::TernaryElse,
            _ => return Err("Unknown binary operator".to_string()),
        };
        let right = build_expr(inner.next().unwrap())?;
        Ok(BinaryOp { left, operator, right })
    }

    fn build_unary_op(pair: Pair<Rule>) -> Result<UnaryOp, String> {
        let mut inner = pair.into_inner();
        let operator = match inner.next().unwrap().as_str() {
            "-" => UnaryOperator::Negate,
            "!" => UnaryOperator::Not,
            _ => return Err("Unknown unary operator".to_string()),
        };
        let expr = build_expr(inner.next().unwrap())?;
        Ok(UnaryOp { operator, expr })
    }

    fn build_literal(pair: Pair<Rule>) -> Result<Literal, String> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::number_with_unit => {
                let mut parts = inner.into_inner();
                let number = parts.next().unwrap().as_str().parse().unwrap();
                let unit = parts.next().unwrap().as_str().to_string();
                Ok(Literal::NumberWithUnit(number, unit))
            },
            Rule::number => Ok(Literal::Number(inner.as_str().parse().unwrap())),
            Rule::string => Ok(Literal::String(inner.into_inner().next().unwrap().as_str().to_string())),
            Rule::boolean => Ok(Literal::Boolean(inner.as_str() == "true")),
            _ => Err(format!("Unexpected literal type: {:?}", inner.as_rule())),
        }
    }

    fn build_loop(pair: Pair<Rule>) -> Result<Loop, String> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::for_loop => build_for_loop(inner),
            Rule::while_loop => build_while_loop(inner),
            _ => Err(format!("Unexpected loop type: {:?}", inner.as_rule())),
        }
    }

    fn build_for_loop(pair: Pair<Rule>) -> Result<Loop, String> {
        let mut inner = pair.into_inner();
        let iterator = inner.next().unwrap().as_str().to_string();
        let iterable = build_expr(inner.next().unwrap())?;
        let body = inner.next().unwrap().into_inner()
            .map(build_stmt)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Loop::For(ForLoop { iterator, iterable: Box::new(iterable), body }))
    }

    fn build_while_loop(pair: Pair<Rule>) -> Result<Loop, String> {
        let mut inner = pair.into_inner();
        let condition = build_expr(inner.next().unwrap())?;
        let body = inner.next().unwrap().into_inner()
            .map(build_stmt)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Loop::While(WhileLoop { condition: Box::new(condition), body }))
    }
}