use pest::error::Error as PestError;
use pest::error::LineColLocation;
use pest::Parser;
use pest::Position;
use pest_derive::Parser;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "klang.pest"]
pub struct Klang;

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Program { statements }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    ActionDef(ActionDefStmt),
    ActionCall(ActionCallStmt),
    Loop(LoopStmt),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActionDefStmt {
    pub notes: Option<NotesBlock>,
    pub outcomes: Option<OutcomesBlock>,
}

impl ActionDefStmt {
    pub fn new(notes: Option<NotesBlock>, outcomes: Option<OutcomesBlock>) -> Self {
        ActionDefStmt { notes, outcomes }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActionCallStmt {
    pub name: String,
}

impl ActionCallStmt {
    pub fn new(name: String) -> Self {
        ActionCallStmt { name }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoopStmt {
    pub actions: Vec<ActionCallStmt>,
    pub condition: Option<Vec<String>>,
}

impl LoopStmt {
    pub fn new(actions: Vec<ActionCallStmt>, condition: Option<Vec<String>>) -> Self {
        LoopStmt { actions, condition }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotesBlock {
    pub notes: Vec<Note>,
}

impl NotesBlock {
    pub fn new(notes: Vec<Note>) -> Self {
        NotesBlock { notes }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Note {
    Prefer(String),
    Avoid(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct OutcomesBlock {
    pub outcomes: Vec<Outcome>,
}

impl OutcomesBlock {
    pub fn new(outcomes: Vec<Outcome>) -> Self {
        OutcomesBlock { outcomes }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Success(String),
    Failure(String),
    Retry(String),
    Handler(String),
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
        let mut statements = Vec::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::stmt => {
                    let mut inner_rules = inner_pair.into_inner();
                    let stmt = build_stmt(inner_rules.next().unwrap())?;
                    statements.push(stmt);
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }

        Ok(Program::new(statements))
    }

    fn build_stmt(pair: pest::iterators::Pair<Rule>) -> Result<Stmt, String> {
        match pair.as_rule() {
            Rule::action_def_stmt => build_action_def_stmt(pair).map(Stmt::ActionDef),
            Rule::action_call_stmt => build_action_call_stmt(pair).map(Stmt::ActionCall),
            Rule::loop_stmt => build_loop_stmt(pair).map(Stmt::Loop),
            _ => unreachable!(),
        }
    }

    fn build_action_def_stmt(pair: pest::iterators::Pair<Rule>) -> Result<ActionDefStmt, String> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::action_def_body => {
                    let mut inner_rules = inner_pair.into_inner();
                    let notes = inner_rules
                        .clone() // Clone the iterator to use it later
                        .find(|p| p.as_rule() == Rule::notes_block)
                        .map(build_notes_block)
                        .transpose()?;
                    let outcomes = inner_rules
                        .find(|p| p.as_rule() == Rule::outcomes_block)
                        .map(build_outcomes_block)
                        .transpose()?;
                    return Ok(ActionDefStmt::new(notes, outcomes));
                }
                _ => {}
            }
        }

        unreachable!()
    }

    fn build_notes_block(pair: pest::iterators::Pair<Rule>) -> Result<NotesBlock, String> {
        let notes = pair
            .into_inner()
            .filter(|p| p.as_rule() == Rule::note)
            .map(|note_pair| {
                let mut inner_rules = note_pair.into_inner();
                let note_type = inner_rules.next().unwrap().as_rule();
                let note_name = inner_rules.next().unwrap().as_str().to_string();
                match note_type {
                    Rule::PREFER => Note::Prefer(note_name),
                    Rule::AVOID => Note::Avoid(note_name),
                    _ => unreachable!(),
                }
            })
            .collect();

        Ok(NotesBlock::new(notes))
    }

    fn build_outcomes_block(pair: pest::iterators::Pair<Rule>) -> Result<OutcomesBlock, String> {
        let outcomes = pair
            .into_inner()
            .filter(|p| p.as_rule() == Rule::outcome)
            .map(|outcome_pair| {
                let mut inner_rules = outcome_pair.into_inner();
                let outcome_type = inner_rules.next().unwrap().as_rule();
                let outcome_name = inner_rules.next().unwrap().as_str().to_string();
                match outcome_type {
                    Rule::SUCCESS => Outcome::Success(outcome_name),
                    Rule::FAILURE => Outcome::Failure(outcome_name),
                    Rule::RETRY => Outcome::Retry(outcome_name),
                    Rule::HANDLER => Outcome::Handler(outcome_name),
                    _ => unreachable!(),
                }
            })
            .collect();

        Ok(OutcomesBlock::new(outcomes))
    }

    fn build_action_call_stmt(pair: pest::iterators::Pair<Rule>) -> Result<ActionCallStmt, String> {
        let name = pair.into_inner().next().unwrap().as_str().to_string();
        Ok(ActionCallStmt::new(name))
    }

    fn build_loop_stmt(pair: pest::iterators::Pair<Rule>) -> Result<LoopStmt, String> {
        let mut inner_rules = pair.into_inner();

        let actions: Vec<_> = inner_rules
            .clone() // Clone the iterator to use it later
            .filter(|p| p.as_rule() == Rule::action_call_stmt)
            .map(build_action_call_stmt)
            .collect::<Result<Vec<_>, _>>()?;

        let condition = inner_rules
            .find(|p| p.as_rule() == Rule::condition)
            .map(|p| {
                p.into_inner()
                    .map(|outcome_name| outcome_name.as_str().to_string())
                    .collect()
            });

        Ok(LoopStmt::new(actions, condition))
    }
}
